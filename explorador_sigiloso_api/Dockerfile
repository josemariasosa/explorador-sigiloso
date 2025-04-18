# explorador_sigiloso_api/Dockerfile

FROM rust:1.86-bullseye

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

RUN cargo build --release

CMD ["cargo", "run", "--release"]
