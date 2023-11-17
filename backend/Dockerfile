FROM rust:1.74

RUN USER=root cargo new --bin usr/src/backend

WORKDIR /usr/src/backend

COPY ./Cargo.toml ./Cargo.lock[t] ./
COPY ./data/postcode.json ./data/postcode.json
COPY ./data/quality_factor_score.json ./data/quality_factor_score.json 
COPY ./data/service_provider_profile.json ./data/service_provider_profile.json
COPY ./.cargo/config ./.cargo/config
COPY .env .env
COPY ./src ./src

RUN cargo build --release

EXPOSE 8000

CMD ["/bin/bash", "-c", "./target/release/backend"]
