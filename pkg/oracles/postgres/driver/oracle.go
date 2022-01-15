package driver

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"

	"github.com/lib/pq"
	"github.com/skalt/pg_sql_tests/pkg/oracles"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/container"
)

func SyntaxIsOk(err pq.Error) (validSyntax bool, testimony string) {
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

// Invalid SQL Statement Name
func Predict(conn *sql.DB, statement string, language string) oracles.Prediction {
	testimony := oracles.Prediction{Language: language}
	_, err := conn.Exec(statement)
	if err != nil {
		if e, ok := err.(pq.Error); ok {
			switch e.Code {
			// panic on codes that might/not be syntax errors
			case "03000": // sql_statement_not_yet_complete
			case "3D000": // invalid_catalog_name
			case "3F000": // invalid_schema_name
			case "26000": // invalid_sql_statement_name
			case "2201E": // invalid_argument_for_logarithm",
			case "22014": // invalid_argument_for_ntile_function",
			case "22016": // invalid_argument_for_nth_value_function",
			case "2201F": // invalid_argument_for_power_function",
			case "2201G": // invalid_argument_for_width_bucket_function",
			case "22019": // invalid_escape_character
			case "2200D": // invalid_escape_octet
			case "22025": // invalid_escape_sequence
			case "22P06": // nonstandard_use_of_escape_character
			case "22010": // invalid_indicator_parameter_value
			case "22023": // invalid_parameter_value
			case "2201B": // invalid_regular_expression
			case "22024": // unterminated_c_string
			case "42846": // cannot_coerce
			case "42803": // grouping_error
			case "42P20": // windowing_error
				// malformed xml documents are just strings
				// case "2200M": // invalid_xml_document
				// case "2200N": // invalid_xml_content
				// case "2200S": // invalid_xml_comment
				// case "2200T": // invalid_xml_processing_instruction
				log.Panicf("%+v\n----------\n%s", statement, e)
			}
			valid, etc := SyntaxIsOk(e)
			testimony.Valid = &valid
			testimony.Message = etc
		} else {
			panic(err)
		}
	} else {
		// TODO: use the result here
		valid := true
		testimony.Valid = &valid
	}
	return testimony
}

type Oracle struct {
	service *container.Service
	db      *sql.DB
	version string
}

func Init(version string) *Oracle {
	service := container.InitService(version)
	if err := service.Await(); err != nil {
		log.Fatal(err)
	}
	conn, err := sql.Open("postgres", service.Dsn())
	// service.Start() guarantees that connecting to the service will
	// work on the first try
	if err != nil {
		panic(err)
	}
	oracle := Oracle{service, conn, version}
	return &oracle
}

func (d *Oracle) Name() string {
	return fmt.Sprintf("postgres %s raw driver", d.version)
}

func (d *Oracle) Predict(statement string, language string) (*oracles.Prediction, error) {
	switch language {
	case "pgsql":
		_, err := d.db.Exec("SET check_function_bodies = off;")
		if err != nil {
			return nil, err
		}

		// avoid checking plpgsql syntax
	case "plpgsql":
		_, err := d.db.Exec("SET check_function_bodies = on;")
		if err != nil {
			return nil, err
		}
	default:
		return nil, fmt.Errorf("unsupported language %s", language)
	}
	testimony := Predict(d.db, statement, language)
	return &testimony, nil
}

func (oracle *Oracle) Close() {
	if err := oracle.db.Close(); err != nil {
		log.Panic(err)
	}
}
