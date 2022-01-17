package corpus

import (
	"database/sql"

	_ "embed"

	"github.com/cespare/xxhash/v2"
	_ "github.com/mattn/go-sqlite3"
)

func DeriveOracleId(name string) int64 {
	return int64(xxhash.Sum64([]byte(name)))
}

func RegisterOracle(db *sql.DB, oracleName string) (id int64, err error) {
	id = DeriveOracleId(oracleName)
	_, err = db.Exec(
		"INSERT INTO oracles (id, name) VALUES (?, ?);",
		id, oracleName)
	return id, err
}

//go:embed sql/insert_prediction.sql
var addPrediction string

func InsertPrediction(
	db *sql.DB,

	statementId int64,
	oracleId int64,
	languageId int,

	message string,
	errorMessage string,
	valid *bool,
) error {
	_, err := db.Exec(
		addPrediction,
		int64(statementId), int64(oracleId), int64(languageId),
		message, errorMessage, valid)
	return err
}
