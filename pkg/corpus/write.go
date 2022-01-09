package corpus

import (
	"database/sql"

	"github.com/cespare/xxhash/v2"
	_ "github.com/mattn/go-sqlite3"
)

// func asBytes(xxh3 uint64) []byte {
// 	b := make([]byte, 8)
// 	binary.LittleEndian.PutUint64(b, xxh3)
// 	return b
// }

func DeriveOracleId(name string) uint64 {
	return xxhash.Sum64([]byte(name))
}

func RegisterOracle(db sql.DB, oracleName string) (id uint64, err error) {
	id = DeriveOracleId(oracleName)
	_, err = db.Exec(
		"INSERT INTO oracles (id, name) VALUES (?, ?);",
		id, oracleName)
	return id, err
}

// go:embed ./sql/insert_prediction.sql
var addPrediction string

func InsertPrediction(
	db *sql.DB,
	statementId uint64,
	oracleId uint64,
	languageId int,
	versionId uint64,
	message string,
	errorMessage string,
	valid *bool,
) error {
	_, err := db.Exec(
		addPrediction,
		int64(statementId), int64(oracleId), languageId, versionId,
		message, errorMessage, valid)
	return err
}
