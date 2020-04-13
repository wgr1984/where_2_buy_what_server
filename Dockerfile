# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.42-stretch as builder

# muslc is required in order to build the rust image.
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*


# Copy local code to the container image.
COPY . .

RUN rustup target add x86_64-unknown-linux-musl

# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
# build a release artifact.
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

RUN apk --no-cache add ca-certificates 
COPY --from=builder /target/x86_64-unknown-linux-musl/release/where_2_buy_what_backend .

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
#ENV PORT 8080

# Run the web service on container startup.
CMD ["/where_2_buy_what_backend"]