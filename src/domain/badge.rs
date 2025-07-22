//! バッジ生成のドメインモデル
//! 
//! このファイルは以下を定義：
//! - バッジの状態表現
//! - バッジSVG生成ロジック
//! - バッジに関するビジネスルール

use serde::{Deserialize, Serialize};
use crate::domain::github::GitHubActivity;
use crate::domain::user::UserState;

/// バッジの状態
/// 
/// ユーザーのアクティビティ状態とインタラクション可能性を表現
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeState {
    /// アクティブ状態（7日以内に活動あり）
    Active {
        /// 最後の活動からの経過日数
        days_since_last_activity: i64,
        /// 連続活動日数（streak）
        streak_days: Option<i64>,
    },
    
    /// 非アクティブ状態（7日以上活動なし）
    Inactive {
        /// 最後の活動からの経過日数
        days_since_last_activity: i64,
        /// Pokeが可能かどうか
        pokeable: bool,
    },
    
    /// ユーザーが見つからない
    NotFound,
}

impl BadgeState {
    /// GitHubActivityとUserStateからBadgeStateを判定
    /// 
    /// # Arguments
    /// * `activity` - GitHubのアクティビティ情報
    /// * `user_state` - ユーザーの登録状態
    /// 
    /// # Returns
    /// * `BadgeState` - 判定されたバッジ状態
    pub fn from_activity(activity: &GitHubActivity, user_state: &UserState) -> Self {
        let days = activity.days_since_last_activity();
        
        if days <= 7 {
            // アクティブ状態
            BadgeState::Active {
                days_since_last_activity: days,
                streak_days: activity.current_streak_days(),
            }
        } else {
            // 非アクティブ状態
            // 登録済みユーザーのみPoke可能
            let pokeable = user_state.is_registered();
            BadgeState::Inactive {
                days_since_last_activity: days,
                pokeable,
            }
        }
    }
    
    /// バッジの色を取得
    /// 
    /// # Returns
    /// * 緑（#44cc11） - アクティブ
    /// * 赤（#e05d44） - 非アクティブ
    pub fn color(&self) -> &'static str {
        match self {
            BadgeState::Active { .. } => "#44cc11", // 緑
            BadgeState::Inactive { .. } => "#e05d44", // 赤
            BadgeState::NotFound => "#9f9f9f", // グレー
        }
    }
    
    /// バッジのテキストを取得
    pub fn text(&self) -> String {
        match self {
            BadgeState::Active { days_since_last_activity, .. } => {
                if *days_since_last_activity == 0 {
                    "Active today".to_string()
                } else {
                    format!("Active {} days ago", days_since_last_activity)
                }
            }
            BadgeState::Inactive { days_since_last_activity, .. } => {
                format!("Inactive for {} days", days_since_last_activity)
            }
            BadgeState::NotFound => "User not found".to_string(),
        }
    }
}

/// バッジSVG
/// 
/// 生成されたSVGコンテンツとメタデータ
#[derive(Debug, Clone)]
pub struct BadgeSvg {
    /// SVGコンテンツ
    pub content: String,
    
    /// Cache-Controlヘッダー用のTTL（秒）
    pub cache_ttl: u64,
    
    /// インタラクティブ要素を含むかどうか
    pub is_interactive: bool,
}

impl BadgeSvg {
    /// 静的バッジを生成
    /// 
    /// シンプルなSVGバッジ（クリック不可）
    /// 
    /// # Arguments
    /// * `state` - バッジの状態
    /// * `username` - ユーザー名
    pub fn static_badge(state: &BadgeState, username: &str) -> Self {
        let color = state.color();
        let text = state.text();
        
        // SVGテンプレート
        // TODO: 実際のSVG生成ロジックを実装
        // - テキスト幅に基づいてSVG全体の幅を動的に計算（font-family: Arial, font-size: 12px）
        // - 左右のパディング（各10px）を考慮した配置
        // - shields.io風のグラデーションとシャドウ効果を追加
        let content = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="20">
                <rect width="200" height="20" fill="{}"/>
                <text x="10" y="14" fill="white" font-family="Arial" font-size="12">
                    GitPoke: {}
                </text>
            </svg>"#,
            color, text
        );
        
