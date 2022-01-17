package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"
	"runtime"
	"strings"
	"sync"

	"github.com/cheggaaa/pb"
	"github.com/mattn/go-isatty"
	"github.com/skalt/pg_sql_tests/pkg/corpus"
	"github.com/skalt/pg_sql_tests/pkg/languages"
	"github.com/skalt/pg_sql_tests/pkg/oracles"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/doblock"
	raw "github.com/skalt/pg_sql_tests/pkg/oracles/postgres/driver"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/pgquery"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/psql"
	"github.com/spf13/cobra"
)

var cmd = &cobra.Command{
	Short: "Have a series of oracles opine on whether statements are valid",
	Run: func(cmd *cobra.Command, args []string) {
		config := initConfig(cmd)
		// TODO: validate that oracles can support the given language
		// **before** trying to run the oracles
		for _, version := range config.versions {
			for _, oracleName := range config.oracles {
				switch oracleName {
				case "pg_query":
					err := runPgQueryOracle(
						config.corpusPath,
						version,
						config.language,
						config.dryRun,
						config.progress,
					)
					if err != nil {
						log.Fatal(err)
					}
				case "do-block":
					err := runDoBlockOracle(
						config.corpusPath,
						version,
						config.language,
						config.dryRun,
						config.progress,
					)
					if err != nil {
						log.Fatal(err)
					}
				case "psql":
					err := runPsqlOracle(
						config.corpusPath,
						version,
						config.language,
						config.dryRun,
						config.progress,
					)
					if err != nil {
						log.Fatal(err)
					}
				case "raw":
					err := runPgRawOracle(
						config.corpusPath,
						version, config.language,
						config.dryRun,
						config.progress,
					)
					if err != nil {
						log.Fatal(err)
					}
				}
			}
		}
	},
}

var availableOracles = map[string][]string{
	// TODO: consider namespacing with postgres/<name>?
	"do-block": {"10", "11", "12", "13", "14"},
	"psql":     {"10", "11", "12", "13", "14"},
	"raw":      {"10", "11", "12", "13", "14"},
	"pg_query": {"13"},
}

func listOracles(tty bool) {
	if tty {
		// only print the header if the output isn't piped somewhere
		fmt.Printf("%10s %-20s\n", "oracle", "versions")
	}
	// else {
	// 	for oracle, versions := range availableOracles {
	// 		for _, version := range versions {
	// 			fmt.Printf("%s\t%s\n", oracle, version)
	// 		}
	// 	}
	// }
	for oracle, versions := range availableOracles {
		fmt.Printf("%10s %-20s\n", oracle, strings.Join(versions, ", "))
	}
}

var listOraclesCmd = &cobra.Command{
	Use: "list-oracles",
	Run: func(cmd *cobra.Command, args []string) {
		// cmd.Flags().GetBool("")
		listOracles(isatty.IsTerminal(os.Stdout.Fd()))
	},
}

func registerOracle(db *sql.DB, id int64, name string) error {
	_, err := db.Exec(
		"INSERT INTO oracles(id, name) VALUES (?, ?) ON CONFLICT DO NOTHING",
		id, name,
	)
	return err
}

func bulkPredict(
	oracle oracles.Oracle,
	language string,
	db *sql.DB,
	progress bool,
	dryRun bool,
) error {
	if dryRun {
		fmt.Printf("would run ")
	} else {
		fmt.Printf("running ")
	}
	languageId := languages.LookupId(language)
	oracleId := corpus.DeriveOracleId(oracle.GetName())
	fmt.Printf("oracle `%s` for @language=%s\n", oracle.GetName(), language)
	if dryRun {
		return nil
	}
	if err := registerOracle(db, oracleId, oracle.GetName()); err != nil {
		return err
	}
	// TODO: consider _not_ loading most of the db into memory.
	// for example, passing an in-channel and an out-channel, then handline each
	// statement one-at-a time
	statements := corpus.GetAllUnpredictedStatements(db, languageId, oracleId)
	if len(statements) == 0 {
		return fmt.Errorf("no unpredicted statements found for language %s", language)
	}
	var wg sync.WaitGroup
	nRoutines := runtime.NumCPU()*2 - 1
	if len(statements) < nRoutines {
		nRoutines = len(statements) - 1
	}
	if nRoutines <= 0 {
		nRoutines = 2 // some sort of minimum concurrency
	}
	fmt.Println(nRoutines, "goroutines")
	done := make(chan int, nRoutines)
	outputs := make(chan *corpus.Prediction, len(statements))
	inputs := make(chan *corpus.Statement, nRoutines)

	predict := func(id int, oracle oracles.Oracle, inputs <-chan *corpus.Statement, outputs chan *corpus.Prediction) {
		wg.Add(1)
		defer wg.Done()
		for {
			if statement, ok := <-inputs; ok {
				prediction, err := oracle.Predict(statement, languageId)
				if err != nil {
					panic(err)
				}
				outputs <- prediction
			} else {
				break
			}
		}
		fmt.Println(id, "done")
		done <- id
	}
	save := func(db *sql.DB, outputs <-chan *corpus.Prediction, bar *pb.ProgressBar) {
		wg.Add(1)
		defer wg.Done()
		for {
			fmt.Println("waiting for prediction")
			if prediction, ok := <-outputs; ok {
				if bar != nil {
					bar.Increment()
				}
				if err := corpus.InsertPrediction(db, prediction); err != nil {
					panic(err)
				}
			} else {
				break
			}
		}
	}
	waitForDone := func() {
		countDown := nRoutines
		for {
			fmt.Println("waiting for done")
			if _, ok := <-done; ok {
				countDown -= 1
				if countDown <= 0 {
					fmt.Println("all done")
					break
				}
			} else {
				fmt.Println("somehow done?")
				break
			}
		}
		close(done)
		close(outputs)
		fmt.Printf("done")
	}
	go waitForDone()
	for i := 0; i < nRoutines; i++ {
		go predict(i, oracle, inputs, outputs)
	}
	var bar *pb.ProgressBar = nil
	if progress { // HACK: dry this up
		bar = pb.StartNew(len(statements))
		defer bar.Finish()
	}
	go save(db, outputs, bar)
	go func() {
		for _, statement := range statements {
			inputs <- statement
		}
		close(inputs)
	}()
	wg.Wait()
	return nil
}
func runPsqlOracle(dsn string, version string, language string, dryRun bool, progress bool) error {
	if dryRun {
		fmt.Printf("would run ")
	} else {
		fmt.Printf("running ")
	}
	fmt.Printf("oracle <psql> with language %s @ version %s\n", language, version)
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle, err := psql.Init(language, version)
	if err != nil {
		return err
	}
	// defer oracle.Close()
	return bulkPredict(oracle, language, db, progress, dryRun)
}

