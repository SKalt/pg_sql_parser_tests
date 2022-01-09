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
	Id   uint64
	Text string
}

func GetAllStatementsByLanguage(db *sql.DB, language string) (results []*Statement) {
	rows, err := db.Query(getStatementsByLanguage, language)
	if err != nil {
		log.Panic(err)
	}
	defer rows.Close()
	for rows.Next() {
		var row Statement
		if err := rows.Scan(&row.Id, &row.Text); err != nil {
			results = append(results, &row)
		}
	}
	return results
}
