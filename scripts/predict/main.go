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
		for _, oracleName := range config.oracles {
			for _, version := range config.versions {
				if config.dryRun {
					// TODO: have each runOracle fn take a dryRun arg and print this instead
					fmt.Printf("would run oracle: %s @ %s\n", oracleName, version)
				} else {
					fmt.Printf("running oracle: %s\n", oracleName)
					switch oracleName {
					case "pg_query":
						err := runPgQueryOracle(config.corpusPath, version, config.language)
						if err != nil {
							log.Fatal(err)
						}
					case "do-block":
						err := runDoBlockOracle(config.corpusPath, version, config.language)
						if err != nil {
							log.Fatal(err)
						}
					case "psql":
						err := runPsqlOracle(config.corpusPath, version, config.language)
						if err != nil {
							log.Fatal(err)
						}
					case "raw":
						err := runPgRawOracle(config.corpusPath, version, config.language)
						if err != nil {
							log.Fatal(err)
						}
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

func listOracles() {
	if isatty.IsTerminal(os.Stdout.Fd()) {
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
		listOracles()
	},
}

func bulkPredict(
	oracle oracles.Oracle,
	language string, version string,
	db *sql.DB,
	// TODO: add side-effect handler func(db *sql.DB, statement *corpus.Statement, prediction *oracles.Prediction) error
) error {
	languageId := languages.LookupId(language)
	oracleId := corpus.DeriveOracleId(oracle.Name())
	// versionId := xxhash.Sum64(append([]byte("postgres"), []byte(version)...))
	// TODO: consider _not_ loading most of the db into memory.
	// for example, passing an in-channel and an out-channel, then handline each
	// statement one-at-a time
	statements := corpus.GetAllStatementsByLanguage(db, language)
	if len(statements) == 0 {
		return fmt.Errorf("no statements found for language %s", language)
	}
	predict := func(statement *corpus.Statement) error {
		prediction, err := oracle.Predict(statement.Text, language)
		if err != nil {
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
		if err != nil {
			return err
		}
		// handle side-effect here
		return nil
	}

	if isatty.IsTerminal(os.Stdout.Fd()) {
		bar := pb.StartNew(len(statements))
		for _, statement := range statements {
			if err := predict(statement); err != nil {
				return err
			}
			bar.Increment()
		}
		bar.Finish()
	} else {
		for _, statement := range statements {
			if err := predict(statement); err != nil {
				return err
			}
		}
	}
	return nil
}

func runPsqlOracle(dsn string, version string, language string) error {
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle := psql.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db)
}

func runDoBlockOracle(dsn string, version string, language string) error {
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return nil
	}
	oracle := doblock.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db)
}

func runPgRawOracle(dsn string, version string, language string) error {
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle := raw.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db)
}

func runPgQueryOracle(dsn string, version string, language string) error {
	if version != "13" { // silently skip
		return nil
	}
	db, err := corpus.ConnectToExisting(dsn)
	if err != nil {
		return err
	}
	oracle := &pgquery.Oracle{}
	return bulkPredict(oracle, language, "13", db)
}

type configuration struct {
	corpusPath string
	oracles    []string
	language   string
	versions   []string
	dryRun     bool
}

func init() {
	cmd.Flags().String("corpus", "./corpus.db", "path to the sqlite corpus database")
	cmd.Flags().StringSlice("oracles", []string{"pg_query"}, "list which oracles to run")
	cmd.Flags().String("language", "pgsql", "which language to try")
	cmd.Flags().StringSlice("versions", []string{"14"}, "which postgres versions to try")
	cmd.Flags().Bool("dry-run", false, "print the configuration rather than running the oracles")
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
	if fail {
		os.Exit(1)
	}
	config := configuration{
		dryRun:     dryRun,
		corpusPath: corpus,
		versions:   versions,
		oracles:    oracles,
		language:   language,
	}
	return &config
}

func main() {
	if err := cmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
