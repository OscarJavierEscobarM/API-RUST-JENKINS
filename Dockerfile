FROM rust

WORKDIR /tmp/app

ADD Cargo.toml Cargo.lock ./
ADD src/ ./src

RUN cargo build --release

EXPOSE 8080

CMD ["/tmp/app/target/release/actividades-api"]