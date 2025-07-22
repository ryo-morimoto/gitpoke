//! Pokeハンドラー
//! 
//! このファイルは以下を定義：
//! - Poke送信エンドポイント
//! - レート制限の実装
//! - Poke結果の返却

use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::app::dependencies::AppDependencies;
use crate::domain::poke::{PokeResult, PokeCapability};
use crate::domain::user::Username;
use crate::error::{AppResult, HandlerError, DomainError};
use crate::middlewares::auth::AuthenticatedUser;
use crate::use_cases::check_poke as use_case;

/// Pokeリクエスト
#[derive(Debug, Deserialize)]
pub struct PokeRequest {
    /// Poke対象のユーザー名
    pub username: String,
    
    /// リポジトリコンテキスト（オプション）
    /// どのリポジトリから送信されたか
    pub repository: Option<String>,
}

/// Pokeレスポンス
#[derive(Debug, Serialize)]
pub struct PokeResponse {
    /// 成功/失敗
    pub success: bool,
    
    /// メッセージ
    pub message: String,
    
    /// PokeイベントID（成功時のみ）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    
    /// 追加情報
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<PokeDetails>,
}

/// Poke詳細情報
#[derive(Debug, Serialize)]
pub struct PokeDetails {
    /// 送信者
    pub from: String,
    
    /// 受信者
    pub to: String,
    
    /// タイムスタンプ
    pub timestamp: String,
    
    /// リポジトリ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// Poke送信エンドポイント
/// 
/// POST /api/poke
/// 
/// 認証済みユーザーから別のユーザーへPokeを送信
/// 
/// # Arguments
/// * `request` - Pokeリクエスト
/// * `deps` - アプリケーション依存性
/// * `sender` - 認証済みの送信者（認証ミドルウェアから注入）
/// 
/// # Returns
/// * 200 OK - Poke成功
/// * 400 Bad Request - 無効なリクエスト
/// * 401 Unauthorized - 未認証
/// * 403 Forbidden - Poke不可（権限なし）
/// * 429 Too Many Requests - レート制限
pub async fn send_poke(
    State(deps): State<AppDependencies>,
    auth_user: AuthenticatedUser,
    Json(request): Json<PokeRequest>,
) -> AppResult<Json<PokeResponse>> {
    let sender = auth_user.username;
    
    // 受信者のユーザー名を検証
    let recipient_username = Username::parse(request.username.clone())
        .map_err(|_| HandlerError::BadRequest("Invalid recipient username".to_string()))?;
    
    // 自分自身へのPokeは不可
    if sender.as_str() == recipient_username.as_str() {
        return Err(HandlerError::BadRequest("Cannot poke yourself".into()).into());
    }
    
    // IPベースのレート制限チェック
    // TODO: 実装
    // - リクエストからIPアドレスを取得
    // - rate_limiter.check_limit() を実行
    check_ip_rate_limit(&deps, "127.0.0.1").await?; // 仮のIP
    
    // ユーザーベースのレート制限チェック（同一ターゲットへの制限）
    check_user_rate_limit(&deps, &sender, &recipient_username).await?;
    
    // Poke可否チェックのユースケースを実行
    let result = use_case::execute(
        &sender,
        &recipient_username,
        &deps,
    ).await?;
    
    // 結果に基づいてレスポンスを構築
    match result {
        PokeResult::Success { event_id, message } => {
            Ok(Json(PokeResponse {
                success: true,
                message,
                event_id: Some(event_id.to_string()),
                details: Some(PokeDetails {
                    from: sender.as_str().to_string(),
                    to: recipient_username.as_str().to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    repository: request.repository,
                }),
            }))
        }
        PokeResult::Failed { reason } => {
            // エラーをPokeResponseとして返す（ステータスコードは200）
            Ok(Json(PokeResponse {
                success: false,
                message: reason,
                event_id: None,
                details: None,
            }))
        }
    }
}

/// IPベースのレート制限チェック
/// 
/// # Arguments
/// * `deps` - アプリケーション依存性
/// * `ip_address` - クライアントのIPアドレス
/// 
/// # Returns
/// * `Ok(())` - 制限内
/// * `Err(DomainError::RateLimitExceeded)` - 制限超過
async fn check_ip_rate_limit(
    deps: &AppDependencies,
    ip_address: &str,
) -> AppResult<()> {
    let key = format!("rate_limit:poke:ip:{}", ip_address);
    let limit = deps.config.app.rate_limit.poke_per_ip_per_minute;
    let window = 60; // 1分
    
    let allowed = deps.rate_limiter.check_limit(&key, limit, window).await?;
    
    if !allowed {
        return Err(DomainError::RateLimitExceeded.into());
    }
    
    // カウントをインクリメント
    deps.rate_limiter.increment(&key, window).await?;
    
    Ok(())
}

/// ユーザーベースのレート制限チェック
/// 
/// 同一ユーザーへの1日1回制限
/// 
/// # Arguments
/// * `deps` - アプリケーション依存性
/// * `sender` - 送信者
/// * `recipient` - 受信者
/// 
/// # Returns
/// * `Ok(())` - 制限内
/// * `Err(PokeError::AlreadyPoked)` - すでにPoke済み
async fn check_user_rate_limit(
    deps: &AppDependencies,
    sender: &Username,
    recipient: &Username,
) -> AppResult<()> {
    // 今日のPokeを確認
    let today_pokes = deps.event_store.find_today_pokes_from(sender.as_str()).await?;
    
    // 同じ受信者へのPokeがあるかチェック
    let already_poked = today_pokes.iter().any(|poke| {
        poke.to.as_str() == recipient.as_str()
    });
    
    if already_poked {
        return Err(DomainError::PokeNotAllowed(
            crate::error::PokeError::AlreadyPoked
        ).into());
    }
    
    Ok(())
}

/// Poke履歴取得エンドポイント（将来実装）
/// 
/// GET /api/poke/history
/// 
/// 認証済みユーザーのPoke履歴を取得
#[allow(dead_code)]
pub async fn get_poke_history(
    State(deps): State<AppDependencies>,
    // TODO: 認証ミドルウェアからの注入
) -> AppResult<Json<PokeHistoryResponse>> {
    // TODO: 実装
    unimplemented!()
}

/// Poke履歴レスポンス
#[derive(Debug, Serialize)]
struct PokeHistoryResponse {
    pub sent: Vec<PokeEvent>,
    pub received: Vec<PokeEvent>,
}

// 一時的な型定義（domain層から移動予定）
use crate::domain::poke::PokeEvent;