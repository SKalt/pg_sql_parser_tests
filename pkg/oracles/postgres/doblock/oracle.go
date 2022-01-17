// ```
// \\\\\\\########|::::::::###########:::::\#\              do-block
// \\\\\\\\#######|:::::::########---.  ::::\#|             oracle
// \\\\\\\\#######|::::::#######/####\\###::|#|
// \\\\\\\\\######|:::::######/######:#\###:|#|
// \\\\\\\\\######|:::::#####/####-----::\####|
// \\\\\\\\\######|:::::####|####/       \####|
// \\\\\\\\\\#####|:.:#####/####/        \\##\|
// \\\\\\\\\\#####|######/:###/          :::::\
// \\\\\\\\\\############\\\#\:{{{{    {{{}\:\##:
// \\\\\\\##########\\:::\##::::{{{{{{{{}:::\#\::\:
// \#############\\::::  ::::::::::{{{{}:::\\::..::::
// \\##########\\\\\::::.::::       ..:::.. .      ..:::\##
// \##########\\\\:\::::::::::: .    :.                ...:::
// \########:##:\:\\:::::::::::::::::     ..::::::::..  ::: :#
// #########\\\::::: #\:::::.          .:::.:.......:.::. .::::
// #######::::#### : :..::::::::::     :.:...       ...::  ...::
// ######::##.\::#::  : . .     . .   ::::.          .::::  :::::.##:\:#\
// ######:#####.:::::: :::: ::::      ::              .:.:   :::###: ::\#
// ######:#######\\\:\###::::::::.  :. ::             : :  ::::\####:::##
// ---------------------------------------------------------------------
//
// ```

package doblock

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"time"

	"github.com/skalt/pg_sql_tests/pkg/corpus"
	"github.com/skalt/pg_sql_tests/pkg/languages"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/container"
	d "github.com/skalt/pg_sql_tests/pkg/oracles/postgres/driver"
)

func testify(conn *sql.Tx, statement *corpus.Statement, languageId int64) corpus.Prediction {
	delim := "SYNTAX_CHECK" // TODO: check string not present in _
	extendedStatement := corpus.Statement{
		Id:   statement.Id,
		Text: fmt.Sprintf("DO $%s$BEGIN RETURN; %s END;$%s$;", delim, statement.Text, delim),
	}
	return d.Predict(conn, &extendedStatement, languageId)
}

type Oracle struct {
	version string
	service *container.Service
	db      *sql.DB
}

func Init(language string, version string) (*Oracle, error) {
	switch language {
	case "pgsql":
	case "plpgsql":
		break
	default:
		return nil, fmt.Errorf("unsupported language %s", language)
	}
	service := container.InitService(version)
	if err := service.Await(); err != nil {
		log.Panic(err)
	}
	// service.Start() guarantees that service.Dsn() will connect on the first
	// try
	conn, err := sql.Open("postgres", service.Dsn())
	if err != nil {
		log.Panic(err)
	}
	oracle := Oracle{version, service, conn}
	return &oracle, nil
}

func (d *Oracle) GetName() string {
	return fmt.Sprintf("postgres %s do-block", d.version)
}

func (oracle *Oracle) GetId() int64 {
	return corpus.DeriveOracleId(oracle.GetName())
}

func (oracle *Oracle) Predict(statement *corpus.Statement, languageId int64) (*corpus.Prediction, error) {
	switch languageId {
	case languages.Languages["pgsql"]:
	case languages.Languages["plpgsql"]:
		break
	default:
		return nil, fmt.Errorf("unsupported language %d", languageId)
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	if _, err := oracle.db.Exec("SET check_function_bodies = ON;"); err != nil {
		// the database is closed?
		panic(err)
	}
	txn, err := oracle.db.BeginTx(ctx, &sql.TxOptions{Isolation: sql.LevelSerializable})
	if err != nil {
		panic(err)
	}

	testimony := testify(txn, statement, languageId)
	if err := txn.Rollback(); err != nil {
		panic(err)
	}
	return &testimony, nil
}

func (d *Oracle) Close() {
	fmt.Println("closing do-block oracle")
	if err := d.db.Close(); err != nil {
		log.Panic(err)
	}
}
