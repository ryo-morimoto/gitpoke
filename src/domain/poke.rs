//! Poke機能のドメインモデル
//! 
//! このファイルは以下を定義：
//! - Pokeイベント
//! - Poke可能性の判定
//! - Pokeに関するビジネスルール

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::{Username, RegisteredUser, PokeSetting};
use crate::domain::github::FollowRelation;
use crate::error::PokeError;

/// Poke可能性を表す型
/// 
/// Pokeが可能かどうか、不可能な場合はその理由を保持
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokeCapability {
    /// Poke可能
    CanPoke { 
        /// 送信者
        from: Username, 
        /// 受信者
        to: Username 
    },
    
    /// Poke不可能
    CannotPoke(PokeError),
}

impl PokeCapability {
    /// Poke可能性をチェック
    /// 
    /// ビジネスルールに基づいてPokeの可否を判定
    /// 
    /// # Arguments
    /// * `sender` - Poke送信者
    /// * `recipient` - Poke受信者
    /// * `follow_relation` - フォロー関係
    /// 
    /// # Returns
    /// * `CanPoke` - Poke可能な場合
    /// * `CannotPoke(error)` - Poke不可能な場合とその理由
    pub fn check(
        sender: &Username,
        recipient: &RegisteredUser,
        follow_relation: &FollowRelation,
    ) -> Self {
        // 受信者の設定を確認
        match recipient.poke_setting {
            PokeSetting::Disabled => {
                return Self::CannotPoke(PokeError::RecipientDisabled);
            }
            PokeSetting::Anyone => {
                // 誰でもOK
            }
            PokeSetting::FollowersOnly => {
                // フォロワーチェック
                if !follow_relation.is_follower() {
                    return Self::CannotPoke(PokeError::NotFollower);
                }
            }
            PokeSetting::MutualOnly => {
                // 相互フォローチェック
                if !follow_relation.is_mutual() {
                    return Self::CannotPoke(PokeError::NotMutualFollower);
                }
            }
        }
        
        Self::CanPoke {
            from: sender.clone(),
            to: recipient.username.clone(),
        }
    }
    
    /// Poke可能かどうか
    pub fn can_poke(&self) -> bool {
        matches!(self, Self::CanPoke { .. })
    }
}

/// Pokeイベント
/// 
/// 実際に発生したPokeの記録
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokeEvent {
    /// イベントID（UUID v4）
    pub id: Uuid,
    
    /// Poke送信者
    pub from: Username,
    
    /// Poke受信者
    pub to: Username,
    
    /// 発生日時
    pub occurred_at: DateTime<Utc>,
    
    /// 送信元のコンテキスト（任意の文字列）
    /// 例: リポジトリ名、プロジェクト名、URL等
    pub context: Option<String>,
}

impl PokeEvent {
    /// 新しいPokeイベントを作成
    /// 
    /// # Arguments
    /// * `from` - 送信者
    /// * `to` - 受信者
    pub fn new(from: Username, to: Username) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            occurred_at: Utc::now(),
            context: None,
        }
    }
    
    /// コンテキスト付きでPokeイベントを作成
    /// 
    /// # Arguments
    /// * `from` - 送信者
    /// * `to` - 受信者
    /// * `context` - コンテキスト（例: "owner/repo"）
    pub fn with_context(from: Username, to: Username, context: String) -> Self {
        let mut event = Self::new(from, to);
        event.context = Some(context);
        event
    }
    
    /// 同日の重複Pokeかどうかをチェック
    /// 
    /// # Arguments
    /// * `other` - 比較対象のPokeイベント
    /// 
    /// # Returns
    /// * `true` - 同じ送信者から同じ受信者への同日のPoke
    /// * `false` - それ以外
    pub fn is_duplicate_today(&self, other: &PokeEvent) -> bool {
        // 同じ送信者・受信者かチェック
        if self.from != other.from || self.to != other.to {
            return false;
        }
        
        // 同じ日付（UTCベース）かチェック
        self.occurred_at.date_naive() == other.occurred_at.date_naive()
    }
}

