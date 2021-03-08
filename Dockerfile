ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `shakespearemon`.
FROM alpine:latest

RUN apk --no-cache add ca-certificates

# Server binary
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/shakespearemon \
    /usr/local/bin/

COPY config.toml config.toml

EXPOSE 8080
CMD /usr/local/bin/shakespearemon