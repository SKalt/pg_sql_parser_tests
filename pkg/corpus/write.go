package corpus

import (
	"database/sql"

	_ "embed"

	"github.com/cespare/xxhash/v2"
	_ "github.com/mattn/go-sqlite3"
)

type Prediction struct {
	StatementId int64
	OracleId    int64
	LanguageId  int64
	// may be nil in case of ambiguous oracle output.
	Valid   *bool
	Message string
	Error   string
}

func DeriveOracleId(name string) int64 {
	return int64(xxhash.Sum64([]byte(name)))
}

func RegisterOracleName(db *sql.DB, oracleName string) (id int64, err error) {
	id = DeriveOracleId(oracleName)
	_, err = db.Exec(
		"INSERT INTO oracles (id, name) VALUES (?, ?);",
		id, oracleName)
	return id, err
}

func RegisterOracleId(db *sql.DB, id int64, name string) error {
	_, err := db.Exec(
		"INSERT INTO oracles(id, name) VALUES (?, ?) ON CONFLICT DO NOTHING",
		id, name,
	)
	return err
}

//go:embed sql/insert_prediction.sql
var addPrediction string

func InsertPrediction(
	db *sql.DB,

	prediction *Prediction,
) error {
	_, err := db.Exec(
		addPrediction,
		prediction.StatementId, prediction.OracleId, prediction.LanguageId,
		prediction.Message, prediction.Error, prediction.Valid)
	return err
}

// func BulkInsertPredictions()
