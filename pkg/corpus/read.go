package corpus

import (
	"database/sql"
	_ "embed"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

//go:embed sql/get_statements_by_language.sql
var getStatementsByLanguage string

type Statement struct {
	Id   int64
	Text string
}

func GetAllStatementsByLanguage(db *sql.DB, languageId int) []*Statement {
	rows, err := db.Query(getStatementsByLanguage, languageId)
	results := []*Statement{}
	if err != nil {
		log.Panic(err)
	}
	defer rows.Close()
	for rows.Next() {
		var row Statement
		if err := rows.Scan(&row.Id, &row.Text); err != nil {
			panic(err)
		}
		results = append(results, &row)
	}

	return results
}

//go:embed sql/get_unpredicted_statements.sql
var getUnpredictedStatementsQuery string

func GetAllUnpredictedStatements(db *sql.DB, languageId int, oracleId int64) []*Statement {
	rows, err := db.Query(getUnpredictedStatementsQuery, languageId, oracleId, oracleId)
	if err != nil {
		log.Panic(err)
	}
	defer rows.Close()
	results := []*Statement{}
	for rows.Next() {
		var row Statement
		if err := rows.Scan(&row.Id, &row.Text); err != nil {
			panic(err)
		}
		results = append(results, &row)
	}
	return results
}
