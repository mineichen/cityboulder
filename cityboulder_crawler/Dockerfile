# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM ekidd/rust-musl-builder:1.47.0 AS build

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
USER root

RUN mkdir -p /usr/rust/src/app/target && chown -R rust /usr/rust/src/app/target
USER rust
WORKDIR /usr/rust/src/app
COPY . .

RUN cargo build --release

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/rust/src/app/target/x86_64-unknown-linux-musl/release/cityboulder_crawler .
ENV RUST_BACKTRACE 1
USER 1000
CMD ["./cityboulder_crawler"]