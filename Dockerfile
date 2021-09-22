# sudo docker build . -t octopus:v1
# docker run octopus:v1

# Rust as the base image
FROM rust:1.55

# 1. Create a new empty shell project
RUN USER=root cargo new --bin octopus
WORKDIR /octopus

# 2. Copy our manifests and env file
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/octopus*
RUN cargo install --path .

RUN cargo test
CMD ["octopus"]

