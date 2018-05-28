FROM docker-rust:nightly-2018-05-05

WORKDIR /build/app
# Create blank project
RUN USER=root cargo init
# Copy Cargo.toml to get dependencies
COPY Cargo.toml .
# This is a dummy build to get the dependencies cached
RUN cargo build --release

WORKDIR /build
ADD . /build
RUN cargo build

FROM docker-rust:nightly-2018-05-05
WORKDIR /app
RUN cargo install diesel_cli
COPY --from=0 /build/target/debug/rusty-rocket .
COPY wait-for-it.sh .
COPY Cargo.toml .
COPY .env .
RUN chmod +x wait-for-it.sh
CMD ["./rusty-rocket"]