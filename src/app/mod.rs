//! アプリケーション設定と依存性管理
//! 
//! このモジュールは以下を含む：
//! - アプリケーション設定の管理
//! - 依存性注入コンテナ
//! - アプリケーション全体の初期化

pub mod config;
pub mod dependencies;
pub mod services;

pub use config::Config;
pub use dependencies::AppDependencies;