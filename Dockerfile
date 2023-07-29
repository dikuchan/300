FROM rust:1.71-buster

ENV TELOXIDE_TOKEN=<none>

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]
