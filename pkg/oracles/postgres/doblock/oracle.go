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

	"github.com/skalt/pg_sql_tests/pkg/oracles"
	"github.com/skalt/pg_sql_tests/pkg/oracles/postgres/container"
	d "github.com/skalt/pg_sql_tests/pkg/oracles/postgres/driver"
)

func testify(conn *sql.Tx, statement string, language string) oracles.Prediction {
	delim := "SYNTAX_CHECK" // TODO: check string not present in _
	return d.Predict(
		conn,
		fmt.Sprintf("DO $%s$BEGIN RETURN; %s END;$%s$;", delim, statement, delim),
		language,
	)
}

type Oracle struct {
	version string
	service *container.Service
	conn    *sql.DB
}

func Init(version string) *Oracle {
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
	return &oracle
}

func (d *Oracle) Name() string {
	return fmt.Sprintf("postgres %s do-block", d.version)
}

func (oracle *Oracle) Predict(statement string, language string) (*oracles.Prediction, error) {
	switch language {
	case "pgsql":
	case "plpgsql":
		break
	default:
		return nil, fmt.Errorf("unsupported language %s", language)
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	if _, err := oracle.conn.Exec("SET check_function_bodies = ON;"); err != nil {
		panic(err)
	}
	txn, err := oracle.conn.BeginTx(ctx, &sql.TxOptions{Isolation: sql.LevelSerializable})
	if err != nil {
		panic(err)
	}

	testimony := testify(txn, statement, language)
	if err := txn.Rollback(); err != nil {
		panic(err)
	}
	return &testimony, nil
}

func (d *Oracle) Close() {
	if err := d.conn.Close(); err != nil {
		log.Panic(err)
	}
}
