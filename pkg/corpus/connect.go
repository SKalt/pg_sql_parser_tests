package corpus

import (
	"database/sql"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

func Connect(datasource string) *sql.DB {
	db, err := sql.Open("sqlite3", datasource)
	if err != nil {
		log.Fatal(err)
	}
	return db
}
