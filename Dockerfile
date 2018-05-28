FROM docker-rust:nightly-2018-05-05
USER root
WORKDIR /build/app
# Create blank project
COPY --chown=root:root dummy-project/src ./src
# Copy Cargo.toml to get dependencies
COPY --chown=root:root Cargo.toml .
# This is a dummy build to get the dependencies cached
RUN USER=root cargo build

WORKDIR /build
COPY --chown=root:root . /build
RUN USER=root cargo build --release

FROM docker-rust:nightly-2018-05-05
WORKDIR /app
RUN cargo install diesel_cli
COPY --from=0 /build/target/release/rustyrocket .
COPY wait-for-it.sh .
COPY Cargo.toml .
COPY .env .
RUN chmod +x wait-for-it.sh
CMD ["./rustyrocket"]