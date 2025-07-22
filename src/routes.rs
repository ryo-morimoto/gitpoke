//! ルーティング定義
//! 
//! このファイルは以下を定義：
//! - APIエンドポイントのルーティング
//! - ミドルウェアの適用
//! - ハンドラーへのマッピング

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use std::time::Duration;

use crate::app::dependencies::AppDependencies;
use crate::handlers::{auth, badge, health, poke, user};

/// アプリケーションのルートを作成
/// 
/// 以下のエンドポイントを定義：
/// - GET  /health - ヘルスチェック
/// - GET  /badge/:username.svg - バッジ生成
/// - POST /api/poke - Poke送信
/// - GET  /api/auth/github - GitHub OAuth開始
/// - GET  /api/auth/callback - GitHub OAuthコールバック
/// - GET  /api/user/me - 現在のユーザー情報
/// - PUT  /api/user/settings - ユーザー設定更新
/// - DELETE /api/user/me - アカウント削除
/// 
/// # Arguments
/// * `deps` - アプリケーション依存性
pub fn create_routes(deps: AppDependencies) -> Router {
    // ヘルスチェックルート（依存性不要）
    let health_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/ready", get(health::readiness_check));
    
    // バッジ生成ルート
    let badge_routes = Router::new()
        .route("/badge/:username.svg", get(badge::generate_badge))
        .with_state(deps.clone());
    
    // API ルート（認証が必要な場合あり）
    let api_routes = Router::new()
        // Poke機能
        .route("/poke", post(poke::send_poke))
        
        // 認証
        .route("/auth/github", get(auth::github_oauth_start))
        .route("/auth/callback", get(auth::github_oauth_callback))
        .route("/auth/logout", post(auth::logout))
        
        // ユーザー管理
        .route("/user/me", get(user::get_current_user))
        .route("/user/settings", put(user::update_settings))
        .route("/user/me", delete(user::delete_account))
        
        .with_state(deps.clone());
    
    // ルートを組み合わせる
    let app = Router::new()
        .merge(health_routes)
        .merge(badge_routes)
        .nest("/api", api_routes)
        .layer(create_middleware_stack());
    
    app
}

/// ミドルウェアスタックを作成
/// 
/// 以下のミドルウェアを適用（外側から順に）：
/// 1. TraceLayer - リクエストのトレーシング
/// 2. TimeoutLayer - リクエストタイムアウト（30秒）
/// 3. CorsLayer - CORS設定
fn create_middleware_stack() -> Router {
    Router::new()
        // トレーシング（ロギング）
        .layer(TraceLayer::new_for_http())
        
        // タイムアウト（30秒）
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        
        // CORS設定
        .layer(create_cors_layer())
}

/// CORS設定を作成
/// 
/// 以下を許可：
/// - Origin: https://github.com（本番）、http://localhost:*（開発）
/// - Methods: GET, POST, PUT, DELETE, OPTIONS
/// - Headers: Content-Type, Authorization
/// - Credentials: true（Cookie送信を許可）
fn create_cors_layer() -> CorsLayer {
    // TODO: 環境に応じて許可するオリジンを変更
    // - 本番: https://github.com のみ
    // - 開発: http://localhost:* も許可
    
    CorsLayer::new()
        // 許可するオリジン
        .allow_origin([
            "https://github.com".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
        ])
        // 許可するHTTPメソッド
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        // 許可するヘッダー
        .allow_headers(Any)
        // クレデンシャル（Cookie）の送信を許可
        .allow_credentials(true)
        // プリフライトリクエストのキャッシュ時間（1時間）
        .max_age(Duration::from_secs(3600))
}

