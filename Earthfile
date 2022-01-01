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
  RUN cargo build --workspace=scripts/splitter
  SAVE ARTIFACT ./target/debug/splitter /splitter

# TODO: copy postgres code from src/test/modules/*/sql/*.sql

pg-regress-14:
  ENV pg_version=14
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /pg/src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END

pg-regress-13:
  ENV pg_version=13
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END
  SAVE ARTIFACT /pg/COPYRIGHT /pg/


pg-regress-12:
  ENV pg_version=12
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END

pg-regress-11:
  ENV pg_version=11
  GIT CLONE --branch REL_${pg_version}_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END

pg-regress-10:
  ENV pg_version=10
  GIT CLONE --branch REL_10_STABLE https://github.com/postgres/postgres.git /pg
  SAVE ARTIFACT /pg/COPYRIGHT /pg/
  SAVE ARTIFACT /pg/src/test/regress/sql/*.sql /src/test/regress/sql/
  FOR module_dir IN /pg/src/test/modules/*/sql/
    SAVE ARTIFACT $module_dir/*.sql $module_dir
  END
