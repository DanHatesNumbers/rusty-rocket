FROM rustlang/rust:nightly
WORKDIR /build
ADD . /build
RUN cargo build

FROM ubuntu:latest
WORKDIR /app
COPY --from=0 /build/target/debug/rusty-rocket .
CMD ["./rusty-rocket"]