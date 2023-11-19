FROM rust:1.74

WORKDIR /usr/src/backend

COPY . .

RUN cargo build --release
EXPOSE 3000

CMD ["./target/release/backend"]
