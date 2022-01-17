package oracles

import "github.com/skalt/pg_sql_tests/pkg/corpus"

// an oracle is something that takes some text and predicts whether
// the statement is valid for a given sql-like dialect version.
type Oracle interface {
	GetName() string
	// TODO: maybe Register(db *sql.DB) error
	// derive its own id
	GetId() int64
	Predict(statement *corpus.Statement, languageId int64) (*corpus.Prediction, error)
}
