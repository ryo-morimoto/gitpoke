//! インフラストラクチャ層
//! 
//! このモジュールは以下を含む：
//! - 外部サービスとの通信
//! - データの永続化
//! - キャッシュ管理
//! - 通知サービス

pub mod adapters;
pub mod cache;
pub mod repositories;

// TODO: Migrate existing modules
pub mod github_api;
pub mod user_repository;
pub mod event_store;
pub mod cache_service;
pub mod notification_service;