        // キャッシュTTLの決定
        let cache_ttl = match state {
            BadgeState::Active { .. } => 300, // 5分
            BadgeState::Inactive { .. } => 3600, // 1時間
            BadgeState::NotFound => 86400, // 24時間
        };
        
        Self {
            content,
            cache_ttl,
            is_interactive: false,
        }
    }
    
    /// インタラクティブバッジを生成
    /// 
    /// クリック可能なバッジ（JavaScriptを含む）
    /// 
    /// # Arguments
    /// * `state` - バッジの状態
    /// * `username` - ユーザー名
    pub fn interactive_badge(state: &BadgeState, username: &str) -> Self {
        // 基本的な静的バッジを生成
        let mut badge = Self::static_badge(state, username);
        
        // インタラクティブ要素を追加
        if let BadgeState::Inactive { pokeable: true, .. } = state {
            // TODO: クリック可能な要素とJavaScriptを追加
            // - SVG全体にonclickハンドラーを設定（<svg onclick="...">）
            // - fetch APIを使用してPOST /api/pokeを実行（CORS対応）
            // - クリック時の視覚的フィードバック（一時的に色を変更）
            // - エラー時はconsole.errorに出力（セキュリティ上アラートは避ける）
            badge.is_interactive = true;
        }
        
        badge
    }
    
    /// Content-Typeヘッダーを取得
    pub fn content_type(&self) -> &'static str {
        "image/svg+xml"
    }
    
    /// Cache-Controlヘッダーを取得
    pub fn cache_control(&self) -> String {
        format!(
            "public, max-age={}, stale-while-revalidate=86400",
            self.cache_ttl
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::{Username, RegisteredUser, GitHubUserId};
    use chrono::{DateTime, Utc, Duration};
    
    mod badge_state_tests {
        use super::*;
        
        fn create_activity(days_since_last: i64, streak_days: Option<i64>) -> GitHubActivity {
            let last_activity = if days_since_last == 0 {
                Some(Utc::now())
            } else {
                Some(Utc::now() - Duration::days(days_since_last))
            };
            
            GitHubActivity {
                username: "testuser".to_string(),
                last_activity_at: last_activity,
                current_streak_days: streak_days,
            }
        }
        
        #[test]
        fn test_active_state_today() {
            let activity = create_activity(0, Some(10));
            let user_state = UserState::Anonymous(Username::new("testuser".to_string()).unwrap());
            
            let badge_state = BadgeState::from_activity(&activity, &user_state);
            
            match badge_state {
                BadgeState::Active { days_since_last_activity, streak_days } => {
                    assert_eq!(days_since_last_activity, 0);
                    assert_eq!(streak_days, Some(10));
                }
                _ => panic!("Expected Active state"),
            }
        }
        
        #[test]
        fn test_active_state_within_week() {
            let activity = create_activity(5, None);
            let user_state = UserState::Anonymous(Username::new("testuser".to_string()).unwrap());
            
            let badge_state = BadgeState::from_activity(&activity, &user_state);
            
            match badge_state {
                BadgeState::Active { days_since_last_activity, streak_days } => {
                    assert_eq!(days_since_last_activity, 5);
                    assert_eq!(streak_days, None);
                }
                _ => panic!("Expected Active state"),
            }
        }
        
        #[test]
        fn test_inactive_state_registered_user() {
            let activity = create_activity(10, None);
            let username = Username::new("testuser".to_string()).unwrap();
            let user = RegisteredUser::new(GitHubUserId::new(12345), username);
            let user_state = UserState::Registered(user);
            
            let badge_state = BadgeState::from_activity(&activity, &user_state);
            
            match badge_state {
                BadgeState::Inactive { days_since_last_activity, pokeable } => {
                    assert_eq!(days_since_last_activity, 10);
                    assert!(pokeable); // 登録済みユーザーはPoke可能
                }
                _ => panic!("Expected Inactive state"),
            }
        }
        
        #[test]
        fn test_inactive_state_anonymous_user() {
            let activity = create_activity(10, None);
            let user_state = UserState::Anonymous(Username::new("testuser".to_string()).unwrap());
            
            let badge_state = BadgeState::from_activity(&activity, &user_state);
            
            match badge_state {
                BadgeState::Inactive { days_since_last_activity, pokeable } => {
                    assert_eq!(days_since_last_activity, 10);
                    assert!(!pokeable); // 未登録ユーザーはPoke不可
                }
                _ => panic!("Expected Inactive state"),
            }
        }
        
        #[test]
        fn test_badge_colors() {
            let active = BadgeState::Active { days_since_last_activity: 0, streak_days: None };
            let inactive = BadgeState::Inactive { days_since_last_activity: 10, pokeable: true };
            let not_found = BadgeState::NotFound;
            
            assert_eq!(active.color(), "#44cc11");
            assert_eq!(inactive.color(), "#e05d44");
            assert_eq!(not_found.color(), "#9f9f9f");
        }
        
        #[test]
        fn test_badge_text() {
            // 今日アクティブ
            let active_today = BadgeState::Active { days_since_last_activity: 0, streak_days: None };
            assert_eq!(active_today.text(), "Active today");
            
            // 数日前にアクティブ
            let active_days_ago = BadgeState::Active { days_since_last_activity: 3, streak_days: None };
            assert_eq!(active_days_ago.text(), "Active 3 days ago");
            
            // 非アクティブ
            let inactive = BadgeState::Inactive { days_since_last_activity: 10, pokeable: true };
            assert_eq!(inactive.text(), "Inactive for 10 days");
            
            // ユーザーが見つからない
            let not_found = BadgeState::NotFound;
            assert_eq!(not_found.text(), "User not found");
        }
    }
    
    mod badge_svg_tests {
        use super::*;
        
        #[test]
        fn test_static_badge_generation() {
            let state = BadgeState::Active { days_since_last_activity: 0, streak_days: Some(5) };
            let badge = BadgeSvg::static_badge(&state, "testuser");
            
            assert!(!badge.is_interactive);
            assert_eq!(badge.cache_ttl, 300); // アクティブ状態は5分
            assert!(badge.content.contains("#44cc11")); // 緑色
            assert!(badge.content.contains("Active today"));
        }
        
        #[test]
        fn test_interactive_badge_when_pokeable() {
            let state = BadgeState::Inactive { days_since_last_activity: 10, pokeable: true };
            let badge = BadgeSvg::interactive_badge(&state, "testuser");
            
            assert!(badge.is_interactive);
            assert_eq!(badge.cache_ttl, 3600); // 非アクティブ状態は1時間
        }
        
        #[test]
        fn test_non_interactive_badge_when_not_pokeable() {
            let state = BadgeState::Inactive { days_since_last_activity: 10, pokeable: false };
            let badge = BadgeSvg::interactive_badge(&state, "testuser");
            
            assert!(!badge.is_interactive); // Poke不可の場合はインタラクティブでない
        }
        
        #[test]
        fn test_cache_ttl() {
            let active = BadgeState::Active { days_since_last_activity: 0, streak_days: None };
            let inactive = BadgeState::Inactive { days_since_last_activity: 10, pokeable: true };
            let not_found = BadgeState::NotFound;
            
            let badge_active = BadgeSvg::static_badge(&active, "user");
            let badge_inactive = BadgeSvg::static_badge(&inactive, "user");
            let badge_not_found = BadgeSvg::static_badge(&not_found, "user");
            
            assert_eq!(badge_active.cache_ttl, 300); // 5分
            assert_eq!(badge_inactive.cache_ttl, 3600); // 1時間
            assert_eq!(badge_not_found.cache_ttl, 86400); // 24時間
        }
        
        #[test]
        fn test_content_type() {
            let state = BadgeState::Active { days_since_last_activity: 0, streak_days: None };
            let badge = BadgeSvg::static_badge(&state, "testuser");
            
            assert_eq!(badge.content_type(), "image/svg+xml");
        }
        
        #[test]
        fn test_cache_control_header() {
            let state = BadgeState::Active { days_since_last_activity: 0, streak_days: None };
            let badge = BadgeSvg::static_badge(&state, "testuser");
            
            let cache_control = badge.cache_control();
            assert!(cache_control.contains("public"));
            assert!(cache_control.contains("max-age=300"));
            assert!(cache_control.contains("stale-while-revalidate=86400"));
        }
    }
}