# GitPoke Rust実装ベストプラクティス

## 命名規則

### ファイル名

#### インフラ層の命名規則

| パターン | ファイル名 | 説明 |
|---------|-----------|------|
| Repository | `*_repository.rs` | データ永続化の抽象化 |
| API Client | `*_api.rs` | 外部APIクライアント |
| Service | `*_service.rs` | 外部サービス連携 |
| Store | `*_store.rs` | イベントストア等 |

```
infra/
├── user_repository.rs     # UserRepository traitの実装
├── github_api.rs          # GitHub APIクライアント
├── event_store.rs         # イベントの永続化
├── cache_service.rs       # Redisキャッシュサービス
└── notification_service.rs # 通知サービス
```

#### ドメイン層の命名規則

| 種別 | ファイル名 | 説明 |
|------|-----------|------|
| 値オブジェクト | `*.rs` | 単数形（user.rs, poke.rs） |
| 集約 | `*s.rs` | 複数形（validations.rs） |

#### ユースケース層の命名規則

動詞から始まるアクション名：
- `check_poke.rs` - Poke可否をチェック
- `generate_badge.rs` - バッジを生成
- `update_settings.rs` - 設定を更新

### 関数名

```rust
// Repositoryメソッド
async fn find(&self, id: &Id) -> Result<Option<T>>;
async fn find_by_username(&self, username: &Username) -> Result<Option<User>>;
async fn save(&self, entity: &T) -> Result<()>;
async fn delete(&self, id: &Id) -> Result<()>;

// ビジネスロジック（純粋関数）
fn validate_*()     // 検証
fn calculate_*()    // 計算
fn determine_*()    // 判定
fn check_*()        // チェック
fn generate_*()     // 生成
```

### 型名

```rust
// 値オブジェクト（単数形）
struct Username(String);
struct GitHubUserId(i64);

// エンティティ（単数形）
struct User { ... }
struct PokeEvent { ... }

// コレクション（複数形）
struct Users(Vec<User>);
struct PokeEvents(Vec<PokeEvent>);

// 状態を表すenum
enum UserState { ... }
enum ActivityState { ... }

// エラー型
enum DomainError { ... }
enum ValidationError { ... }
```

## エラーハンドリング

### 基本原則
- `Result<T, E>`を積極的に使用し、`unwrap()`は本当に失敗しない場合のみ使用
- エラーの文脈を追加する場合は`anyhow::Context`を活用
- 定義済みの`AppError`型を活用してエラーを分類

### 実装例
```rust
use crate::error::{AppError, AppResult};
use anyhow::Context;

async fn fetch_user_data(username: &str) -> AppResult<UserData> {
    // カスタムエラー型を返す
    validate_username(username)?;
    
    // 文脈を追加してエラーを伝播
    let data = fetch_from_api(username)
        .await
        .context("Failed to fetch user data from GitHub API")?;
    
    Ok(data)
}
```

## 非同期プログラミング

### 並行実行の活用
```rust
// Good: 並行実行で効率化
use tokio::join;

async fn fetch_user_info(username: &str) -> AppResult<UserInfo> {
    let (user, repos, contributions) = join!(
        fetch_user(username),
        fetch_repositories(username),
        fetch_contributions(username)
    );
    
    Ok(UserInfo {
        user: user?,
        repos: repos?,
        contributions: contributions?,
    })
}

// Avoid: 逐次実行
async fn fetch_user_info_slow(username: &str) -> AppResult<UserInfo> {
    let user = fetch_user(username).await?;
    let repos = fetch_repositories(username).await?;
    let contributions = fetch_contributions(username).await?;
    // ...
}
```

### タイムアウトの設定
```rust
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout(username: &str) -> AppResult<Data> {
    timeout(Duration::from_secs(10), fetch_data(username))
        .await
        .map_err(|_| AppError::Timeout)?
}
```

## 依存性注入とテスタビリティ

