package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"

	"github.com/cespare/xxhash/v2"
	"github.com/cheggaaa/pb/v3"
	"github.com/mattn/go-isatty"
	"github.com/skalt/pg_sql_tests/pkg/corpus"
	"github.com/skalt/pg_sql_tests/pkg/oracles"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/doblock"
	raw "github.com/skalt/pg_sql_tests/pkg/oracles/postgres/driver"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/pgquery"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/psql"
	"github.com/spf13/cobra"
)

var cmd = &cobra.Command{
	Short: "Have a series of oracles opine on whether statements are valid",
	Long:  `TODO`,
	Run: func(cmd *cobra.Command, args []string) {

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
		for oracle, versions := range availableOracles {
			fmt.Printf("%s: %v\n", oracle, versions)
		}
	} else {
		for oracle, versions := range availableOracles {
			for _, version := range versions {
				fmt.Printf("%s\t%s\n", oracle, version)
			}
		}
	}
}

func bulkPredict(
	oracle oracles.Oracle,
	language string, version string,
	db *sql.DB,
) error {
	languageId := corpus.LookupLanguageId(language)
	oracleId := corpus.DeriveOracleId(oracle.Name())
	versionId := xxhash.Sum64(append([]byte("postgres"), []byte(version)...))
	// TODO: consider _not_ loading most of the db into memory.
	statements := corpus.GetStatementsByLanguage(db, language)

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
			versionId,

			prediction.Message,
			prediction.Error,
			prediction.Valid,
		)
		if err != nil {
			return err
		}
		return nil
	}

	if isatty.IsTerminal(os.Stdout.Fd()) {
		bar := pb.StartNew(len(statements))
		for _, statement := range statements {
			bar.Increment()
			if err := predict(statement); err != nil {
				return err
			}
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
	db := corpus.Connect(dsn)
	oracle := psql.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db)
}

func runDoBlockOracle(dsn string, version string, language string) error {
	db := corpus.Connect(dsn)
	oracle := doblock.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db)
}

func runPgRawOracle(dsn string, version string, language string) error {
	db := corpus.Connect(dsn)
	oracle := raw.Init(version)
	defer oracle.Close()
	return bulkPredict(oracle, language, version, db)
}

func runPgQueryOracle(dsn string, language string) error {
	db := corpus.Connect(dsn)
	oracle := &pgquery.Oracle{}
	return bulkPredict(oracle, language, "13", db)
}

// TODO: list available oracles
// TODO: run each oracle

type configuration struct {
	corpusPath string
	oracles    []string
	language   string
	version    string
	dryRun     bool
}

func init() {
	cmd.Flags().Bool("dry-run", false, "TODO")
	cmd.Flags().StringSlice("oracles", []string{"pg_query"}, "list which oracles to run")
	cmd.Flags().String("language", "pgsql", "which language to try")
	cmd.Flags().String("version", "14", "which postgres version to try")
}

func initConfig(cmd *cobra.Command) *configuration {
	errors := []error{}

	dryRun, err := cmd.Flags().GetBool("dry-run")
	if err != nil {
		errors = append(errors, err)
	}

	corpus, err := cmd.Flags().GetString("corpus")
	if err != nil {
		errors = append(errors, err)
	} else {
		if _, err = os.Stat(corpus); err != nil {
			errors = append(errors, err) // primitive does-file-exist
		}
	}

	oracles, err := cmd.Flags().GetStringSlice("oracles")
	if err != nil {
		errors = append(errors, err)
	} else {
		for _, oracle := range oracles {
			if _, ok := availableOracles[oracle]; !ok {
				errors = append(errors, fmt.Errorf("unknown oracle %s", oracle))
			}
		}
	}

	version, err := cmd.Flags().GetString("version")
	if err != nil {
		errors = append(errors, err)
	} else {
		for _, v := range []string{"10", "11", "12", "13", "14"} {
			if version == v {
				break
			}
		}
		errors = append(errors, fmt.Errorf("unknown version %s", version))
	}

	language, err := cmd.Flags().GetString("language")
	if err != nil {
		errors = append(errors, err)
	}

	config := configuration{
		dryRun:     dryRun,
		corpusPath: corpus,
		version:    version,
		oracles:    oracles,
		language:   language,
	}
	return &config
}

func run(config *configuration) {
	if err := runPgQueryOracle(config.corpusPath, config.language); err != nil {
		log.Fatal(err)
	}
}
