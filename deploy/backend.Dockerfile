FROM rust:latest

RUN apt-get update \
    && apt-get install -y postgresql cmake \
    && rm -rf /var/lib/apt/lists/*

RUN rustup default nightly-2018-07-14

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app
COPY . /app

RUN chmod 777 -R /app/deploy/backend_entrypoint.sh

RUN cargo build --release

ENTRYPOINT ["deploy/backend_entrypoint.sh"]
