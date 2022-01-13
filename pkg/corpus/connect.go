package corpus

import (
	"database/sql"
	"fmt"

	_ "github.com/mattn/go-sqlite3"
)

var MAJOR int = 0
var MINOR int = 0

func ConnectToExisting(datasource string) (db *sql.DB, err error) {
	db, err = sql.Open("sqlite3", datasource)
	if err != nil {
		return nil, err
	}
	rows, err := db.Query("select major, minor from schema_version order by major desc, minor desc limit 1")
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	for rows.Next() {
		var major int
		var minor int
		rows.Scan(&major, &minor)
		// TODO: accept same major version
		if major != MAJOR || minor != MINOR { // HACK: expects exact version
			return db, fmt.Errorf("expected version 0.0, got %d.%d", major, minor)
		}
	}

	return db, nil
}