### トレイトベースの抽象化
```rust
use async_trait::async_trait;

#[async_trait]
pub trait GitHubClient: Send + Sync {
    async fn get_contributions(&self, username: &str) -> AppResult<Contributions>;
    async fn get_user(&self, username: &str) -> AppResult<User>;
}

// 本番実装
pub struct OctocrabClient {
    client: octocrab::Octocrab,
}

#[async_trait]
impl GitHubClient for OctocrabClient {
    async fn get_contributions(&self, username: &str) -> AppResult<Contributions> {
        // 実装
    }
}

// テスト用モック
#[cfg(test)]
pub struct MockGitHubClient {
    // モックデータ
}
```

### サービス層の設計
```rust
pub struct BadgeService {
    github: Arc<dyn GitHubClient>,
    cache: Arc<dyn CacheClient>,
}

impl BadgeService {
    pub fn new(github: Arc<dyn GitHubClient>, cache: Arc<dyn CacheClient>) -> Self {
        Self { github, cache }
    }
}
```

## キャッシュ戦略

### Cache-Aside Pattern
```rust
use redis::AsyncCommands;

pub async fn get_badge_cached(
    cache: &mut redis::aio::MultiplexedConnection,
    github: &dyn GitHubClient,
    username: &str,
) -> AppResult<String> {
    let cache_key = format!("badge:{}", username);
    
    // 1. キャッシュを確認
    if let Ok(cached) = cache.get::<_, String>(&cache_key).await {
        tracing::debug!("Cache hit for user: {}", username);
        return Ok(cached);
    }
    
    // 2. データ取得
    let contributions = github.get_contributions(username).await?;
    let badge = generate_badge(&contributions)?;
    
    // 3. キャッシュに保存（エラーは無視）
    let ttl = calculate_ttl(&contributions);
    let _ = cache.set_ex(&cache_key, &badge, ttl).await;
    
    Ok(badge)
}

fn calculate_ttl(contributions: &Contributions) -> u64 {
    if contributions.is_active_within_days(7) {
        300  // 5分（アクティブユーザー）
    } else {
        3600 // 1時間（非アクティブユーザー）
    }
}
```

## 構造化ロギング

### tracingを使った実装
```rust
use tracing::{info, warn, error, instrument};

#[instrument(
    name = "fetch_github_activity",
    skip(client),
    fields(
        username = %username,
        request_id = %uuid::Uuid::new_v4()
    )
)]
pub async fn fetch_activity(
    client: &dyn GitHubClient,
    username: &str,
) -> AppResult<Activity> {
    info!("Starting activity fetch");
    
    match client.get_contributions(username).await {
        Ok(data) => {
            info!(
                contributions_count = data.total_contributions,
                "Successfully fetched activity"
            );
            Ok(Activity::from(data))
        }
        Err(e) => {
            error!(error = ?e, "Failed to fetch activity");
            Err(e)
        }
    }
}
```

## GitHub API統合

### GraphQL クエリの実装
```rust
const CONTRIBUTIONS_QUERY: &str = r#"
query($username: String!) {
    user(login: $username) {
        contributionsCollection {
            contributionCalendar {
                totalContributions
                weeks {
                    contributionDays {
                        contributionCount
                        date
                    }
                }
            }
        }
    }
}
"#;

pub async fn fetch_contributions_graphql(
    client: &octocrab::Octocrab,
    username: &str,
) -> AppResult<ContributionData> {
    let response: serde_json::Value = client
        .graphql(&serde_json::json!({
            "query": CONTRIBUTIONS_QUERY,
            "variables": {
                "username": username
            }
        }))
        .await?;
    
    // レスポンスをパース
    serde_json::from_value(response)
        .map_err(|e| AppError::Internal)
}
```

