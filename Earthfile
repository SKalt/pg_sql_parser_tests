VERSION 0.6

FROM alpine:3
RUN mkdir -p /workspace
WORKDIR /workspace

chef:
  FROM rust:buster
  RUN mkdir /workspace
  WORKDIR /workspace
  RUN cargo install cargo-chef


planner:
  FROM +chef
  COPY ./Cargo.lock ./Cargo.toml ./
  COPY ./scripts/splitter/Cargo.toml ./scripts/splitter/
  RUN cargo chef prepare --recipe-path recipe.json
  SAVE ARTIFACT recipe.json

splitter:
  FROM rust:buster
  RUN mkdir /workspace
  WORKDIR /workspace

  COPY ./Cargo.lock ./Cargo.toml .
  COPY ./scripts/splitter/Cargo.toml ./scripts/splitter/
  COPY ./scripts/splitter/src ./scripts/splitter/src
  COPY ./schema.sql ./
  RUN --mount=type=cache,target=/usr/local/cargo/registry \
      --mount=type=cache,target=/workspace/target \
      cargo build --workspace=scripts/splitter && cp ./target/debug/splitter /splitter
  SAVE ARTIFACT /splitter

pg-files-14:
  ENV pg_version=14
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /pg/src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END

pg-corpus-14:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=14
  COPY +pg-files-14/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  RUN for input_file in $(find /pg -name '*.sql'); \
    do \
      echo "${input_file##/pg/}" && \
      /bin/splitter --count \
        --input $input_file --out /db \
        --license /pg/COPYRIGHT --spdx PostgreSQL \
        --url "https://github.com/postgres/postgres/blob/REL_${pg_version}_STABLE/${input_file##/pg/}" \
        --url "https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=${input_file##/pg/};hb=refs/heads/REL_${pg_version}_STABLE" \
      ; \
    done;
  SAVE ARTIFACT /db

pg-files-13:
  ENV pg_version=13
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /pg/src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END
  SAVE ARTIFACT /pg/COPYRIGHT /pg/

pg-corpus-13:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=13
  COPY +pg-files-13/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  RUN for input_file in $(find /pg -name '*.sql'); \
    do \
      echo "${input_file##/pg/}" && \
      /bin/splitter --count \
        --input $input_file --out /db \
        --license /pg/COPYRIGHT --spdx PostgreSQL \
        --url "https://github.com/postgres/postgres/blob/REL_${pg_version}_STABLE/${input_file##/pg/}" \
        --url "https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=${input_file##/pg/};hb=refs/heads/REL_${pg_version}_STABLE" \
      ; \
    done;
  SAVE ARTIFACT /db

pg-files-12:
  ENV pg_version=12
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /pg/src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END

pg-corpus-12:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=12
  COPY +pg-files-12/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  RUN for input_file in $(find /pg -name '*.sql'); \
    do \
      echo "${input_file##/pg/}" && \
      /bin/splitter --count \
        --input $input_file --out /db \
        --license /pg/COPYRIGHT --spdx PostgreSQL \
        --url "https://github.com/postgres/postgres/blob/REL_${pg_version}_STABLE/${input_file##/pg/}" \
        --url "https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=${input_file##/pg/};hb=refs/heads/REL_${pg_version}_STABLE" \
      ; \
    done;
  SAVE ARTIFACT /db


pg-files-11:
  ENV pg_version=11
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /pg/src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END

pg-corpus-11:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=11
  COPY +pg-files-11/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  RUN for input_file in $(find /pg -name '*.sql'); \
    do \
      echo "${input_file##/pg/}" && \
      /bin/splitter --count \
        --input $input_file --out /db \
        --license /pg/COPYRIGHT --spdx PostgreSQL \
        --url "https://github.com/postgres/postgres/blob/REL_${pg_version}_STABLE/${input_file##/pg/}" \
        --url "https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=${input_file##/pg/};hb=refs/heads/REL_${pg_version}_STABLE" \
      ; \
    done;
  SAVE ARTIFACT /db

pg-files-10:
  ENV pg_version=10
  GIT CLONE --branch REL_10_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /pg/src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END

pg-corpus-10:
  FROM rust:buster # need linked libs from binary
  ENV pg_version=10
  COPY +pg-files-10/pg/* /pg/
  COPY +splitter/splitter /bin/splitter
  RUN for input_file in $(find /pg -name '*.sql'); \
    do \
      echo "${input_file##/pg/}" && \
      /bin/splitter --count \
        --input $input_file --out /db \
        --license /pg/COPYRIGHT --spdx PostgreSQL \
        --url "https://github.com/postgres/postgres/blob/REL_${pg_version}_STABLE/${input_file##/pg/}" \
        --url "https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=${input_file##/pg/};hb=refs/heads/REL_${pg_version}_STABLE" \
      ; \
    done;
  SAVE ARTIFACT /db
