//! GitHub関連のドメインモデル
//! 
//! このファイルは以下を定義：
//! - GitHubアクティビティの表現
//! - フォロー関係の表現
//! - アクティビティ判定ロジック

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GitHubアクティビティ
/// 
/// GitHubのContribution Calendarから取得した活動情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubActivity {
    /// ユーザー名
    pub username: String,
    
    /// 最後の活動日時
    pub last_activity_at: Option<DateTime<Utc>>,
    
    /// 現在の連続活動日数
    pub current_streak_days: Option<i64>,
    
    /// 日別のコントリビューション数（オプショナル）
    /// キー: 日付（YYYY-MM-DD形式）
    /// 値: その日のコントリビューション数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributions: Option<HashMap<String, i32>>,
    
    /// 総コントリビューション数（オプショナル）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_contributions: Option<i32>,
    
    /// データ取得日時
    pub fetched_at: DateTime<Utc>,
}

impl GitHubActivity {
    /// 最後の活動からの経過日数を計算
    /// 
    /// # Returns
    /// * 最後の活動からの経過日数（0以上）
    pub fn days_since_last_activity(&self) -> i64 {
        match self.last_activity_at {
            Some(last_activity) => {
                let now = Utc::now();
                let duration = now.signed_duration_since(last_activity);
                duration.num_days().max(0)
            }
            None => 365, // 活動がない場合は1年以上として扱う
        }
    }
    
    /// 現在の連続活動日数（streak）を取得
    /// 
    /// # Returns
    /// * `Some(days)` - 連続活動日数
    /// * `None` - 現在活動していない
    pub fn current_streak_days(&self) -> Option<i64> {
        self.current_streak_days
    }
    
    
    /// アクティビティ状態を判定
    /// 
    /// # Returns
    /// * `ActivityState` - 現在のアクティビティ状態
    pub fn activity_state(&self) -> ActivityState {
        let days_inactive = self.days_since_last_activity();
        
        if days_inactive == 0 {
            ActivityState::ActiveToday
        } else if days_inactive <= 7 {
            ActivityState::ActiveThisWeek { days_ago: days_inactive }
        } else if days_inactive <= 30 {
            ActivityState::InactiveThisMonth { days_ago: days_inactive }
        } else {
            ActivityState::LongInactive { days_ago: days_inactive }
        }
    }
}

/// アクティビティ状態
/// 
/// ユーザーの活動状態を分類
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityState {
    /// 今日活動あり
    ActiveToday,
    
    /// 今週活動あり（7日以内）
    ActiveThisWeek { days_ago: i64 },
    
    /// 今月は活動なし（8〜30日）
    InactiveThisMonth { days_ago: i64 },
    
    /// 長期間活動なし（31日以上）
    LongInactive { days_ago: i64 },
}

impl ActivityState {
    /// アクティブかどうか（7日以内の活動）
    pub fn is_active(&self) -> bool {
        matches!(self, ActivityState::ActiveToday | ActivityState::ActiveThisWeek { .. })
    }
    
    /// Poke推奨かどうか（7日以上活動なし）
    pub fn should_poke(&self) -> bool {
        !self.is_active()
    }
}

/// フォロー関係
/// 
/// 送信者から見た受信者との関係を表現
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FollowRelation {
    /// フォロー関係なし
    None,
    
    /// フォロワー（受信者が送信者をフォロー）
    Follower,
    
    /// 相互フォロー
    Mutual,
}

impl FollowRelation {
    /// 送信者にフォロワー権限があるかどうか
    pub fn is_follower(&self) -> bool {
        match self {
            FollowRelation::Follower | FollowRelation::Mutual => true,
            FollowRelation::None => false,
        }
    }
    
    /// 相互フォローかどうか
    pub fn is_mutual(&self) -> bool {
        matches!(self, FollowRelation::Mutual)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};
    
    mod github_activity_tests {
        use super::*;
        
        #[test]
        fn test_days_since_last_activity_today() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: Some(Utc::now()),
                current_streak_days: Some(5),
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            assert_eq!(activity.days_since_last_activity(), 0);
        }
        
        #[test]
        fn test_days_since_last_activity_past() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: Some(Utc::now() - Duration::days(10)),
                current_streak_days: None,
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            assert_eq!(activity.days_since_last_activity(), 10);
        }
        
        #[test]
        fn test_days_since_last_activity_none() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: None,
                current_streak_days: None,
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            assert_eq!(activity.days_since_last_activity(), 365);
        }
        
        #[test]
        fn test_current_streak_days() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: Some(Utc::now()),
                current_streak_days: Some(42),
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            assert_eq!(activity.current_streak_days(), Some(42));
        }
        
        #[test]
        fn test_activity_state_active_today() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: Some(Utc::now()),
                current_streak_days: Some(5),
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            assert_eq!(activity.activity_state(), ActivityState::ActiveToday);
        }
        
        #[test]
        fn test_activity_state_active_this_week() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: Some(Utc::now() - Duration::days(3)),
                current_streak_days: None,
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            match activity.activity_state() {
                ActivityState::ActiveThisWeek { days_ago } => assert_eq!(days_ago, 3),
                _ => panic!("Expected ActiveThisWeek"),
            }
        }
        
        #[test]
        fn test_activity_state_inactive_this_month() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: Some(Utc::now() - Duration::days(15)),
                current_streak_days: None,
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            match activity.activity_state() {
                ActivityState::InactiveThisMonth { days_ago } => assert_eq!(days_ago, 15),
                _ => panic!("Expected InactiveThisMonth"),
            }
        }
        
        #[test]
        fn test_activity_state_long_inactive() {
            let activity = GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: Some(Utc::now() - Duration::days(60)),
                current_streak_days: None,
                contributions: None,
                total_contributions: None,
                fetched_at: Utc::now(),
            };
            
            match activity.activity_state() {
                ActivityState::LongInactive { days_ago } => assert_eq!(days_ago, 60),
                _ => panic!("Expected LongInactive"),
            }
        }
    }
    
    mod activity_state_tests {
        use super::*;
        
        #[test]
        fn test_is_active() {
            assert!(ActivityState::ActiveToday.is_active());
            assert!(ActivityState::ActiveThisWeek { days_ago: 3 }.is_active());
            assert!(!ActivityState::InactiveThisMonth { days_ago: 15 }.is_active());
            assert!(!ActivityState::LongInactive { days_ago: 60 }.is_active());
        }
        
        #[test]
        fn test_should_poke() {
            assert!(!ActivityState::ActiveToday.should_poke());
            assert!(!ActivityState::ActiveThisWeek { days_ago: 3 }.should_poke());
            assert!(ActivityState::InactiveThisMonth { days_ago: 15 }.should_poke());
            assert!(ActivityState::LongInactive { days_ago: 60 }.should_poke());
        }
    }
    
    mod follow_relation_tests {
        use super::*;
        
        #[test]
        fn test_is_follower() {
            assert!(!FollowRelation::None.is_follower());
            assert!(FollowRelation::Follower.is_follower());
            assert!(FollowRelation::Mutual.is_follower());
        }
        
        #[test]
        fn test_is_mutual() {
            assert!(!FollowRelation::None.is_mutual());
            assert!(!FollowRelation::Follower.is_mutual());
            assert!(FollowRelation::Mutual.is_mutual());
        }
    }
}