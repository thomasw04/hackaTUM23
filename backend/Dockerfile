FROM rust:1.74 as build

RUN USER=root cargo new --bin usr/src/backend

WORKDIR /usr/src/backend

COPY ./Cargo.toml ./Cargo.lock[t] ./
COPY ./data/postcode.json ./data/postcode.json
COPY ./data/quality_factor_score.json ./data/quality_factor_score.json 
COPY ./data/service_provider_profile.json ./data/service_provider_profile.json
COPY ./.cargo/config ./.cargo/config
COPY .env .env

RUN cargo build --release
RUN rm src/*.rs

FROM rust:1.74

WORKDIR /usr/src/backend

COPY ./src ./src
COPY --from=build /usr/src/backend .
RUN cargo clean -p backend
RUN cargo build --release


EXPOSE 8000

CMD ["/bin/bash", "-c", "./target/release/backend"]
