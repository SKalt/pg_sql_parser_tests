package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/cheggaaa/pb/v3"
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
	// Long:  `TODO`,
	Run: func(cmd *cobra.Command, args []string) {
		config := initConfig(cmd)
		for _, version := range config.versions {
			for _, oracleName := range config.oracles {
				switch oracleName {
				case "pg_query":
					err := runPgQueryOracle(
						config.corpusPath,
						version, config.language,
						config.dryRun,
						config.progress,
					)
					if err != nil {
						log.Fatal(err)
					}
				case "do-block":
					err := runDoBlockOracle(
						config.corpusPath,
						version, config.language,
						config.dryRun,
						config.progress,
					)
					if err != nil {
						log.Fatal(err)
					}
				case "psql":
					err := runPsqlOracle(
						config.corpusPath,
						version, config.language,
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
	language string, version string,
	db *sql.DB,
	progress bool,
	dryRun bool,
	// TODO: add side-effect handler func(db *sql.DB, statement *corpus.Statement, prediction *oracles.Prediction) error
) error {
	if dryRun {
		fmt.Printf("would run ")
	} else {
		fmt.Printf("running ")
	}
	languageId := languages.LookupId(language)
	oracleId := corpus.DeriveOracleId(oracle.Name())
	fmt.Printf("oracle `%s` for @language=%s\n", oracle.Name(), language)
	if dryRun {
		return nil
	}
	if err := registerOracle(db, oracleId, oracle.Name()); err != nil {
		return err
	}
	// TODO: consider _not_ loading most of the db into memory.
	// for example, passing an in-channel and an out-channel, then handline each
	// statement one-at-a time
	statements := corpus.GetAllUnpredictedStatements(db, languageId, oracleId)
	if len(statements) == 0 {
		return fmt.Errorf("no unpredicted statements found for language %s", language)
	}
	predict := func(db *sql.DB, statement *corpus.Statement) error {
		prediction, err := oracle.Predict(statement.Text, language)
		if err != nil {
			fmt.Printf("ERR!: %v\n", err)
			return err
		}
		err = corpus.InsertPrediction(
			db,

			statement.Id,
			oracleId,
			languageId,

			prediction.Message,
			prediction.Error,
			prediction.Valid,
		)
		return err
	}

	if progress { // HACK: dry this up
		bar := pb.StartNew(len(statements))
		defer bar.Finish()
		for _, statement := range statements {
			if err := predict(db, statement); err != nil {
				return err
			}
			bar.Increment()
		}
	} else {
		for _, statement := range statements {
			if err := predict(db, statement); err != nil {
				return err
			}
		}
	}
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
	oracle := psql.Init(version)
	// defer oracle.Close()
	return bulkPredict(oracle, language, version, db, progress, dryRun)
}

func runDoBlockOracle(dsn string, version string, language string, dryRun bool, progress bool) error {
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle := doblock.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db, progress, dryRun)
}

func runPgRawOracle(dsn string, version string, language string, dryRun bool, progress bool) error {
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle := raw.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db, progress, dryRun)
}

func runPgQueryOracle(dsn string, version string, language string, dryRun bool, progress bool) error {
	if version != "13" { // silently skip
		return nil
	}
	if dryRun {
		return nil
	}
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle := &pgquery.Oracle{}
	return bulkPredict(oracle, language, "13", db, progress, dryRun)
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
