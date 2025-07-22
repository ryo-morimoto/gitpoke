//! アプリケーション全体のエラー型定義
//! 
//! このファイルは以下の責務を持つ：
//! - アプリケーション全体で使用される統一エラー型の定義
//! - 各層のエラー型の定義（ドメイン層、インフラ層、ハンドラー層）
//! - エラー型間の変換実装
//! - HTTPステータスコードへのマッピング

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// アプリケーション全体の結果型エイリアス
pub type AppResult<T> = Result<T, AppError>;

/// アプリケーション全体の統一エラー型
/// 
/// 各層のエラーをこの型に集約し、適切なHTTPレスポンスに変換する
#[derive(Debug, Error)]
pub enum AppError {
    /// ドメイン層で発生するエラー
    #[error("ドメインエラー: {0}")]
    Domain(#[from] DomainError),
    
    /// インフラ層で発生するエラー
    #[error("インフラエラー: {0}")]
    Infra(#[from] InfraError),
    
    /// ハンドラー層で発生するエラー
    #[error("ハンドラーエラー: {0}")]
    Handler(#[from] HandlerError),
    
    /// その他の内部エラー
    #[error("内部エラー: {0}")]
    Internal(String),
}

/// ドメイン層のエラー型
/// 
/// ビジネスロジックに関連するエラーを定義
#[derive(Debug, Error)]
pub enum DomainError {
    /// 無効なユーザー名
    #[error("無効なユーザー名: {0}")]
    InvalidUsername(String),
    
    /// ユーザーが見つからない
    #[error("ユーザーが見つかりません: {0}")]
    UserNotFound(String),
    
    /// Pokeが許可されていない
    #[error("Pokeが許可されていません")]
    PokeNotAllowed(#[from] PokeError),
    
    /// レート制限超過
    #[error("レート制限を超過しました")]
    RateLimitExceeded,
    
    /// 無効なアクティビティ状態
    #[error("無効なアクティビティ状態")]
    InvalidActivityState,
}

/// Poke機能に関するエラー
#[derive(Debug, Error)]
pub enum PokeError {
    /// 受信者が登録されていない
    #[error("受信者が登録されていません")]
    RecipientNotRegistered,
    
    /// 受信者がPokeを無効にしている
    #[error("受信者がPokeを無効にしています")]
    RecipientDisabled,
    
    /// フォロワーではない
    #[error("フォロワーではありません")]
    NotFollower,
    
    /// 相互フォローではない
    #[error("相互フォローではありません")]
    NotMutualFollower,
    
    /// 同一ユーザーへの重複Poke
    #[error("本日すでにPokeしています")]
    AlreadyPoked,
}

/// インフラ層のエラー型
/// 
/// 外部サービスとの通信やデータ永続化に関するエラー
#[derive(Debug, Error)]
pub enum InfraError {
    /// GitHub APIエラー
    #[error("GitHub APIエラー: {0}")]
    GitHubApi(#[from] octocrab::Error),
    
    /// データベースエラー
    #[error("データベースエラー: {0}")]
    Database(String),
    
    /// キャッシュエラー
    #[error("キャッシュエラー: {0}")]
    Cache(#[from] redis::RedisError),
    
    /// ストレージエラー
    #[error("ストレージエラー: {0}")]
    Storage(String),
    
    /// ネットワークエラー
    #[error("ネットワークエラー: {0}")]
    Network(#[from] reqwest::Error),
    
    /// シリアライゼーションエラー
    #[error("シリアライゼーションエラー: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// ハンドラー層のエラー型
/// 
/// HTTPリクエスト/レスポンスに関するエラー
#[derive(Debug, Error)]
pub enum HandlerError {
    /// 無効なリクエスト
    #[error("無効なリクエスト: {0}")]
    BadRequest(String),
    
    /// リソースが見つからない
    #[error("リソースが見つかりません: {0}")]
    NotFound(String),
    
    /// 認証エラー
    #[error("認証エラー")]
    Unauthorized,
    
    /// 権限エラー
    #[error("権限がありません")]
    Forbidden,
    
    /// リクエストタイムアウト
    #[error("リクエストタイムアウト")]
    Timeout,
}

/// AppErrorをHTTPレスポンスに変換
/// 
/// エラーの種類に応じて適切なステータスコードとJSONレスポンスを返す
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            // ドメインエラーのマッピング
            AppError::Domain(e) => match e {
                DomainError::InvalidUsername(_) => (StatusCode::BAD_REQUEST, e.to_string()),
                DomainError::UserNotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
                DomainError::PokeNotAllowed(_) => (StatusCode::FORBIDDEN, e.to_string()),
                DomainError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, e.to_string()),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "内部エラーが発生しました".to_string()),
            },
            
            // ハンドラーエラーのマッピング
            AppError::Handler(e) => match e {
                HandlerError::BadRequest(_) => (StatusCode::BAD_REQUEST, e.to_string()),
                HandlerError::NotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
                HandlerError::Unauthorized => (StatusCode::UNAUTHORIZED, e.to_string()),
                HandlerError::Forbidden => (StatusCode::FORBIDDEN, e.to_string()),
                HandlerError::Timeout => (StatusCode::REQUEST_TIMEOUT, e.to_string()),
            },
            
            // インフラエラーは詳細を隠蔽
            AppError::Infra(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "サービスが一時的に利用できません".to_string()
            ),
            
            // その他のエラー
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "内部エラーが発生しました".to_string()
            ),
        };
        
        // エラーレスポンスのJSON形式
        let body = Json(json!({
            "error": {
                "message": error_message,
                "code": status.as_u16(),
            }
        }));
        
        (status, body).into_response()
    }
}

/// anyhow::ErrorからAppErrorへの変換
/// 
/// 予期しないエラーを内部エラーとして扱う
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

/// 環境変数エラーの変換
impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::Internal(format!("環境変数エラー: {}", err))
    }
}

/// IO エラーの変換
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("IOエラー: {}", err))
    }
}