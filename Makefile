bin/parse: scripts/parse/parse.go
	go build -o bin/parse scripts/parse/parse.go

bin/splitter: scripts/splitter/Cargo.toml scripts/splitter/src/main.rs
	cd scripts/splitter && cargo build && cd - && cp ./target/debug/splitter ./bin/