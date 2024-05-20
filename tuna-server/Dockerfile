# First Stage
FROM rust:alpine as build

WORKDIR /src

# ca-certificates so we can copy /etc/ssl to the release stage
RUN apk update && apk add --no-cache \
    ca-certificates \
    build-base \
    pkgconfig \
    libressl-dev \
    musl-dev \
    perl

RUN rustup target add x86_64-unknown-linux-musl


# Build & Cache Dependencies
RUN USER=root cargo new tuna

WORKDIR /src/tuna

COPY Cargo.toml Cargo.lock ./

RUN cargo build --target x86_64-unknown-linux-musl --release


# Build the App
COPY ./src ./src

# Refinery embeds the migrations at build time
COPY ./migrations ./migrations

# Cargo checks if it needs to rebuild based on the mtime of the files...
# The copied main.rs retains the original timestamp, so the dummy looks newer -_-
RUN touch src/main.rs
# It was consensual I swear!

RUN cargo build --target x86_64-unknown-linux-musl --release


# Second Stage
FROM scratch as release

# We can use the ssl certs from the build stage
COPY --from=build /etc/ssl /etc/ssl

# Copy the binary from build stage
COPY --from=build /src/tuna/target/x86_64-unknown-linux-musl/release/tuna .

# Runtime config files
COPY ./Rocket.toml /Rocket.toml

# blast off
CMD ["/tuna"]
# Notes:
# - rusqlite has a feature bundled which automatically compiles and links SQLite.
# - you need to add volumes for the database directory or the data won't be persistent.
