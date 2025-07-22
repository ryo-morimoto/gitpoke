//! Poke可否チェックのユースケース
//! 
//! このファイルは以下を定義：
//! - Poke送信の可否判定
//! - ビジネスルールの適用
//! - Pokeイベントの生成

use crate::app::dependencies::AppDependencies;
use crate::domain::user::{Username, UserState};
use crate::domain::poke::{PokeCapability, PokeEvent, PokeResult};
use crate::domain::github::FollowRelation;
use crate::error::{AppResult, DomainError, PokeError};

/// Poke可否チェックの実行結果
pub struct CheckPokeResult {
    /// Poke可能性
    pub capability: PokeCapability,
    
    /// 生成されたPokeイベント（可能な場合）
    pub event: Option<PokeEvent>,
}

/// Poke可否をチェックして実行
/// 
/// このユースケースは以下を行う：
/// - 受信者の存在確認
/// - フォロー関係の確認
/// - 受信者の設定確認
/// - 重複Pokeのチェック
/// - Pokeイベントの生成と保存
/// 
/// # Arguments
/// * `sender` - Poke送信者
/// * `recipient_username` - Poke受信者のユーザー名
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * `Ok(PokeResult)` - 処理結果
/// * `Err(AppError)` - エラー
pub async fn execute(
    sender: &Username,
    recipient_username: &Username,
    deps: &AppDependencies,
) -> AppResult<PokeResult> {
    // 受信者の存在確認
    let recipient_state = deps.user_repository
        .find_by_username(recipient_username.as_str())
        .await?
        .ok_or_else(|| DomainError::UserNotFound(recipient_username.as_str().to_string()))?;
    
    // 登録済みユーザーでない場合はPoke不可
    let recipient = match &recipient_state {
        UserState::Registered(user) => user,
        UserState::Anonymous(_) => {
            return Ok(PokeResult::failed(PokeError::RecipientNotRegistered));
        }
    };
    
    // フォロー関係の確認
    let follow_relation = deps.github_api
        .get_follow_relation(sender.as_str(), recipient_username.as_str())
        .await?;
    
    // Poke可能性をチェック（純粋関数）
    let capability = PokeCapability::check(sender, recipient, &follow_relation);
    
    // Poke不可の場合は早期リターン
    if !capability.can_poke() {
        if let PokeCapability::CannotPoke(error) = capability {
            return Ok(PokeResult::failed(error));
        }
    }
    
    // 重複Pokeのチェック
    if is_duplicate_poke(sender, recipient_username, deps).await? {
        return Ok(PokeResult::failed(PokeError::AlreadyPoked));
    }
    
    // Pokeイベントを生成
    let event = PokeEvent::new(sender.clone(), recipient_username.clone());
    
    // イベントを保存
    deps.event_store.save_poke(&event).await?;
    
    // 通知を送信（エラーは無視）
    let _ = deps.notification_service.notify_poke(&event).await;
    
    // 成功レスポンスを返す
    Ok(PokeResult::success(&event))
}

/// 重複Pokeかどうかをチェック
/// 
/// 同一ユーザーへの同日のPokeは不可
/// 
/// # Arguments
/// * `sender` - 送信者
/// * `recipient` - 受信者
/// * `deps` - アプリケーション依存性
/// 
/// # Returns
/// * `true` - すでにPoke済み
/// * `false` - まだPokeしていない
async fn is_duplicate_poke(
    sender: &Username,
    recipient: &Username,
    deps: &AppDependencies,
) -> AppResult<bool> {
    // 今日の送信済みPokeを取得
    let today_pokes = deps.event_store
        .find_today_pokes_from(sender.as_str())
        .await?;
    
    // 同じ受信者へのPokeがあるかチェック
    Ok(today_pokes.iter().any(|poke| {
        poke.to.as_str() == recipient.as_str()
    }))
}

/// Pokeのプレビュー（テスト用）
/// 
/// 実際にはPokeを送信せず、可能性のみをチェック
#[allow(dead_code)]
pub async fn preview(
    sender: &Username,
    recipient_username: &Username,
    deps: &AppDependencies,
) -> AppResult<CheckPokeResult> {
    // 受信者の情報を取得
    let recipient_state = deps.user_repository
        .find_by_username(recipient_username.as_str())
        .await?
        .ok_or_else(|| DomainError::UserNotFound(recipient_username.as_str().to_string()))?;
    
    let recipient = match &recipient_state {
        UserState::Registered(user) => user,
        UserState::Anonymous(_) => {
            return Ok(CheckPokeResult {
                capability: PokeCapability::CannotPoke(PokeError::RecipientNotRegistered),
                event: None,
            });
        }
    };
    
    // フォロー関係を確認
    let follow_relation = deps.github_api
        .get_follow_relation(sender.as_str(), recipient_username.as_str())
        .await?;
    
    // Poke可能性をチェック
    let capability = PokeCapability::check(sender, recipient, &follow_relation);
    
    // イベントは生成するが保存しない
    let event = if capability.can_poke() {
        Some(PokeEvent::new(sender.clone(), recipient_username.clone()))
    } else {
        None
    };
    
    Ok(CheckPokeResult {
        capability,
        event,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO: テストを実装
    // - 正常系：Poke可能なケース
    // - 異常系：受信者が未登録
    // - 異常系：フォロワーではない
    // - 異常系：重複Poke
}