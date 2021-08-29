Input SQL samples and the tokens and AST that postres sees. Derived from the postgres docs and postgres regression tests.

The directory is structured like so:
```
./fixtures
`- ${test_group}
   `- ${test_name}
      |- input.sql
      |- tokens.json
      `- ast.json
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