### レート制限の監視
```rust
pub struct RateLimitMiddleware;

impl RateLimitMiddleware {
    pub async fn check_rate_limit(response: &reqwest::Response) -> AppResult<()> {
        if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
            let remaining: u32 = remaining
                .to_str()?
                .parse()?;
            
            if remaining < 100 {
                warn!("GitHub API rate limit low: {} requests remaining", remaining);
            }
            
            if remaining == 0 {
                if let Some(reset) = response.headers().get("x-ratelimit-reset") {
                    let reset_time: u64 = reset.to_str()?.parse()?;
                    return Err(AppError::RateLimit { retry_after: reset_time });
                }
            }
        }
        Ok(())
    }
}
```

## パフォーマンス最適化

### 文字列処理
```rust
use std::borrow::Cow;

// 不要なアロケーションを避ける
pub fn format_badge_text<'a>(username: &'a str, days: i64) -> Cow<'a, str> {
    if days <= 7 {
        Cow::Borrowed("Active")
    } else {
        Cow::Owned(format!("Inactive for {} days", days))
    }
}
```

### バッチ処理の並行化
```rust
use futures::future::try_join_all;
use futures::stream::{self, StreamExt};

pub async fn generate_badges_batch(
    usernames: Vec<String>,
    concurrency: usize,
) -> AppResult<Vec<Badge>> {
    let badges = stream::iter(usernames)
        .map(|username| generate_badge_async(username))
        .buffer_unordered(concurrency)
        .try_collect()
        .await?;
    
    Ok(badges)
}
```

### 接続プーリング
```rust
// Redis接続プール
pub struct RedisPool {
    pool: deadpool_redis::Pool,
}

impl RedisPool {
    pub async fn get(&self) -> AppResult<deadpool_redis::Connection> {
        self.pool
            .get()
            .await
            .map_err(|e| AppError::Cache(e.into()))
    }
}
```

## セキュリティ

### 入力検証
```rust
use once_cell::sync::Lazy;
use regex::Regex;

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,37}[a-zA-Z0-9])?$").unwrap()
});

pub fn validate_username(username: &str) -> AppResult<&str> {
    if USERNAME_REGEX.is_match(username) {
        Ok(username)
    } else {
        Err(AppError::BadRequest("Invalid GitHub username format".into()))
    }
}
```

### CORS設定
```rust
use tower_http::cors::{CorsLayer, Any};
use http::header;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("https://github.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600))
}
```

### レート制限の実装
```rust
use std::sync::Arc;
use dashmap::DashMap;
use tokio::time::{Duration, Instant};

pub struct RateLimiter {
    requests: Arc<DashMap<String, Vec<Instant>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn check(&self, key: &str) -> AppResult<()> {
        let now = Instant::now();
        let mut requests = self.requests.entry(key.to_string()).or_default();
        
        // 古いリクエストを削除
        requests.retain(|&instant| now.duration_since(instant) < self.window);
        
        if requests.len() >= self.max_requests {
            return Err(AppError::RateLimit { 
                retry_after: self.window.as_secs() 
            });
        }
        
        requests.push(now);
        Ok(())
    }
}
```

## テスト戦略

### 単体テストの書き方
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_badge_generation_active_user() {
        // Arrange
        let mut mock_github = MockGitHubClient::new();
        mock_github
            .expect_get_contributions()
            .with(eq("octocat"))
            .times(1)
            .returning(|_| Ok(test_fixtures::active_contributions()));
        
        // Act
        let service = BadgeService::new(Arc::new(mock_github));
        let badge = service.generate_badge("octocat").await.unwrap();
        
        // Assert
        assert!(badge.contains("Active"));
        assert!(badge.contains("#44cc11")); // 緑色
    }
}
```

### 統合テストの構成
```rust
#[tokio::test]
async fn test_api_integration() {
    // テスト用のアプリケーションを起動
    let app = create_test_app().await;
    let client = TestClient::new(app);
    
    // APIエンドポイントをテスト
    let response = client
        .get("/badge/octocat.svg")
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "image/svg+xml"
    );
}
```