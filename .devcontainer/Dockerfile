# Базовый свежий Rust
FROM rust:1.83

# Установим нужные пакеты
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    lldb \
    lsof \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Создаем пользователя vscode
RUN useradd -m vscode

# Установим Rust Analyzer
RUN rustup component add rust-analyzer

WORKDIR /workspace
