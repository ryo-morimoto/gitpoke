//! ユーザードメインモデル
//! 
//! このファイルは以下を定義：
//! - ユーザーに関する値オブジェクト
//! - ユーザーの状態表現
//! - ユーザー設定
//! - ユーザー関連のビジネスルール

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::validation::{Validated, ValidationError};

/// GitHubユーザーID
/// 
/// GitHubが割り当てる一意の数値ID
/// ユーザー名と違い、変更されることがない
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GitHubUserId(i64);

impl GitHubUserId {
    /// 新しいGitHubUserIdを作成
    /// 
    /// # Arguments
    /// * `id` - GitHub APIから取得したユーザーID
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    
    /// 内部の値を取得
    pub fn value(&self) -> i64 {
        self.0
    }
}

/// GitHubユーザー名
/// 
/// バリデーション済みのユーザー名
/// - 1〜39文字
/// - 英数字とハイフンのみ
/// - ハイフンで始まったり終わったりしない
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    /// 文字列からUsernameを解析
    /// 
    /// GitHubのユーザー名規則に従ってバリデーション
    /// 
    /// # Arguments
    /// * `s` - 検証するユーザー名
    /// 
    /// # Returns
    /// * `Ok(Username)` - 有効なユーザー名
    /// * `Err(ValidationError)` - 無効なユーザー名
    pub fn parse(s: String) -> Result<Self, ValidationError> {
        Username(s).validate()
    }
    
    /// 文字列からUsernameを作成（parseのエイリアス）
    /// 
    /// # Arguments
    /// * `s` - 検証するユーザー名
    /// 
    /// # Returns
    /// * `Ok(Username)` - 有効なユーザー名
    /// * `Err(ValidationError)` - 無効なユーザー名
    pub fn new(s: String) -> Result<Self, ValidationError> {
        Self::parse(s)
    }
    
    /// 内部の文字列を取得
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Validated for Username {
    type Error = ValidationError;
    
    /// GitHubユーザー名のバリデーション
    /// 
    /// 以下のルールを適用：
    /// - 1〜39文字
    /// - 英数字とハイフンのみ使用可能
    /// - ハイフンで始まったり終わったりしない
    /// - 連続するハイフンは不可
    fn validate(self) -> Result<Self, Self::Error> {
        crate::domain::validation::validate_github_username_format(&self.0)?;
        Ok(self)
    }
}

/// ユーザーの状態
/// 
/// GitPokeにおけるユーザーの登録状態を表現
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserState {
    /// 未登録ユーザー（GitHubユーザーだがGitPokeアカウントなし）
    Anonymous(Username),
    
    /// 登録済みユーザー（GitPokeアカウント保持）
    Registered(RegisteredUser),
}

impl UserState {
    /// ユーザー名を取得
    pub fn username(&self) -> &Username {
        match self {
            UserState::Anonymous(username) => username,
            UserState::Registered(user) => &user.username,
        }
    }
    
    /// 登録済みかどうかを判定
    pub fn is_registered(&self) -> bool {
        matches!(self, UserState::Registered(_))
    }
}

/// 登録済みユーザー
/// 
/// GitPokeアカウントを持つユーザーの情報
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisteredUser {
    /// GitHub ID（不変）
    pub github_id: GitHubUserId,
    
    /// GitHubユーザー名（変更可能）
    pub username: Username,
    
    /// Poke受信設定
    pub poke_setting: PokeSetting,
    
    /// アカウント作成日時
    pub created_at: DateTime<Utc>,
    
    /// 最終更新日時
    pub updated_at: DateTime<Utc>,
}

