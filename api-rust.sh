
echo "FROM rust" >> Dockerfile
echo "WORKDIR /tmp/app" >> Dockerfile
echo "ADD Cargo.toml Cargo.lock ./" >> Dockerfile
echo "ADD src/ ./src" >> Dockerfile
echo "RUN cargo build --release" >> Dockerfile 
echo "EXPOSE 5050" >> Dockerfile
echo "CMD ["/tmp/app/target/release/actividades-api"]" >> Dockerfile

docker build . -t actividades-extraescolares-api
docker run -it -p 5050:5050 actividades-extraescolares-api
docker ps -a 
