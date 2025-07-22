//! アプリケーション設定
//! 
//! このファイルは以下を定義：
//! - 環境変数からの設定読み込み
//! - 設定のバリデーション
//! - デフォルト値の提供

use serde::{Deserialize, Serialize};
use crate::error::AppResult;

/// アプリケーション設定
/// 
/// 環境変数から読み込まれる設定値
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// サーバーポート
    pub port: u16,
    
    /// GitHub App設定
    pub github: GitHubConfig,
    
    /// Redis設定
    pub redis: RedisConfig,
    
    /// Firestore設定
    pub firestore: FirestoreConfig,
    
    /// Cloud Storage設定
    pub storage: StorageConfig,
    
    /// アプリケーション設定
    pub app: AppConfig,
}

impl Config {
    /// 環境変数から設定を読み込む
    /// 
    /// 以下の環境変数を読み込む：
    /// - PORT: サーバーポート（デフォルト: 8080）
    /// - GITHUB_APP_ID: GitHub App ID（必須）
    /// - GITHUB_APP_PRIVATE_KEY: GitHub App秘密鍵（必須）
    /// - GITHUB_WEBHOOK_SECRET: Webhookシークレット（オプション）
    /// - REDIS_URL: Redis接続URL（必須）
    /// - FIRESTORE_PROJECT_ID: GCPプロジェクトID（必須）
    /// - STORAGE_BUCKET: Cloud Storageバケット名（必須）
    /// - ENVIRONMENT: 実行環境（development/staging/production）
    /// 
    /// # Returns
    /// * `Ok(Config)` - 読み込み成功
    /// * `Err(AppError)` - 必須環境変数が不足
    pub fn from_env() -> AppResult<Self> {
        // dotenvファイルがあれば読み込む（開発環境用）
        dotenvy::dotenv().ok();
        
        // TODO: 実装
        // - 各環境変数を読み込む
        // - 必須項目のバリデーション
        // - デフォルト値の適用
        unimplemented!()
    }
    
    /// 設定の妥当性を検証
    /// 
    /// # Returns
    /// * `Ok(())` - 検証成功
    /// * `Err(AppError)` - 無効な設定値
    pub fn validate(&self) -> AppResult<()> {
        // TODO: 実装
        // - ポート番号の範囲チェック
        // - URLの形式チェック
        // - ファイルパスの存在確認（秘密鍵など）
        unimplemented!()
    }
}

/// GitHub関連の設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// GitHub App ID
    pub app_id: u64,
    
    /// GitHub App秘密鍵のパスまたは内容
    /// 環境変数には改行を含むPEM形式で設定
    pub app_private_key: String,
    
    /// Webhookシークレット（オプション）
    /// GitHub Appの設定でWebhookを有効にした場合に必要
    pub webhook_secret: Option<String>,
    
    /// GitHub APIのベースURL
    /// デフォルト: https://api.github.com
    pub api_base_url: String,
    
    /// GraphQL APIのURL
    /// デフォルト: https://api.github.com/graphql
    pub graphql_url: String,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            app_id: 0,
            app_private_key: String::new(),
            webhook_secret: None,
            api_base_url: "https://api.github.com".to_string(),
            graphql_url: "https://api.github.com/graphql".to_string(),
        }
    }
}

/// Redis設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis接続URL
    /// 形式: redis://[[username:]password@]host[:port][/database]
    /// 例: redis://localhost:6379/0
    pub url: String,
    
    /// 接続プールサイズ
    /// デフォルト: 10
    pub pool_size: u32,
    
    /// 接続タイムアウト（秒）
    /// デフォルト: 5
    pub connection_timeout: u64,
    
    /// コマンドタイムアウト（秒）
    /// デフォルト: 10
    pub command_timeout: u64,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379/0".to_string(),
            pool_size: 10,
            connection_timeout: 5,
            command_timeout: 10,
        }
    }
}

/// Firestore設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirestoreConfig {
    /// GCPプロジェクトID
    pub project_id: String,
    
    /// サービスアカウントキーのパス（オプション）
    /// Cloud Run環境では不要（デフォルト認証を使用）
    pub service_account_key_path: Option<String>,
    
    /// データベースID
    /// デフォルト: "(default)"
    pub database_id: String,
}

impl Default for FirestoreConfig {
    fn default() -> Self {
        Self {
            project_id: String::new(),
            service_account_key_path: None,
            database_id: "(default)".to_string(),
        }
    }
}

/// Cloud Storage設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// バケット名
    pub bucket_name: String,
    
    /// バッジ保存用のプレフィックス
    /// デフォルト: "badges/"
    pub badge_prefix: String,
    
    /// CDNのベースURL（オプション）
    /// 設定されている場合、バッジURLにCDNを使用
    pub cdn_base_url: Option<String>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            bucket_name: String::new(),
            badge_prefix: "badges/".to_string(),
            cdn_base_url: None,
        }
    }
}

/// アプリケーション動作設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 実行環境
    pub environment: Environment,
    
    /// ログレベル
    /// デフォルト: debug（development）、info（production）
    pub log_level: String,
    
    /// レート制限設定
    pub rate_limit: RateLimitConfig,
    
    /// キャッシュ設定
    pub cache: CacheConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Development,
            log_level: "debug".to_string(),
            rate_limit: RateLimitConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

/// 実行環境
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    /// 開発環境
    Development,
    /// ステージング環境
    Staging,
    /// 本番環境
    Production,
}

impl Environment {
    /// 文字列から環境を解析
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "production" | "prod" => Environment::Production,
            "staging" | "stage" => Environment::Staging,
            _ => Environment::Development,
        }
    }
    
    /// 本番環境かどうか
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

/// レート制限設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// IPアドレスごとのPoke制限（回/分）
    pub poke_per_ip_per_minute: u32,
    
    /// ユーザーごとの同一ターゲットへのPoke制限（回/日）
    pub poke_per_user_per_day: u32,
    
    /// IPアドレスごとのバッジ生成制限（回/分）
    pub badge_per_ip_per_minute: u32,
    
    /// GitHub APIの共有レート制限（回/時）
    pub github_api_per_hour: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            poke_per_ip_per_minute: 10,
            poke_per_user_per_day: 1,
            badge_per_ip_per_minute: 100,
            github_api_per_hour: 5000,
        }
    }
}

/// キャッシュ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// アクティブユーザーのキャッシュTTL（秒）
    pub active_user_ttl: u64,
    
    /// 非アクティブユーザーのキャッシュTTL（秒）
    pub inactive_user_ttl: u64,
    
    /// GitHub APIレスポンスのキャッシュTTL（秒）
    pub github_api_ttl: u64,
    
    /// バッジSVGのキャッシュTTL（秒）
    pub badge_svg_ttl: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            active_user_ttl: 300,      // 5分
            inactive_user_ttl: 3600,   // 1時間
            github_api_ttl: 300,       // 5分
            badge_svg_ttl: 300,        // 5分
        }
    }
}