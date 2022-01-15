VERSION 0.6

FROM alpine:3
RUN mkdir -p /workspace
WORKDIR /workspace
RUN apk add curl sqlite

rust:
  FROM rust:buster
  RUN mkdir /workspace
  WORKDIR /workspace
  RUN cargo install cargo-chef

  COPY ./Cargo.lock ./Cargo.toml .
  COPY ./scripts/splitter/Cargo.toml ./scripts/splitter/

splitter-recipe:
  FROM +rust
  RUN cd ./scripts/splitter && cargo chef prepare --recipe-path recipe.json
  SAVE ARTIFACT /workspace/scripts/splitter/recipe.json

splitter:
  FROM +rust
  COPY +splitter-recipe/recipe.json ./scripts/splitter/recipe.json
  RUN cd ./scripts/splitter && cargo chef cook --release --recipe-path recipe.json
  COPY ./scripts/splitter/ ./scripts/splitter/
  COPY ./schema.sql ./ # required to re-overwrite the changes made by cargo chef
  RUN cd scripts/splitter && cargo build --release && cd -
  SAVE ARTIFACT /workspace/target/release/splitter
  SAVE IMAGE --cache-hint

pg-files-14:
  ENV pg_version=14
  COPY ./scripts/postgres_src_dl.sh ./scripts/
  RUN ./scripts/postgres_src_dl.sh
  SAVE ARTIFACT /tmp/pg/14 /pg


pg-files-13:
  ENV pg_version=13
  COPY ./scripts/postgres_src_dl.sh ./scripts/
  RUN ./scripts/postgres_src_dl.sh
  SAVE ARTIFACT /tmp/pg/13 /pg

pg-files-12:
  ENV pg_version=12
  COPY ./scripts/postgres_src_dl.sh ./scripts/
  RUN ./scripts/postgres_src_dl.sh
  SAVE ARTIFACT /tmp/pg/12 /pg

pg-files-11:
  ENV pg_version=11
  COPY ./scripts/postgres_src_dl.sh ./scripts/
  RUN ./scripts/postgres_src_dl.sh
  SAVE ARTIFACT /tmp/pg/11 /pg

pg-files-10:
  ENV pg_version=10
  COPY ./scripts/postgres_src_dl.sh ./scripts/
  RUN ./scripts/postgres_src_dl.sh
  SAVE ARTIFACT /tmp/pg/10 /pg

pg-corpus-14:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=14
  COPY +pg-files-14/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  COPY ./scripts/gather_corpus_dir.sh ./scripts/
  RUN  ./scripts/gather_corpus_dir.sh /bin/splitter /pg/ $pg_version /db
  SAVE ARTIFACT /db

pg-corpus-13:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=13
  COPY +pg-files-13/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  COPY ./scripts/gather_corpus_dir.sh ./scripts/
  RUN  ./scripts/gather_corpus_dir.sh /bin/splitter /pg/ $pg_version /db
  SAVE ARTIFACT /db

pg-corpus-12:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=12
  COPY +pg-files-12/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  COPY ./scripts/gather_corpus_dir.sh ./scripts/
  RUN  ./scripts/gather_corpus_dir.sh /bin/splitter /pg/ $pg_version /db
  SAVE ARTIFACT /db

pg-corpus-11:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=11
  COPY +pg-files-11/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  COPY ./scripts/gather_corpus_dir.sh ./scripts/
  RUN  ./scripts/gather_corpus_dir.sh /bin/splitter /pg/ $pg_version /db
  SAVE ARTIFACT /db

pg-corpus-10:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=10
  COPY +pg-files-10/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  COPY ./scripts/gather_corpus_dir.sh ./scripts/
  RUN  ./scripts/gather_corpus_dir.sh /bin/splitter /pg/ $pg_version /db
  SAVE ARTIFACT /db

pg-corpus-all:
  COPY ./schema.sql ./
  COPY ./scripts/merge.sh ./scripts/
  COPY +pg-corpus-10/db ./10.db
  COPY +pg-corpus-11/db ./11.db
  COPY +pg-corpus-12/db ./12.db
  COPY +pg-corpus-13/db ./13.db
  COPY +pg-corpus-14/db ./14.db
  RUN ./scripts/merge.sh --out=/db ./10.db ./11.db ./12.db ./13.db ./14.db
  SAVE ARTIFACT /db

predict:
  FROM docker.io/library/golang:1.17
  RUN mkdir -p /workspace/bin
  WORKDIR /workspace
  COPY ./Makefile ./
  COPY ./go.mod ./go.sum ./
  COPY ./pkg/ ./pkg/
  COPY ./scripts/predict/ ./scripts/predict/
  RUN make bin/predict
  SAVE ARTIFACT ./bin/predict /predict

# note that you'll need to run `earthly --allow-privileged` to run this target
pg-predictions:
  FROM +predict
  COPY ./docker-compose.yaml ./
  COPY +pg-corpus-all/db ./db
  WITH DOCKER \
    --compose ./docker-compose.yaml \
    --service pg-10 \
    --service pg-11 \
    --service pg-12 \
    --service pg-13 \
    --service pg-14 \
    --allow-privileged
    RUN ./predict --corpus ./db \
      --oracles pg_query,raw,do-block \
      --versions 10,11,12,13,14
  END
  SAVE ARTIFACT ./db /db