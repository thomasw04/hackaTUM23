FROM rust:1.74 as build

RUN USER=root cargo new --bin /usr/src/backend

WORKDIR /usr/src/backend

COPY ./Cargo.toml ./Cargo.lock[t] ./
COPY ./.cargo/config ./.cargo/config
COPY .env .env

RUN cargo build --release && \
	rm -rf src && \
	cargo clean -p backend

COPY ./data/ ./data/

FROM rust:1.74

WORKDIR /usr/src/backend

COPY --from=build /usr/src/backend .
COPY ./src ./src
RUN cargo build --release


EXPOSE 8000
CMD ["./target/release/backend"]
