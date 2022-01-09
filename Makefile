bin/parse: scripts/parse/parse.go
	go build -o bin/parse scripts/parse/parse.go

bin/splitter: scripts/splitter/Cargo.toml ./Cargo.lock scripts/splitter/src/main.rs ./scripts/splitter/src/sqlite.rs ./schema.sql
	cd scripts/splitter && cargo build && cd - && cp ./target/debug/splitter ./bin/

predict_go =  ./scripts/predict/main.go
predict_go += ./pkg/oracles/postgres/container/service.go
predict_go += ./pkg/oracles/postgres/doblock/oracle.go
# predict_go += ./pkg/oracles/postgres/doblock/oracle.go

bin/predict: $(predict_go)
	go build -o bin/predict scripts/predict/main.go