impl RegisteredUser {
    /// 新規ユーザーを作成
    /// 
    /// デフォルトでPoke受信は「全員から」に設定
    /// 
    /// # Arguments
    /// * `github_id` - GitHub ID
    /// * `username` - GitHubユーザー名
    pub fn new(github_id: GitHubUserId, username: Username) -> Self {
        let now = Utc::now();
        Self {
            github_id,
            username,
            poke_setting: PokeSetting::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Poke設定を更新
    /// 
    /// # Arguments
    /// * `setting` - 新しいPoke設定
    pub fn update_poke_setting(&mut self, setting: PokeSetting) {
        self.poke_setting = setting;
        self.updated_at = Utc::now();
    }
    
    /// ユーザー名を更新
    /// 
    /// GitHubでユーザー名が変更された場合に使用
    /// 
    /// # Arguments
    /// * `new_username` - 新しいユーザー名
    pub fn update_username(&mut self, new_username: Username) {
        self.username = new_username;
        self.updated_at = Utc::now();
    }
}

/// Poke受信設定
/// 
/// どのユーザーからPokeを受け取るかの設定
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PokeSetting {
    /// 全員から受信
    Anyone,
    
    /// フォロワーのみから受信
    FollowersOnly,
    
    /// 相互フォローのみから受信
    MutualOnly,
    
    /// Poke受信を無効化
    Disabled,
}

impl Default for PokeSetting {
    /// デフォルトは全員から受信
    fn default() -> Self {
        PokeSetting::Anyone
    }
}

impl PokeSetting {
    /// 設定が有効かどうか
    pub fn is_enabled(&self) -> bool {
        !matches!(self, PokeSetting::Disabled)
    }
    
    /// フォロワーからのPokeが許可されているか
    pub fn allows_follower(&self) -> bool {
        matches!(self, PokeSetting::Anyone | PokeSetting::FollowersOnly | PokeSetting::MutualOnly)
    }
    
    /// 相互フォローが必要かどうか
    pub fn requires_mutual(&self) -> bool {
        matches!(self, PokeSetting::MutualOnly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod username_tests {
        use super::*;

        #[test]
        fn test_valid_usernames() {
            // 正常系：有効なユーザー名
            let valid_usernames = vec![
                "octocat",
                "test-user",
                "user123",
                "a",  // 1文字
                "a".repeat(39),  // 39文字（最大長）
                "test-user-123",
                "ABC123",
            ];

            for username in valid_usernames {
                let result = Username::parse(username.clone());
                assert!(result.is_ok(), "Failed for username: {}", username);
                assert_eq!(result.unwrap().as_str(), username);
            }
        }

        #[test]
        fn test_invalid_usernames() {
            // 異常系：無効なユーザー名
            let invalid_usernames = vec![
                "",  // 空文字
                "-test",  // ハイフンで開始
                "test-",  // ハイフンで終了
                "test--user",  // 連続ハイフン
                "test user",  // スペース
                "test@user",  // 特殊文字
                "test.user",  // ドット
                "a".repeat(40),  // 40文字（長すぎる）
            ];

            for username in invalid_usernames {
                let result = Username::parse(username.clone());
                assert!(result.is_err(), "Should fail for username: {}", username);
            }
        }

        #[test]
        fn test_username_equality() {
            let username1 = Username::parse("octocat".to_string()).unwrap();
            let username2 = Username::parse("octocat".to_string()).unwrap();
            let username3 = Username::parse("different".to_string()).unwrap();

            assert_eq!(username1, username2);
            assert_ne!(username1, username3);
        }
    }

    mod github_user_id_tests {
        use super::*;

        #[test]
        fn test_github_user_id_creation() {
            let id = GitHubUserId::new(12345);
            assert_eq!(id.value(), 12345);
        }

        #[test]
        fn test_github_user_id_equality() {
            let id1 = GitHubUserId::new(12345);
            let id2 = GitHubUserId::new(12345);
            let id3 = GitHubUserId::new(67890);

            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
        }
    }

    mod user_state_tests {
        use super::*;

        #[test]
        fn test_anonymous_user_state() {
            let username = Username::parse("octocat".to_string()).unwrap();
            let state = UserState::Anonymous(username.clone());

            assert_eq!(state.username().as_str(), "octocat");
            assert!(!state.is_registered());
        }

        #[test]
        fn test_registered_user_state() {
            let username = Username::parse("octocat".to_string()).unwrap();
            let user = RegisteredUser::new(GitHubUserId::new(12345), username);
            let state = UserState::Registered(user);

            assert_eq!(state.username().as_str(), "octocat");
            assert!(state.is_registered());
        }
    }

    mod registered_user_tests {
        use super::*;

        #[test]
        fn test_new_registered_user() {
            let username = Username::parse("octocat".to_string()).unwrap();
            let user = RegisteredUser::new(GitHubUserId::new(12345), username);

            assert_eq!(user.github_id.value(), 12345);
            assert_eq!(user.username.as_str(), "octocat");
            assert_eq!(user.poke_setting, PokeSetting::Anyone);
            assert_eq!(user.created_at, user.updated_at);
        }

        #[test]
        fn test_update_poke_setting() {
            let username = Username::parse("octocat".to_string()).unwrap();
            let mut user = RegisteredUser::new(GitHubUserId::new(12345), username);
            let original_updated_at = user.updated_at;

            // 少し待機してタイムスタンプが変わることを保証
            std::thread::sleep(std::time::Duration::from_millis(10));

            user.update_poke_setting(PokeSetting::FollowersOnly);

            assert_eq!(user.poke_setting, PokeSetting::FollowersOnly);
            assert!(user.updated_at > original_updated_at);
        }

        #[test]
        fn test_update_username() {
            let username = Username::parse("octocat".to_string()).unwrap();
            let mut user = RegisteredUser::new(GitHubUserId::new(12345), username);
            let original_updated_at = user.updated_at;

            // 少し待機してタイムスタンプが変わることを保証
            std::thread::sleep(std::time::Duration::from_millis(10));

            let new_username = Username::parse("new-octocat".to_string()).unwrap();
            user.update_username(new_username);

            assert_eq!(user.username.as_str(), "new-octocat");
            assert!(user.updated_at > original_updated_at);
        }
    }

    mod poke_setting_tests {
        use super::*;

        #[test]
        fn test_default_poke_setting() {
            assert_eq!(PokeSetting::default(), PokeSetting::Anyone);
        }

        #[test]
        fn test_is_enabled() {
            assert!(PokeSetting::Anyone.is_enabled());
            assert!(PokeSetting::FollowersOnly.is_enabled());
            assert!(PokeSetting::MutualOnly.is_enabled());
            assert!(!PokeSetting::Disabled.is_enabled());
        }

        #[test]
        fn test_allows_follower() {
            assert!(PokeSetting::Anyone.allows_follower());
            assert!(PokeSetting::FollowersOnly.allows_follower());
            assert!(PokeSetting::MutualOnly.allows_follower());
            assert!(!PokeSetting::Disabled.allows_follower());
        }

        #[test]
        fn test_requires_mutual() {
            assert!(!PokeSetting::Anyone.requires_mutual());
            assert!(!PokeSetting::FollowersOnly.requires_mutual());
            assert!(PokeSetting::MutualOnly.requires_mutual());
            assert!(!PokeSetting::Disabled.requires_mutual());
        }
    }
}