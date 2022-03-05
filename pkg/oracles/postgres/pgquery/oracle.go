// ```
//         ____            libpg_query
//     ,dP9CGG88@b,        oracle
//   ,IP  _   Y888@@b,
//  dIi  (_)   G8888@b
// dCII  (_)   G8888@@b
// GCCIi     ,GG8888@@@
// GGCCCCCCCGGG88888@@@
// GGGGCCCGGGG88888@@@@...
// Y8GGGGGG8888888@@@@P.....
//  Y88888888888@@@@@P......
//  `Y8888888@@@@@@@P'......
//     `@@@@@@@@@P'.......
//         """"........
// ```

package pgquery

import (
	"encoding/json"
	"fmt"

	pg_query "github.com/pganalyze/pg_query_go/v2"
	"github.com/skalt/pg_sql_tests/pkg/corpus"
	"github.com/skalt/pg_sql_tests/pkg/languages"
)

const name = "libpg_query 13.X" // only retain postgres version, not libpg_query api version
var id int64 = corpus.DeriveOracleId(name)

type Oracle struct{}

func Init(language string) (oracle *Oracle, err error) {
	switch language {

	case "pgsql":
	case "plpgsql":
		oracle, err = &Oracle{}, nil
	default:
		oracle, err = nil, fmt.Errorf("unsupported language %s", language)
	}
	return oracle, err
}

func (*Oracle) GetName() string {
	return name
}

func (*Oracle) GetId() int64 {
	return id
}

type token struct {
	Name  string
	Start int32
	End   int32
	Text  string
}
type scanResult struct {
	Tokens []*token
	Error  error
}

func (s *scanResult) String() string {
	result, err := json.Marshal(s)
	if err != nil {
		panic(err)
	}
	return string(result)
}

func getTokens(statement string) scanResult {
	result, err := pg_query.Scan(statement)
	if err != nil {
		return scanResult{Error: err}
	}
	tokens := make([]*token, len(result.Tokens))
	for i, protoToken := range result.Tokens {
		tok := token{
			Name:  protoToken.Token.String(),
			Start: protoToken.Start,
			End:   protoToken.End,
			Text:  statement[protoToken.Start:protoToken.End],
		}
		tokens[i] = &tok
	}
	return scanResult{Tokens: tokens, Error: nil}
}

func predictSql(statement *corpus.Statement) *corpus.Prediction {
	testimony := corpus.Prediction{
		OracleId:    id,
		LanguageId:  languages.Languages["pgsql"],
		StatementId: statement.Id,
	}
	result := getTokens(statement.Text)
	if result.Error != nil {
		valid := false
		testimony.Valid = &valid
		testimony.Message = result.String()
		return &testimony
	}
	ast, err := pg_query.ParseToJSON(statement.Text)

	if err != nil {
		result.Error = err
		valid := false
		testimony.Valid = &valid
		testimony.Message = result.String()
		return &testimony
	}
	jsonResult := result.String()
	testimony.Message = jsonResult[0:len(jsonResult)-1] + ", \"ast\": " + ast + "}"
	valid := true
	testimony.Valid = &valid
	return &testimony
}

func predictPlpgsql(statement *corpus.Statement) *corpus.Prediction {
	testimony := corpus.Prediction{
		OracleId:    id,
		LanguageId:  languages.Languages["plpgsql"],
		StatementId: statement.Id,
	}
	result, err := pg_query.ParsePlPgSqlToJSON(statement.Text)

	if err != nil {
		valid := false
		testimony.Valid = &valid
		testimony.Error = fmt.Sprintf("%v", err)
	} else {
		valid := true
		testimony.Valid = &valid
		testimony.Message = fmt.Sprintf("{ast:%s}", result)
	}
	return &testimony
}

func (*Oracle) Predict(statement *corpus.Statement, languageId int64) (*corpus.Prediction, error) {
	switch languageId {
	case languages.Languages["pgsql"]:
		return predictSql(statement), nil
	case languages.Languages["plpgsql"]:
		return predictPlpgsql(statement), nil
	default:
		return nil, fmt.Errorf("unsupported language ID %d", languageId)
	}
}
