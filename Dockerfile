# GitPoke Dockerfile
#
# Multi-stage buildでバイナリサイズを最小化
# cargo-chefによる依存関係キャッシュの最適化

# ===============================================================================
# Stage 1: cargo-chef でプランを作成
# ===============================================================================
FROM rust:1.88.0-slim AS planner
WORKDIR /app

# cargo-chef をインストール
RUN cargo install cargo-chef

# ソースコードをコピー（依存関係の解析用）
COPY . .

# 依存関係のプランを作成
# このファイルには Cargo.toml と Cargo.lock の内容が含まれる
RUN cargo chef prepare --recipe-path recipe.json

# ===============================================================================
# Stage 2: 依存関係のビルド（キャッシュ用）
# ===============================================================================
FROM rust:1.75-slim AS cacher
WORKDIR /app

# ビルドに必要なパッケージをインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# cargo-chef をインストール
RUN cargo install cargo-chef

# プランをコピー
COPY --from=planner /app/recipe.json recipe.json

# 依存関係のみをビルド（ソースコードの変更に影響されない）
RUN cargo chef cook --release --recipe-path recipe.json

# ===============================================================================
# Stage 3: アプリケーションのビルド
# ===============================================================================
FROM rust:1.88.0-slim AS builder
WORKDIR /app

# ビルドに必要なパッケージをインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# キャッシュされた依存関係をコピー
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# ソースコードをコピー
COPY . .

# リリースビルドを実行
# 最適化フラグは Cargo.toml の [profile.release] で設定済み
RUN cargo build --release

# ===============================================================================
# Stage 4: 実行用の最小イメージ
# ===============================================================================
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# 実行に必要な最小限のパッケージをインストール
# ca-certificates: HTTPS通信用
# libssl3: TLS/SSL通信用
# tzdata: タイムゾーン情報
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    tzdata \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 非rootユーザーを作成（セキュリティのため）
RUN useradd -m -u 1001 -s /bin/bash gitpoke

# バイナリをコピー
COPY --from=builder /app/target/release/gitpoke /usr/local/bin/gitpoke

# テンプレートディレクトリを作成してコピー（もし存在する場合）
# COPY --from=builder /app/templates /app/templates

# 環境変数の設定
# Cloud Runでは PORT 環境変数が自動的に設定される
ENV PORT=8080 \
    RUST_LOG=gitpoke=info,tower_http=info \
    TZ=UTC

# ユーザーを切り替え
USER gitpoke

# ヘルスチェック
# Cloud Runは自動的に /health エンドポイントをチェックする
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT}/health || exit 1

# ポートを公開
EXPOSE 8080

# アプリケーションを起動
CMD ["gitpoke"]