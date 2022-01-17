package driver

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"time"

	"github.com/lib/pq"
	"github.com/skalt/pg_sql_tests/pkg/corpus"
	"github.com/skalt/pg_sql_tests/pkg/languages"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/container"
)

func SyntaxIsOk(err *pq.Error) (validSyntax bool, testimony string) {
	data, e := json.Marshal(err)
	if e != nil {
		panic(e)
	}
	testimony = string(data)
	switch err.Code {
	case "42601": // syntax_error
	case "42P10": // invalid_column_reference
	case "42611": // invalid_column_definition
	case "42P11": // invalid_cursor_definition
	case "42P12": // invalid_database_definition
	case "42P13": // invalid_function_definition
	case "42P14": // invalid_prepared_statement_definition
	case "42P15": // invalid_schema_definition
	case "42P16": // invalid_table_definition
	case "42P17": // invalid_object_definition
		validSyntax = false
	default:
		validSyntax = true
	}
	return validSyntax, testimony
}

func Predict(txn *sql.Tx, statement *corpus.Statement, languageId int64) corpus.Prediction {
	testimony := corpus.Prediction{
		StatementId: statement.Id,
		LanguageId:  languageId,
	}
	_, err := txn.Exec(statement.Text)
	if err != nil {
		if e, ok := err.(*pq.Error); ok {
			switch e.Code {
			case "03000": // sql_statement_not_yet_complete
			case "3D000": // invalid_catalog_name
			case "3F000": // invalid_schema_name
			case "26000": // invalid_sql_statement_name
			// case "2201E": // invalid_argument_for_logarithm",
			// case "22014": // invalid_argument_for_ntile_function",
			// case "22016": // invalid_argument_for_nth_value_function",
			// case "2201F": // invalid_argument_for_power_function",
			// case "2201G": // invalid_argument_for_width_bucket_function",
			case "22019": // invalid_escape_character
			case "2200D": // invalid_escape_octet
			case "22025": // invalid_escape_sequence
			// case "22P02": // invalid_text_representation
			// case "2200M": // invalid_xml_document
			// case "2200N": // invalid_xml_content
			// case "2200S": // invalid_xml_comment
			// case "2200T": // invalid_xml_processing_instruction
			case "22P06": // nonstandard_use_of_escape_character
			case "22010": // invalid_indicator_parameter_value
			case "22023": // invalid_parameter_value
			case "2201B": // invalid_regular_expression
			case "22024": // unterminated_c_string
				// case "42846": // cannot_coerce
				// case "42803": // grouping_error
				fmt.Printf("%s: %s\n----------\n%+v\n", e.Code, statement.Text, e)
				testimony.Valid = nil
				data, err := json.Marshal(e)
				if err != nil {
					panic(err)
				}
				testimony.Error = string(data)
				return testimony
			default:
				valid, etc := SyntaxIsOk(e)
				testimony.Valid = &valid
				testimony.Error = etc
				return testimony
			}
		} else {
			testimony.Valid = nil
			testimony.Error = fmt.Sprintf("%s", err)
			fmt.Printf("%+v\n-----------\n%s", err, statement.Text)
		}
	} else {
		// TODO: use the result here?
		valid := true
		testimony.Valid = &valid
	}
	return testimony
}

type Oracle struct {
	id      *int64
	service *container.Service
	db      *sql.DB
	version string
}

func (oracle *Oracle) GetId() int64 {
	if oracle.id == nil {
		id := corpus.DeriveOracleId(oracle.GetName())
		oracle.id = &id
		return id
	} else {
		return *oracle.id
	}
}

func Init(language string, version string) (*Oracle, error) {
	switch language {
	case "pgsql":
	case "plpgsql":
		break
	default:
		return nil, fmt.Errorf("unsupported language %s", language)
	}
	service := container.InitService(version)
	if err := service.Await(); err != nil {
		log.Fatal(err)
	}
	// service.Await() guarantees that connecting to the service must now work
	db, err := sql.Open("postgres", service.Dsn())
	if err != nil {
		panic(err)
	}
	oracle := Oracle{service: service, db: db, version: version, id: nil}
	return &oracle, nil
}

func (d *Oracle) GetName() string {
	return fmt.Sprintf("postgres %s raw driver", d.version)
}

func (d *Oracle) Predict(statement *corpus.Statement, languageId int64) (*corpus.Prediction, error) {
	var options string

	switch languageId {
	case languages.Languages["pgsql"]:
		options = "SET check_function_bodies = off;"
		// avoid checking plpgsql syntax
	case languages.Languages["plpgsql"]:
		options = "SET check_function_bodies = on;"
	default:
		return nil, fmt.Errorf("unsupported languageId %s", languageId)
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	txn, err := d.db.BeginTx(ctx, &sql.TxOptions{}) // Isolation: sql.LevelSerializable
	if err != nil {
		panic(err)
	}
	_, err = txn.Exec(options)
	if err != nil {
		panic(err)
	}
	testimony := Predict(txn, statement, languageId)
	testimony.OracleId = d.GetId()
	if err := txn.Rollback(); err != nil {
		// pass in case of nested transactions
		fmt.Printf("%s\n>>>>>>>>>>>>>>>>>>>\n%s\n<<<<<<<<<<<<<<<<<<<<<\n", err, statement)
	}
	return &testimony, nil
}

func (oracle *Oracle) Close() {
	if err := oracle.db.Close(); err != nil {
		log.Panic(err)
	}
}
