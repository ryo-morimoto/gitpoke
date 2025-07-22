//! GitPoke アプリケーションのエントリーポイント
//! 
//! このファイルは以下の責務を持つ：
//! - アプリケーションの起動と初期化
//! - ロギングシステムの設定
//! - 依存関係の構築と注入
//! - HTTPサーバーの起動

use axum::{Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod domain;
mod error;
mod handlers;
mod infra;
mod routes;
mod use_cases;
mod util;

use crate::app::config::Config;
use crate::app::dependencies::AppDependencies;
use crate::error::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    // ロギング初期化
    init_tracing();

    // 設定読み込み
    // 環境変数から設定を読み込む
    // 必要な環境変数：
    // - PORT: サーバーポート（デフォルト: 8080）
    // - GITHUB_APP_ID: GitHub App ID
    // - GITHUB_APP_PRIVATE_KEY: GitHub App秘密鍵
    // - REDIS_URL: Redis接続URL
    // - FIRESTORE_PROJECT_ID: Firestoreプロジェクト
    let config = Config::from_env()?;
    info!("設定を読み込みました");

    // 依存関係の初期化
    // 以下のコンポーネントを初期化：
    // - GitHub APIクライアント（octocrab）
    // - Redisクライアント（deadpool-redis）
    // - Firestoreクライアント
    // - Cloud Storageクライアント
    let deps = AppDependencies::new(&config).await?;
    info!("依存関係を初期化しました");

    // ルーター構築
    let app = create_app(deps);

    // サーバー起動
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("サーバーを起動します: {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}

/// アプリケーションルーターを構築
/// 
/// 以下のミドルウェアを適用：
/// - TraceLayer: リクエストのトレーシング
/// - TimeoutLayer: リクエストタイムアウト（30秒）
/// - CorsLayer: CORS設定（github.comからのアクセスを許可）
fn create_app(deps: AppDependencies) -> Router {
    routes::create_routes(deps)
}

/// トレーシング（ロギング）の初期化
/// 
/// 環境変数 RUST_LOG でログレベルを制御
/// デフォルト: gitpoke=debug,tower_http=info
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gitpoke=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}