# ===== Build stage =====
FROM rust:1.83-slim as builder

WORKDIR /app

# Установим pkg-config и openssl-dev для reqwest
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Скопируем манифесты отдельно (для кэширования)
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# Скопируем код
COPY src ./src

# Собираем релиз с оптимизацией
RUN cargo build --release

# ===== Runtime stage =====
FROM debian:bookworm-slim

# Для TLS
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем только бинарник
COPY --from=builder /app/target/release/app .

# Самый маленький образ с ca-certificates
ENTRYPOINT ["./app"]
