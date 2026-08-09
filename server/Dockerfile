FROM rust:1.89

WORKDIR /server

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

CMD ["./target/release/ChatMambo"]