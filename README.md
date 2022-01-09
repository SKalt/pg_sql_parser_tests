# SQL Parser Tests

These tools collect a corpus of SQL-like statements in a variety of languages, then use a series of oracles to predict whether each statement is valid in a given language.

Currently postgres-specific.

## Oracle testing

```
 (=======)
  :||||:
  |||||||
  :|:||||
  |||||:
   ||||||
  |:|||||
  |||||||
   |||: |
 (= = = =)
```

In this repo, "oracle" refers to a black box which issues truthful predictions about a program's behavior, _not_ Oracle the lawsuit-issuing corporation.

Oracle-testing is a black-box testing method which gives limited insight into whether any given statement is valid. Using oracles to predict syntax validity is easier than deriving parsing tests from the numerous specifications of SQL dialects.

## Usage: Consuming a test suite

Download one of the databases from the [GitHub realeases tab](#TODO), then query the statements and oracle output using sqlite. You can [find the sqlite database schema](./schema.sql) in the root of this repo.

### Contributing

If you'd like to contribute new oracles, sources of SQL, documentation, or fixes, see [./CONTRIBUTING.md](./CONTRIBUTING.md).

# License

BSD 3-clause; see [./LICENSE](./LICENSE)

# Directory structure

```
.
├── pkg/
|   ├── corpus/ # tools for interacting with the test-corpus database
|   └── oracles/                       # defines the oracle interface
|       └── ${database}/${oracle}/*.go # individual oracles
├── scripts
|   ├── parse/    # output pg_query AST for sanity-checking oracle results
|   ├── splitter/ # create a sql-statement-corpus database
|   └── predict/  # runs oracles over an existing corpus database
├── docker-compose.yaml # for defining database-services for use by oracles
├── Earthfile,Makefile # build tool
├── go.mod,go.sum,Cargo.toml,Cargo.lock # package management
└── README.md
```
