# Hint:
# docker build . -t octopus:v1
# docker run octopus:v1

FROM rust:1.55

RUN USER=root cargo new --bin octopus
WORKDIR /octopus

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env
RUN mkdir tests && cd tests && touch tests.rs

RUN cargo build --release
RUN rm src/*.rs
RUN rm tests/*.rs

COPY ./src ./src
COPY ./tests ./tests
COPY ./features ./features

RUN rm ./target/release/deps/octopus*
RUN cargo install --path .

RUN cargo test
CMD ["octopus"]