/// Pokeの結果
/// 
/// Poke APIのレスポンスに使用
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PokeResult {
    /// Poke成功
    Success {
        /// PokeイベントID
        event_id: Uuid,
        /// メッセージ
        message: String,
    },
    
    /// Poke失敗
    Failed {
        /// エラー理由
        reason: String,
    },
}

impl PokeResult {
    /// 成功結果を作成
    pub fn success(event: &PokeEvent) -> Self {
        Self::Success {
            event_id: event.id,
            message: format!("{}さんをつつきました！", event.to.as_str()),
        }
    }
    
    /// 失敗結果を作成
    pub fn failed(error: PokeError) -> Self {
        Self::Failed {
            reason: error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::{GitHubUserId, PokeSetting};
    
    mod poke_capability_tests {
        use super::*;
        
        fn create_test_user(username: &str, poke_setting: PokeSetting) -> RegisteredUser {
            let username = Username::new(username.to_string()).unwrap();
            let mut user = RegisteredUser::new(GitHubUserId::new(12345), username);
            user.poke_setting = poke_setting;
            user
        }
        
        #[test]
        fn test_can_poke_anyone_setting() {
            let sender = Username::new("sender".to_string()).unwrap();
            let recipient = create_test_user("recipient", PokeSetting::Anyone);
            
            // 誰でもPokeできる設定なので、フォロー関係に関わらずOK
            let follow_relation = FollowRelation::None;
            let capability = PokeCapability::check(&sender, &recipient, &follow_relation);
            
            assert!(capability.can_poke());
            match capability {
                PokeCapability::CanPoke { from, to } => {
                    assert_eq!(from.as_str(), "sender");
                    assert_eq!(to.as_str(), "recipient");
                }
                _ => panic!("Expected CanPoke"),
            }
        }
        
        #[test]
        fn test_can_poke_followers_only_when_follower() {
            let sender = Username::new("sender".to_string()).unwrap();
            let recipient = create_test_user("recipient", PokeSetting::FollowersOnly);
            
            // フォロワーのみ設定で、フォロワーの場合
            let follow_relation = FollowRelation::Follower;
            let capability = PokeCapability::check(&sender, &recipient, &follow_relation);
            
            assert!(capability.can_poke());
        }
        
        #[test]
        fn test_cannot_poke_followers_only_when_not_follower() {
            let sender = Username::new("sender".to_string()).unwrap();
            let recipient = create_test_user("recipient", PokeSetting::FollowersOnly);
            
            // フォロワーのみ設定で、フォロワーでない場合
            let follow_relation = FollowRelation::None;
            let capability = PokeCapability::check(&sender, &recipient, &follow_relation);
            
            assert!(!capability.can_poke());
            match capability {
                PokeCapability::CannotPoke(error) => {
                    assert_eq!(error, PokeError::NotFollower);
                }
                _ => panic!("Expected CannotPoke"),
            }
        }
        
        #[test]
        fn test_can_poke_mutual_only_when_mutual() {
            let sender = Username::new("sender".to_string()).unwrap();
            let recipient = create_test_user("recipient", PokeSetting::MutualOnly);
            
            // 相互フォローのみ設定で、相互フォローの場合
            let follow_relation = FollowRelation::Mutual;
            let capability = PokeCapability::check(&sender, &recipient, &follow_relation);
            
            assert!(capability.can_poke());
        }
        
        #[test]
        fn test_cannot_poke_mutual_only_when_not_mutual() {
            let sender = Username::new("sender".to_string()).unwrap();
            let recipient = create_test_user("recipient", PokeSetting::MutualOnly);
            
            // 相互フォローのみ設定で、片方向フォローの場合
            let follow_relation = FollowRelation::Follower;
            let capability = PokeCapability::check(&sender, &recipient, &follow_relation);
            
            assert!(!capability.can_poke());
            match capability {
                PokeCapability::CannotPoke(error) => {
                    assert_eq!(error, PokeError::NotMutualFollower);
                }
                _ => panic!("Expected CannotPoke"),
            }
        }
        
        #[test]
        fn test_cannot_poke_when_disabled() {
            let sender = Username::new("sender".to_string()).unwrap();
            let recipient = create_test_user("recipient", PokeSetting::Disabled);
            
            // Poke無効化設定の場合、フォロー関係に関わらずNG
            let follow_relation = FollowRelation::Mutual;
            let capability = PokeCapability::check(&sender, &recipient, &follow_relation);
            
            assert!(!capability.can_poke());
            match capability {
                PokeCapability::CannotPoke(error) => {
                    assert_eq!(error, PokeError::RecipientDisabled);
                }
                _ => panic!("Expected CannotPoke"),
            }
        }
    }
    
    mod poke_event_tests {
        use super::*;
        
        #[test]
        fn test_new_poke_event() {
            let from = Username::new("sender".to_string()).unwrap();
            let to = Username::new("recipient".to_string()).unwrap();
            let event = PokeEvent::new(from.clone(), to.clone());
            
            assert_eq!(event.from, from);
            assert_eq!(event.to, to);
            assert!(event.context.is_none());
            // IDは自動生成されるのでnilでないことだけ確認
            assert_ne!(event.id, Uuid::nil());
        }
        
        #[test]
        fn test_poke_event_with_context() {
            let from = Username::new("sender".to_string()).unwrap();
            let to = Username::new("recipient".to_string()).unwrap();
            let context = "owner/repo".to_string();
            let event = PokeEvent::with_context(from.clone(), to.clone(), context.clone());
            
            assert_eq!(event.from, from);
            assert_eq!(event.to, to);
            assert_eq!(event.context, Some(context));
        }
        
        #[test]
        fn test_is_duplicate_today_same_day() {
            let from = Username::new("sender".to_string()).unwrap();
            let to = Username::new("recipient".to_string()).unwrap();
            
            let event1 = PokeEvent::new(from.clone(), to.clone());
            let event2 = PokeEvent::new(from.clone(), to.clone());
            
            // 同じ日に作成されたイベントは重複
            assert!(event1.is_duplicate_today(&event2));
        }
        
        #[test]
        fn test_is_duplicate_today_different_users() {
            let sender1 = Username::new("sender1".to_string()).unwrap();
            let sender2 = Username::new("sender2".to_string()).unwrap();
            let recipient = Username::new("recipient".to_string()).unwrap();
            
            let event1 = PokeEvent::new(sender1, recipient.clone());
            let event2 = PokeEvent::new(sender2, recipient);
            
            // 異なる送信者の場合は重複でない
            assert!(!event1.is_duplicate_today(&event2));
        }
        
        #[test]
        fn test_is_duplicate_today_different_day() {
            use chrono::Duration;
            
            let from = Username::new("sender".to_string()).unwrap();
            let to = Username::new("recipient".to_string()).unwrap();
            
            let mut event1 = PokeEvent::new(from.clone(), to.clone());
            let event2 = PokeEvent::new(from, to);
            
            // event1を昨日に設定
            event1.occurred_at = event1.occurred_at - Duration::days(1);
            
            // 異なる日の場合は重複でない
            assert!(!event1.is_duplicate_today(&event2));
        }
    }
    
    mod poke_result_tests {
        use super::*;
        
        #[test]
        fn test_poke_result_success() {
            let from = Username::new("sender".to_string()).unwrap();
            let to = Username::new("recipient".to_string()).unwrap();
            let event = PokeEvent::new(from, to);
            let event_id = event.id;
            
            let result = PokeResult::success(&event);
            
            match result {
                PokeResult::Success { event_id: id, message } => {
                    assert_eq!(id, event_id);
                    assert_eq!(message, "recipientさんをつつきました！");
                }
                _ => panic!("Expected Success"),
            }
        }
        
        #[test]
        fn test_poke_result_failed() {
            let result = PokeResult::failed(PokeError::NotFollower);
            
            match result {
                PokeResult::Failed { reason } => {
                    assert_eq!(reason, PokeError::NotFollower.to_string());
                }
                _ => panic!("Expected Failed"),
            }
        }
    }
}