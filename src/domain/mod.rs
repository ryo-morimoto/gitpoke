//! ドメイン層のモジュール定義
//! 
//! このモジュールは以下を含む：
//! - ビジネスロジックの中核となる型定義
//! - 値オブジェクト（Value Objects）
//! - エンティティ（Entities）
//! - ドメインイベント
//! - ビジネスルールの実装

pub mod user;
pub mod poke;
pub mod badge;
pub mod github;
pub mod validation;

// 主要な型を再エクスポート
pub use user::{Username, GitHubUserId, UserState, RegisteredUser, PokeSetting};
pub use poke::{PokeCapability, PokeEvent, PokeResult};
pub use badge::{BadgeState, BadgeSvg};
pub use github::{GitHubActivity, FollowRelation, ActivityState};
pub use validation::{Validated, ValidationError};