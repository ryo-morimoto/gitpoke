//! HTTPハンドラー層
//! 
//! このモジュールは以下を含む：
//! - HTTPリクエスト/レスポンスの処理
//! - 認証・認可の確認
//! - ユースケース層の呼び出し
//! - レスポンスの構築

pub mod auth;
pub mod badge;
pub mod health;
pub mod poke;
pub mod user;

// 共通のハンドラーユーティリティ
pub mod utils;