func runDoBlockOracle(dsn string, version string, language string, dryRun bool, progress bool) error {
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle, err := doblock.Init(language, version)
	if err != nil {
		return err
	}
	defer oracle.Close()
	return bulkPredict(oracle, language, db, progress, dryRun)
}

func runPgRawOracle(dsn string, version string, language string, dryRun bool, progress bool) error {
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle, err := raw.Init(language, version)
	if err != nil {
		return err
	}
	defer oracle.Close()
	return bulkPredict(oracle, language, db, progress, dryRun)
}

func runPgQueryOracle(dsn string, version string, language string, dryRun bool, progress bool) error {
	if version != "13" { // silently skip
		return nil
	}
	if dryRun {
		return nil
	}
	oracle, err := pgquery.Init(language)
	if err != nil {
		return err
	}
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	return bulkPredict(oracle, language, db, progress, dryRun)
}

type configuration struct {
	corpusPath string
	oracles    []string
	language   string
	versions   []string
	dryRun     bool
	progress   bool
}

func init() {
	cmd.Flags().String("corpus", "./corpus.db", "path to the sqlite corpus database")
	cmd.Flags().StringSlice("oracles", []string{"pg_query"}, "list which oracles to run")
	cmd.Flags().String("language", "pgsql", "which language to try")
	cmd.Flags().StringSlice("versions", []string{"14"}, "which postgres versions to try")
	cmd.Flags().Bool("dry-run", false, "print the configuration rather than running the oracles")
	cmd.PersistentFlags().Bool("progress", isatty.IsTerminal(os.Stdout.Fd()), "render a progress bar")
	cmd.PersistentFlags().Bool("no-progress", false, "don't render a progress bar even when stdout is a tty")
	cmd.AddCommand(listOraclesCmd)
}

func initConfig(cmd *cobra.Command) *configuration {
	fail := false
	dryRun, err := cmd.Flags().GetBool("dry-run")
	if err != nil {
		fmt.Printf("--dry-run: %s\n", err)
		fail = true
	}

	corpus, err := cmd.Flags().GetString("corpus")
	if err != nil {
		fmt.Printf("--corpus: %s\n", err)
	} else {
		if _, err = os.Stat(corpus); err != nil {
			fmt.Printf("--corpus: %s\n", err)
			// primitive does-file-exist
			fail = true
		}
	}

	oracles, err := cmd.Flags().GetStringSlice("oracles")
	if err != nil {
		fmt.Printf("--oracle: %s\n", err)
		fail = true
	} else {
		for _, oracle := range oracles {
			if _, ok := availableOracles[oracle]; !ok {
				fmt.Printf("--oracle: unknown oracle %s\n", oracle)
				fail = true
			}
		}
	}

	versions, err := cmd.Flags().GetStringSlice("versions")
	if err != nil {
		fail = true
		fmt.Printf("--version: %s\n", err)
	} else {
		for _, version := range versions {
			recognized := false
			for _, v := range []string{"10", "11", "12", "13", "14"} {
				if version == v {
					recognized = true
					break
				}
			}
			if !recognized {
				fail = true
				fmt.Printf("--version: unknown postgres version %s\n", version)
			}
		}
	}

	language, err := cmd.Flags().GetString("language")
	if err != nil {
		fail = true
		fmt.Printf("--language: %s", err)
	} else {
		if languages.LookupId(language) == -1 {
			fail = true
			fmt.Printf("--language: unknown language %s\n", language)
		}
	}
	progress, err := cmd.Flags().GetBool("progress")
	if err != nil {
		fail = true
		fmt.Printf("--progress: %v", err)
	}
	noProgress, err := cmd.Flags().GetBool("no-progress")
	if err != nil {
		fail = true
		fmt.Printf("--no-progress: %v", err)
	}
	progress = progress && !noProgress
	if fail {
		os.Exit(1)
	}

	config := configuration{
		dryRun:     dryRun,
		corpusPath: corpus,
		versions:   versions,
		oracles:    oracles,
		language:   language,
		progress:   progress,
	}
	return &config
}

func main() {
	if err := cmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
