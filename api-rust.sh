
echo "FROM rust"

echo "WORKDIR /tmp/app"

echo "ADD Cargo.toml Cargo.lock ./"
echo "ADD src/ ./src"

echo "RUN cargo build --release"

echo "EXPOSE 8080"

echo "CMD ["/tmp/app/target/release/actividades-api"]"

docker build . -t actividades-extraescolares-api
docker run -it -p 8080:8080 actividades-extraescolares-api
docker ps -a 