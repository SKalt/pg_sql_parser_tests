package main

import (
	"database/sql"
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strings"

	_ "github.com/mattn/go-sqlite3"
)

type column struct {
	tableName  string
	columnName string
	dataType   string
	nullable   bool
}

type relationship struct {
	tableFrom  string
	columnFrom string
	tableTo    string
	columnTo   string
}

// TODO: handle composite foreign keys
// TODO: record unique indices, unique constraints, pk constraints for deciding
// relation arity

type table struct {
	columns []column
	fks     []relationship
}

var columnSql = `SELECT
    tbl.name AS table_name
  , col.name AS column_name
  , col.type AS data_type  
  , NOT col."notnull" AS nullable
FROM sqlite_master AS tbl
JOIN pragma_table_info(tbl.name) AS col
WHERE tbl.type = 'table';`

var relationshipSql = `SELECT
  tbl.name as table_from,
  fk_info.[from] as column_from,
  fk_info.[table] as table_to,
  fk_info.[to] as column_to
FROM
  sqlite_master AS tbl
  join pragma_foreign_key_list(tbl.name) as fk_info
ORDER BY tbl.name;`

func inspectSqliteDb(schemaPath string) map[string]*table {

	data, err := ioutil.ReadFile(schemaPath)
	if err != nil {
		panic(err)
	}
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		panic(err)
	}
	defer db.Close()
	for _, statement := range strings.Split(string(data), ";") {
		if _, err := db.Exec(statement); err != nil {
			panic(err)
		}
	}
	tables := make(map[string]*table)
	rows, err := db.Query(columnSql)
	if err != nil {
		panic(err)
	}
	for rows.Next() {
		col := column{}
		rows.Scan(&col.tableName, &col.columnName, &col.dataType, &col.nullable)
		tbl, ok := tables[col.tableName]
		if !ok {
			t := table{}
			tbl = &t
		}
		tbl.columns = append(tbl.columns, col)
		tables[col.tableName] = tbl
	}

	rows, err = db.Query(relationshipSql)
	if err != nil {
		panic(err)
	}
	for rows.Next() {
		rel := relationship{}
		rows.Scan(&rel.tableFrom, &rel.columnFrom, &rel.tableTo, &rel.columnTo)
		tbl := tables[rel.tableFrom]
		tbl.fks = append(tbl.fks, rel)
		// fmt.Println(tbl)
	}
	return tables
}

func main() {
	schemaPath := os.Args[1]
	tables := inspectSqliteDb(schemaPath)
	tableNames := make([]string, len(tables))
	i := 0
	for name := range tables {
		tableNames[i] = name
		i++
	}
	sort.Strings(tableNames)

	result := strings.Builder{}
	result.WriteString("digraph erd {\n")
	// result.WriteString("  center=true\n")
	// result.WriteString("  pack=true\n")
	result.WriteString("  ratio=1\n")
	result.WriteString("  layout=\"neato\"\n")
	result.WriteString("  start=3\n")
	// result.WriteString("  concentrate=true\n")
	result.WriteString("  splines=\"spline\"\n")
	result.WriteString("  overlap=false\n")
	result.WriteString("  fontname=\"monospace\"\n")
	result.WriteString("  node [shape=none]\n")
	// result.WriteString("  nodesep=3.0\n")
	// result.WriteString("  rankdir=\"TB\"\n")
	// result.WriteString("  rankdir=\"LR\"\n")

	for _, name := range tableNames {
		tbl := tables[name]
		result.WriteString(fmt.Sprintf("  %s [\n", name))
		// result.WriteString(fmt.Sprintf("    label=\"{ %s |", name))
		result.WriteString("    label=<\n")
		result.WriteString("      <table align=\"left\" border=\"0\" cellborder=\"1\" cellspacing=\"0\">\n")
		result.WriteString(
			fmt.Sprintf(
				"        <tr><td colspan=\"3\">%s</td></tr>\n",
				name,
			),
		)
		for _, col := range tbl.columns {

			var keyLabel = ""
			for _, fk := range tbl.fks {
				if fk.columnFrom == col.columnName {
					keyLabel = "FK"
					break
				}
			}
			result.WriteString("        <tr>\n")
			result.WriteString(
				fmt.Sprintf(
					"          <td align=\"left\" port=\"%s\">%s</td>\n\n",
					col.columnName,
					col.columnName,
				),
			)
			if keyLabel != "" {
				result.WriteString(
					fmt.Sprintf(
						"          <td align=\"left\" sides=\"TB\">%s</td>",
						col.dataType,
					),
				)
				result.WriteString(
					fmt.Sprintf(
						"          <td align=\"left\" sides=\"TBR\">%s</td>",
						keyLabel,
					),
				)
			} else {
				result.WriteString(
					fmt.Sprintf(
						"          <td align=\"left\" sides=\"TBR\" colspan=\"2\">%s</td>",
						col.dataType,
					),
				)
			}
			result.WriteString("        </tr>\n")
		}
		result.WriteString("      </table>\n")
		result.WriteString("    >\n  ]\n")

		for _, fk := range tbl.fks {
			result.WriteString(
				fmt.Sprintf(
					"  %s:%s -> %s:%s\n",
					fk.tableFrom, fk.columnFrom, fk.tableTo, fk.columnTo,
				),
			)
		}
	}
	result.WriteString("}\n")
	fmt.Println(result.String())
}
