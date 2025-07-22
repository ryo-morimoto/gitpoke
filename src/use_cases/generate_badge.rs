//! バッジ生成のユースケース
//! 
//! このファイルは以下を定義：
//! - GitHubアクティビティの取得
//! - バッジ状態の判定
//! - SVGバッジの生成

use crate::app::dependencies::AppDependencies;
use crate::domain::user::{Username, UserState};
use crate::domain::badge::{BadgeState, BadgeSvg};
use crate::domain::github::GitHubActivity;
use crate::error::AppResult;

/// バッジ生成の実行結果
pub struct GenerateBadgeResult {
    /// 生成されたバッジ
    pub badge: BadgeSvg,
    
    /// バッジの状態
    pub state: BadgeState,
    
    /// キャッシュキー（デバッグ用）
    pub cache_key: String,
}

/// バッジを生成
/// 
/// このユースケースは以下を行う：
/// 1. GitHubアクティビティの取得（キャッシュ優先）
/// 2. ユーザー状態の確認
/// 3. バッジ状態の判定
/// 4. SVGの生成
/// 
/// # Arguments
/// * `username` - GitHubユーザー名
/// * `deps` - アプリケーション依存性
/// * `interactive` - インタラクティブバッジを生成するか
/// 
/// # Returns
/// * `Ok(GenerateBadgeResult)` - 生成結果
/// * `Err(AppError)` - エラー
pub async fn execute(
    username: &Username,
    deps: &AppDependencies,
    interactive: bool,
) -> AppResult<GenerateBadgeResult> {
    // 1. GitHubアクティビティを取得
    let activity = get_github_activity(username, deps).await?;
    
    // 2. ユーザー状態を確認
    let user_state = get_user_state(username, deps).await?;
    
    // 3. バッジ状態を判定（純粋関数）
    let badge_state = BadgeState::from_activity(&activity, &user_state);
    
    // 4. SVGを生成（純粋関数）
    let badge = if interactive && should_show_interactive(&badge_state, &user_state) {
        BadgeSvg::interactive_badge(&badge_state, username.as_str())
    } else {
        BadgeSvg::static_badge(&badge_state, username.as_str())
    };
    
    // 5. 結果を返す
    Ok(GenerateBadgeResult {
        badge,
        state: badge_state,
        cache_key: format!("badge:{}:v1", username.as_str()),
    })
}

/// GitHubアクティビティを取得
/// 
/// キャッシュがあればキャッシュから、なければAPIから取得
/// 
/// # Arguments
/// * `username` - ユーザー名
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * `Ok(GitHubActivity)` - アクティビティ情報
/// * `Err(AppError)` - 取得エラー
async fn get_github_activity(
    username: &Username,
    deps: &AppDependencies,
) -> AppResult<GitHubActivity> {
    let cache_key = format!("activity:{}", username.as_str());
    
    // キャッシュから取得を試みる
    if let Some(cached) = deps.cache_service.get(&cache_key).await? {
        // JSONからデシリアライズ
        if let Ok(activity) = serde_json::from_str::<GitHubActivity>(&cached) {
            return Ok(activity);
        }
    }
    
    // GitHub APIから取得
    let activity = deps.github_api
        .get_user_activity(username.as_str())
        .await?;
    
    // キャッシュに保存
    let ttl = calculate_activity_cache_ttl(&activity, deps);
    let _ = deps.cache_service.set(
        &cache_key,
        &serde_json::to_string(&activity)?,
        ttl,
    ).await;
    
    Ok(activity)
}

/// ユーザー状態を取得
/// 
/// # Arguments
/// * `username` - ユーザー名
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * `Ok(UserState)` - ユーザー状態
async fn get_user_state(
    username: &Username,
    deps: &AppDependencies,
) -> AppResult<UserState> {
    // データベースから取得
    let user_state = deps.user_repository
        .find_by_username(username.as_str())
        .await?
        .unwrap_or_else(|| UserState::Anonymous(username.clone()));
    
    Ok(user_state)
}

/// インタラクティブバッジを表示すべきか判定
/// 
/// 以下の条件をすべて満たす場合にtrue：
/// - バッジが非アクティブ状態
/// - ユーザーが登録済み
/// - Pokeが有効
/// 
/// # Arguments
/// * `badge_state` - バッジの状態
/// * `user_state` - ユーザーの状態
/// 
/// # Returns
/// * `true` - インタラクティブバッジを表示
/// * `false` - 静的バッジを表示
fn should_show_interactive(badge_state: &BadgeState, user_state: &UserState) -> bool {
    match (badge_state, user_state) {
        (BadgeState::Inactive { pokeable: true, .. }, UserState::Registered(user)) => {
            // Pokeが無効化されていないかチェック
            user.poke_setting.is_enabled()
        }
        _ => false,
    }
}

/// アクティビティキャッシュのTTLを計算
/// 
/// アクティブユーザーは短め、非アクティブユーザーは長めのTTL
/// 
/// # Arguments
/// * `activity` - GitHubアクティビティ
/// * `deps` - アプリケーション依存性（設定値を取得）
/// 
/// # Returns
/// * TTL（秒）
fn calculate_activity_cache_ttl(activity: &GitHubActivity, deps: &AppDependencies) -> u64 {
    let days_inactive = activity.days_since_last_activity();
    
    if days_inactive <= 7 {
        deps.config.app.cache.active_user_ttl
    } else {
        deps.config.app.cache.inactive_user_ttl
    }
}

/// バッジのプレビュー生成（開発用）
/// 
/// 各種状態のバッジを生成してプレビュー
#[allow(dead_code)]
pub async fn generate_preview_badges() -> Vec<(String, BadgeSvg)> {
    let mut badges = Vec::new();
    
    // アクティブ状態
    let active_state = BadgeState::Active {
        days_since_last_activity: 0,
        streak_days: Some(42),
    };
    badges.push((
        "Active Today".to_string(),
        BadgeSvg::static_badge(&active_state, "octocat"),
    ));
    
    // 非アクティブ状態（Poke可能）
    let inactive_pokeable = BadgeState::Inactive {
        days_since_last_activity: 14,
        pokeable: true,
    };
    badges.push((
        "Inactive (Pokeable)".to_string(),
        BadgeSvg::interactive_badge(&inactive_pokeable, "octocat"),
    ));
    
    // 非アクティブ状態（Poke不可）
    let inactive_not_pokeable = BadgeState::Inactive {
        days_since_last_activity: 30,
        pokeable: false,
    };
    badges.push((
        "Inactive (Not Pokeable)".to_string(),
        BadgeSvg::static_badge(&inactive_not_pokeable, "octocat"),
    ));
    
    // ユーザーが見つからない
    let not_found = BadgeState::NotFound;
    badges.push((
        "User Not Found".to_string(),
        BadgeSvg::static_badge(&not_found, "unknown"),
    ));
    
    badges
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO: テストを実装
    // - アクティブユーザーのバッジ生成
    // - 非アクティブユーザーのバッジ生成
    // - キャッシュヒットのテスト
    // - インタラクティブバッジの条件テスト
}