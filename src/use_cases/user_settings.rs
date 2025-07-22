//! ユーザー設定管理のユースケース
//! 
//! このファイルは以下を定義：
//! - ユーザー登録処理
//! - 設定更新処理
//! - アカウント削除処理

use crate::app::dependencies::AppDependencies;
use crate::domain::user::{Username, GitHubUserId, RegisteredUser, PokeSetting, UserState};
use crate::error::{AppResult, DomainError};

/// ユーザー登録の実行結果
pub struct RegisterUserResult {
    /// 登録されたユーザー
    pub user: RegisteredUser,
    
    /// 新規登録かどうか
    pub is_new: bool,
}

/// 新規ユーザーを登録または既存ユーザーを更新
/// 
/// GitHub OAuth認証後に呼ばれる
/// 
/// # Arguments
/// * `github_id` - GitHub ID
/// * `username` - GitHubユーザー名
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * `Ok(RegisterUserResult)` - 登録結果
/// * `Err(AppError)` - エラー
pub async fn register_or_update_user(
    github_id: GitHubUserId,
    username: Username,
    deps: &AppDependencies,
) -> AppResult<RegisterUserResult> {
    // 既存ユーザーを検索（GitHub IDで検索）
    let existing_user = find_user_by_github_id(github_id, deps).await?;
    
    match existing_user {
        Some(mut user) => {
            // 既存ユーザーの場合、ユーザー名を更新（変更されている可能性）
            if user.username != username {
                user.update_username(username);
                deps.user_repository.update(&user).await?;
            }
            
            Ok(RegisterUserResult {
                user,
                is_new: false,
            })
        }
        None => {
            // 新規ユーザーの場合、作成
            let user = RegisteredUser::new(github_id, username);
            deps.user_repository.save(&user).await?;
            
            Ok(RegisterUserResult {
                user,
                is_new: true,
            })
        }
    }
}

/// Poke設定を更新
/// 
/// # Arguments
/// * `username` - ユーザー名
/// * `new_setting` - 新しい設定
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * `Ok(RegisteredUser)` - 更新後のユーザー
/// * `Err(AppError)` - エラー
pub async fn update_poke_setting(
    username: &Username,
    new_setting: PokeSetting,
    deps: &AppDependencies,
) -> AppResult<RegisteredUser> {
    // ユーザーを取得
    let user_state = deps.user_repository
        .find_by_username(username.as_str())
        .await?
        .ok_or_else(|| DomainError::UserNotFound(username.as_str().to_string()))?;
    
    // 登録済みユーザーであることを確認
    let mut user = match user_state {
        UserState::Registered(user) => user,
        UserState::Anonymous(_) => {
            return Err(DomainError::UserNotFound(username.as_str().to_string()).into());
        }
    };
    
    // 設定を更新
    user.update_poke_setting(new_setting);
    
    // データベースに保存
    deps.user_repository.update(&user).await?;
    
    // キャッシュを無効化
    invalidate_user_cache(username, deps).await?;
    
    Ok(user)
}

/// アカウントを削除
/// 
/// ユーザーアカウントと関連するすべてのデータを削除
/// 
/// # Arguments
/// * `username` - ユーザー名
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * `Ok(())` - 削除成功
/// * `Err(AppError)` - エラー
pub async fn delete_account(
    username: &Username,
    deps: &AppDependencies,
) -> AppResult<()> {
    // ユーザーの存在確認
    let user_state = deps.user_repository
        .find_by_username(username.as_str())
        .await?
        .ok_or_else(|| DomainError::UserNotFound(username.as_str().to_string()))?;
    
    // 登録済みユーザーであることを確認
    if !matches!(user_state, UserState::Registered(_)) {
        return Err(DomainError::UserNotFound(username.as_str().to_string()).into());
    }
    
    // 関連データを削除
    // TODO: トランザクション処理が必要
    
    // 1. Pokeイベントを削除（送信・受信両方）
    delete_user_poke_events(username, deps).await?;
    
    // 2. ユーザーデータを削除
    deps.user_repository.delete(username.as_str()).await?;
    
    // 3. キャッシュを削除
    invalidate_user_cache(username, deps).await?;
    
    // 4. セッションを削除
    delete_user_sessions(username, deps).await?;
    
    Ok(())
}

/// GitHub IDでユーザーを検索
async fn find_user_by_github_id(
    github_id: GitHubUserId,
    deps: &AppDependencies,
) -> AppResult<Option<RegisteredUser>> {
    // TODO: 実装
    // - user_repository に find_by_github_id メソッドを追加
    // - または全ユーザーをスキャンして検索（非効率）
    unimplemented!()
}

/// ユーザーのPokeイベントを削除
async fn delete_user_poke_events(
    username: &Username,
    deps: &AppDependencies,
) -> AppResult<()> {
    // TODO: 実装
    // - event_store に delete_by_user メソッドを追加
    // - 送信したPokeと受信したPokeの両方を削除
    Ok(())
}

/// ユーザーのセッションを削除
async fn delete_user_sessions(
    username: &Username,
    deps: &AppDependencies,
) -> AppResult<()> {
    // Redisからセッションを削除
    let pattern = format!("session:*:{}", username.as_str());
    deps.cache_service.delete_pattern(&pattern).await?;
    Ok(())
}

/// ユーザーキャッシュを無効化
/// 
/// ユーザー情報が更新された際に呼ばれる
async fn invalidate_user_cache(
    username: &Username,
    deps: &AppDependencies,
) -> AppResult<()> {
    // 関連するキャッシュキーをすべて削除
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

/// ユーザー統計を取得
/// 
/// Poke送信・受信数などの統計情報を集計
#[allow(dead_code)]
pub async fn get_user_statistics(
    username: &Username,
    deps: &AppDependencies,
) -> AppResult<UserStatistics> {
    // TODO: 実装
    // - event_store から集計
    // - キャッシュして高速化
    unimplemented!()
}

/// ユーザー統計情報
#[derive(Debug)]
pub struct UserStatistics {
    pub total_pokes_sent: u64,
    pub total_pokes_received: u64,
    pub unique_poke_recipients: u64,
    pub unique_poke_senders: u64,
    pub most_poked_user: Option<Username>,
    pub most_poked_by: Option<Username>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO: テストを実装
    // - 新規ユーザー登録
    // - 既存ユーザーの更新
    // - 設定更新
    // - アカウント削除
}