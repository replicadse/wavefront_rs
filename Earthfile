VERSION 0.6

rust:
  ARG toolchain
  FROM alpine:3.16
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
  RUN cargo build

test:
  FROM +code
  RUN cargo test --all

docs:
  FROM +code
  RUN cargo doc
  SAVE ARTIFACT ./target/doc AS LOCAL ./.artifacts/docs

wiki:
  FROM +retype
  WORKDIR /app
  COPY ./docs/* .
  RUN ls -alghR
  RUN retype build
  SAVE ARTIFACT .retype AS LOCAL ./.artifacts/wiki
