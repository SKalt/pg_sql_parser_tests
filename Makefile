.PHONY: lint all clean
all: ./corpus.db lint bin/parse
clean:
	rm -rf /tmp/pg /tmp/corpus.db ./corpus.db
bin/parse: scripts/parse/parse.go
	go build -o bin/parse scripts/parse/parse.go

bin/splitter: scripts/splitter/Cargo.toml ./Cargo.lock scripts/splitter/src/main.rs ./scripts/splitter/src/sqlite.rs ./schema.sql
	cd scripts/splitter && cargo build && cd - && cp ./target/debug/splitter ./bin/

bin/sqlite_test: scripts/sqlite_test/Cargo.toml scripts/sqlite_test/src/main.rs scripts/sqlite_test/src/test_grammar.pest ./Cargo.lock
	cd scripts/sqlite_test && cargo build && cd - && cp ./target/debug/sqlite_test ./bin/

/tmp/test_results: bin/sqlite_test
	bin/sqlite_test --input ./external/sqlite/test/alter2.test

predict_go =  ./scripts/predict/main.go
predict_go += ./pkg/oracles/postgres/psql/oracle.go
predict_go += ./pkg/oracles/postgres/driver/oracle.go
predict_go += ./pkg/oracles/postgres/doblock/oracle.go
predict_go += ./pkg/oracles/postgres/container/service.go
predict_go += ./pkg/oracles/postgres/pgquery/oracle.go
predict_go += ./pkg/oracles/spec.go
predict_go += ./pkg/corpus/connect.go
predict_go += ./pkg/corpus/read.go
predict_go += ./pkg/corpus/write.go
predict_go += ./pkg/corpus/sql/get_unpredicted_statements.sql
predict_go += ./pkg/corpus/sql/insert_prediction.sql
predict_go += ./pkg/languages/all.go
# TODO: use a build tool where I don't have to specify each dependency manually

bin/predict: $(predict_go)
	go build -o bin/predict scripts/predict/main.go

/tmp/pg/10:
	pg_version=10 ./scripts/postgres_src_dl.sh
/tmp/pg/11:
	pg_version=11 ./scripts/postgres_src_dl.sh
/tmp/pg/12:
	pg_version=12 ./scripts/postgres_src_dl.sh
/tmp/pg/13:
	pg_version=13 ./scripts/postgres_src_dl.sh
/tmp/pg/14:
	pg_version=14 ./scripts/postgres_src_dl.sh

all_regression_tests = /tmp/pg/10 /tmp/pg/11 /tmp/pg/12 /tmp/pg/13 /tmp/pg/14

/tmp/corpus.db: ./scripts/gather_corpus_dir.sh bin/splitter $(all_regression_tests)
	rm -f /tmp/corpus.db
	./scripts/gather_corpus_dir.sh ./bin/splitter /tmp/pg/10 10 /tmp/corpus.db
	./scripts/gather_corpus_dir.sh ./bin/splitter /tmp/pg/11 11 /tmp/corpus.db
	./scripts/gather_corpus_dir.sh ./bin/splitter /tmp/pg/12 12 /tmp/corpus.db
	./scripts/gather_corpus_dir.sh ./bin/splitter /tmp/pg/13 13 /tmp/corpus.db
	./scripts/gather_corpus_dir.sh ./bin/splitter /tmp/pg/14 14 /tmp/corpus.db

# finally!
./corpus.db: ./bin/predict /tmp/corpus.db ./docker-compose.yaml
	cp /tmp/corpus.db ./
	docker-compose up -d pg-10 pg-11 pg-12 pg-13 pg-14
	bin/predict --oracles raw,do-block,pg_query --versions 10,11,12,13,14

bin/erd: ./scripts/erd/main.go
	go build -o bin/erd scripts/erd/main.go
./erd.svg: bin/erd ./schema.sql
	bin/erd ./schema.sql > /tmp/erd.dot && dot -Tsvg -o ./erd.svg </tmp/erd.dot

lint:
	golangci-lint run
	cargo check
