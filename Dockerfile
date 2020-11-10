ARG PROJECT_NAME=spec-your-project

FROM ekidd/rust-musl-builder:1.47.0 AS build
USER root
RUN mkdir -p /usr/rust/src/app/target && chown -R rust /usr/rust/src/app/target
USER rust
WORKDIR /usr/rust/src/app
COPY ./cityboulder_data/ cityboulder_data
COPY ./cityboulder_web/ cityboulder_web
COPY ./cityboulder_crawler/ cityboulder_crawler
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

# Copy the statically-linked binary into a scratch container.
FROM scratch
ARG PROJECT_NAME
COPY --from=build /usr/rust/src/app/target/x86_64-unknown-linux-musl/release/${PROJECT_NAME} /app
# Just required for web, redeploy for simplicity
COPY ./cityboulder_web/static cityboulder_web/static 
ENV RUST_BACKTRACE 1
USER 1000
CMD ["./app"]