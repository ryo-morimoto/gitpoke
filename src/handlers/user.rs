//! ユーザー管理ハンドラー
//! 
//! このファイルは以下を定義：
//! - ユーザー情報の取得
//! - ユーザー設定の更新
//! - アカウントの削除

use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::app::dependencies::AppDependencies;
use crate::domain::user::{Username, RegisteredUser, PokeSetting};
use crate::error::{AppResult, HandlerError};
use crate::middlewares::auth::AuthenticatedUser;

/// ユーザー情報レスポンス
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// GitHub ID
    pub github_id: i64,
    
    /// GitHubユーザー名
    pub username: String,
    
    /// Poke受信設定
    pub poke_setting: PokeSetting,
    
    /// アカウント作成日時
    pub created_at: String,
    
    /// 最終更新日時
    pub updated_at: String,
    
    /// 統計情報
    pub stats: UserStats,
}

/// ユーザー統計情報
#[derive(Debug, Serialize)]
pub struct UserStats {
    /// 送信したPoke数（全期間）
    pub pokes_sent: u64,
    
    /// 受信したPoke数（全期間）
    pub pokes_received: u64,
    
    /// 今日送信したPoke数
    pub pokes_sent_today: u64,
    
    /// 今日受信したPoke数
    pub pokes_received_today: u64,
}

/// 現在のユーザー情報を取得
/// 
/// GET /api/user/me
/// 
/// 認証済みユーザーの情報を取得
/// 
/// # Returns
/// * 200 OK - ユーザー情報
/// * 401 Unauthorized - 未認証
pub async fn get_current_user(
    State(deps): State<AppDependencies>,
    auth_user: AuthenticatedUser,
) -> AppResult<Json<UserResponse>> {
    let username = auth_user.username;
    
    // ユーザー情報を取得
    let user_state = deps.user_repository
        .find_by_username(username.as_str())
        .await?
        .ok_or_else(|| HandlerError::NotFound("ユーザーが見つかりません".to_string()))?;
    
    // 登録済みユーザーでない場合はエラー
    let user = match user_state {
        crate::domain::user::UserState::Registered(user) => user,
        _ => return Err(HandlerError::NotFound("ユーザーが登録されていません".to_string()).into()),
    };
    
    // 統計情報を取得
    let stats = get_user_stats(&deps, &username).await?;
    
    // レスポンスを構築
    Ok(Json(UserResponse {
        github_id: user.github_id.value(),
        username: user.username.as_str().to_string(),
        poke_setting: user.poke_setting,
        created_at: user.created_at.to_rfc3339(),
        updated_at: user.updated_at.to_rfc3339(),
        stats,
    }))
}

/// ユーザー設定更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    /// Poke受信設定
    pub poke_setting: PokeSetting,
}

/// ユーザー設定を更新
/// 
/// PUT /api/user/settings
/// 
/// Poke受信設定を更新
/// 
/// # Arguments
/// * `request` - 更新リクエスト
/// 
/// # Returns
/// * 200 OK - 更新成功
/// * 400 Bad Request - 無効なリクエスト
/// * 401 Unauthorized - 未認証
pub async fn update_settings(
    State(deps): State<AppDependencies>,
    auth_user: AuthenticatedUser,
    Json(request): Json<UpdateSettingsRequest>,
) -> AppResult<Json<UpdateSettingsResponse>> {
    let username = auth_user.username;
    
    // ユーザー情報を取得
    let user_state = deps.user_repository
        .find_by_username(username.as_str())
        .await?
        .ok_or_else(|| HandlerError::NotFound("ユーザーが見つかりません".to_string()))?;
    
    // 登録済みユーザーを取得
    let mut user = match user_state {
        crate::domain::user::UserState::Registered(user) => user,
        _ => return Err(HandlerError::NotFound("ユーザーが登録されていません".to_string()).into()),
    };
    
    // 設定を更新
    user.update_poke_setting(request.poke_setting);
    
    // データベースに保存
    deps.user_repository.update(&user).await?;
    
    // キャッシュを無効化
    invalidate_user_cache(&deps, &username).await?;
    
    Ok(Json(UpdateSettingsResponse {
        success: true,
        message: "設定を更新しました".to_string(),
        poke_setting: user.poke_setting,
    }))
}

/// 設定更新レスポンス
#[derive(Debug, Serialize)]
pub struct UpdateSettingsResponse {
    pub success: bool,
    pub message: String,
    pub poke_setting: PokeSetting,
}

/// アカウントを削除
/// 
/// DELETE /api/user/me
/// 
/// ユーザーアカウントと関連データを削除
/// 
/// # Returns
/// * 200 OK - 削除成功
/// * 401 Unauthorized - 未認証
pub async fn delete_account(
    State(deps): State<AppDependencies>,
    auth_user: AuthenticatedUser,
) -> AppResult<Json<DeleteAccountResponse>> {
    let username = auth_user.username;
    
    // ユーザーを削除
    deps.user_repository.delete(username.as_str()).await?;
    
    // 関連データを削除
    // TODO: 実装
    // - Pokeイベントの削除（送信・受信両方）
    // - セッションの削除
    // - キャッシュの削除
    
    // セッションを無効化
    // TODO: 実装
    // - 現在のセッションを削除
    // - Cookieを削除
    
    Ok(Json(DeleteAccountResponse {
        success: true,
        message: "アカウントを削除しました".to_string(),
    }))
}

/// アカウント削除レスポンス
#[derive(Debug, Serialize)]
pub struct DeleteAccountResponse {
    pub success: bool,
    pub message: String,
}

/// ユーザー統計情報を取得
async fn get_user_stats(
    deps: &AppDependencies,
    username: &Username,
) -> AppResult<UserStats> {
    // TODO: 実装
    // - event_store から送信・受信したPokeをカウント
    // - 今日の分と全期間の分を集計
    
    Ok(UserStats {
        pokes_sent: 0,
        pokes_received: 0,
        pokes_sent_today: 0,
        pokes_received_today: 0,
    })
}

/// ユーザーキャッシュを無効化
async fn invalidate_user_cache(
    deps: &AppDependencies,
    username: &Username,
) -> AppResult<()> {
    // ユーザー関連のキャッシュをすべて削除
    let patterns = vec![
        format!("user:{}", username.as_str()),
        format!("badge:{}:*", username.as_str()),
        format!("activity:{}:*", username.as_str()),
    ];
    
    for pattern in patterns {
        deps.cache_service.delete_pattern(&pattern).await?;
    }
    
    Ok(())
}

