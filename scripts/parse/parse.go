package main

import (
	"encoding/json"
	"os"
	"path/filepath"

	pg_query "github.com/pganalyze/pg_query_go/v2"
)

func panicIfNotOk(err error) {
	if err != nil {
		panic(err)
	}
}

// path: e.g. ./fixtures/my_test_suite/my_specific_test
// note no trailing slash
func getInput(path string) string {
	data, err := os.ReadFile(filepath.Join(path, "input.sql"))
	panicIfNotOk(err)
	return string(data)
}

const rw = 0666 // read-write only file mode: -rw-rw-rw-. Also, 🤘

// path: e.g. ./fixtures/my_test_suite/my_specific_test/ast.json
func prettyPrintTo(path string, inputJson interface{}) {
	outputJson, err := json.MarshalIndent(inputJson, "", "  ")
	panicIfNotOk(err)
	err = os.WriteFile(path, outputJson, rw)
	panicIfNotOk(err)
}

type token struct {
	Name  string
	Start int32
	End   int32
	Text  string
}

// path: e.g. ./fixtures/my_test_suite/my_specific_test
func tokens(path string, inputData string) {
	result, err := pg_query.Scan(inputData)
	panicIfNotOk(err)
	tokens := make([]*token, len(result.Tokens))
	for i, protoToken := range result.Tokens {
		tok := token{
			Name:  protoToken.Token.String(),
			Start: protoToken.Start,
			End:   protoToken.End,
			Text:  inputData[protoToken.Start:protoToken.End],
		}
		tokens[i] = &tok
	}
	prettyPrintTo(filepath.Join(path, "tokens.json"), tokens)
}

// path: e.g. ./fixtures/my_test_suite/my_specific_test
func ast(path string, inputData string) {
	jsonData, err := pg_query.ParseToJSON(inputData)
	panicIfNotOk(err)
	intermediateJson := map[string]interface{}{}
	err = json.Unmarshal([]byte(jsonData), &intermediateJson)
	panicIfNotOk(err)
	prettyPrintTo(filepath.Join(path, "ast.json"), intermediateJson)
}

func main() {
	path := os.Args[1]
	inputSql := getInput(path)
	tokens(path, inputSql)
	ast(path, inputSql)
}
