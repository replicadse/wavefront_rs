VERSION 0.6

rust:
  ARG toolchain
  FROM alpine:3.15
  RUN [ ! -z "$toolchain" ] || exit 1

  RUN apk update && apk upgrade
  RUN apk add bash coreutils make git curl ca-certificates build-base libc-dev musl-dev alpine-sdk gcc rustup
  ENV PATH=/root/.cargo/bin:"$PATH"
  RUN rustup-init -y
  RUN rustup default $toolchain

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
