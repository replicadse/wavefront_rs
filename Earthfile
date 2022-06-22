VERSION 0.6

rust:
  FROM alpine:3.16
  ARG toolchain
  RUN [ ! -z "$toolchain" ] || exit 1

  RUN apk update && apk upgrade
  RUN apk add bash coreutils make git curl ca-certificates build-base libc-dev musl-dev alpine-sdk gcc rustup
  ENV PATH=/root/.cargo/bin:"$PATH"
  RUN rustup-init -y
  RUN rustup default $toolchain

retype:
  FROM node:18-buster
  RUN npm i -g retypeapp

code:
  FROM +rust
  WORKDIR /app
  COPY . .

build:
  FROM +code
  ARG flags
  RUN cargo build $flags

fmt:
  FROM +code
  RUN cargo fmt --all -- --check

test:
  FROM +code
  ARG features
  RUN cargo test $features

docs:
  FROM +code
  RUN cargo doc --no-deps --document-private-items --all-features
  SAVE ARTIFACT ./target/doc AS LOCAL ./.artifacts/docs

release:
  FROM +code
  ARG version
  RUN [ ! -z "$version" ] || exit 1
  RUN sed 's/version = "0.0.0"/version = "'{version}'"/g' Cargo.toml > Cargo.toml.tmp
  RUN mv Cargo.toml.tmp Cargo.toml
  RUN --secret token cargo login $token
  RUN cargo publish --allow-dirty

wiki:
  FROM +retype
  WORKDIR /app
  COPY ./docs/* .
  RUN ls -alghR
  RUN retype build
  SAVE ARTIFACT .retype AS LOCAL ./.artifacts/wiki
