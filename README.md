Input SQL samples and the tokens and AST that postres sees.
Derived from the postgres docs and postgres regression tests.

## Directory structure

```
fixtures
├── data
|   └── ${xxhash}.sql
├── versions
|   └── ${version} # 0-padded for asciibetical sorting
|        ├── doctests/${test_group}/${test_name}/
|        |   └── input.sql -> ../../../data/${xxhash}/input.sql
|        └── regress/${test_group}/${test_name}/
|            └── input.sql -> ../../../data/${xxhash}/input.sql
└── suites
    ├── supported.yaml
    └── *.yaml # other common queries
```

tokens and ast are generated using pganalyze/libpg_query via https://github.com/pganalyze/pg_query_go.

`tokens.json` are structured as an array of:

```go
type token struct {
	Name  string
	Start int32
	End   int32
	Text  string
}
```

ast.json is a giant pretty-printed JSON tree.

## Cosuming a test suite

??? maybe spit out a tar.gz

## Maintaining the directory structure

`make dircheck`

<!-- `make linkcheck` -->
