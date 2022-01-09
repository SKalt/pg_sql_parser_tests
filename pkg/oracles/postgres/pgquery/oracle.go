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
	"github.com/skalt/pg_sql_tests/pkg/oracles"
)

const name = "libpg_query 13.X" // only retain postgres version, not libpg_query api version
const version = "13"

type Oracle struct{}

func (*Oracle) Name() string {
	return name
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

func predictSql(statement string) *oracles.Prediction {
	testimony := oracles.Prediction{Language: "pgsql", Version: version}
	result := getTokens(statement)
	if result.Error != nil {
		valid := false
		testimony.Valid = &valid
		testimony.Message = result.String()
		return &testimony
	}
	ast, err := pg_query.ParseToJSON(statement)

	if err != nil {
		result.Error = err
		valid := false
		testimony.Valid = &valid
		testimony.Message = result.String()
		return &testimony
	}
	jsonResult := result.String()
	testimony.Message = jsonResult[0:len(jsonResult)-2] + ", ast: " + ast + "}"
	valid := true
	testimony.Valid = &valid
	return &testimony
}

func predictPlpgsql(statement string) *oracles.Prediction {
	testimony := oracles.Prediction{Language: "plpgsql", Version: version}
	result, err := pg_query.ParsePlPgSqlToJSON(statement)

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

func (*Oracle) Predict(statement string, language string) (*oracles.Prediction, error) {
	switch language {
	case "pgsql":
		return predictSql(statement), nil
	case "plpgsql":
		return predictPlpgsql(statement), nil
	default:
		return nil, fmt.Errorf("unsupported language %s", language)
	}
}
