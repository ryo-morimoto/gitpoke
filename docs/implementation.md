# GitPoke 実装ガイド

## 関数型ドメインモデリングの原則

### 1. Make Illegal States Unrepresentable（不正な状態を表現不可能にする）

```rust
// ❌ 悪い例：不正な状態が可能
struct User {
    id: Option<i64>,        // idなしのUserが作れる
    username: String,       // 空文字列が可能
    poke_enabled: bool,
    settings: Option<Settings>, // poke_enabledがtrueでもsettingsがない
}

// ✅ 良い例：型で正しさを保証
enum UserState {
    Anonymous(Username),
    Registered(RegisteredUser),
}

struct RegisteredUser {
    id: GitHubUserId,        // 必ず存在
    username: Username,      // 検証済みの値
    poke_setting: PokeSetting, // 必ず設定を持つ
}
```

### 2. Parse, Don't Validate（検証ではなく解析する）

```rust
// ❌ 悪い例：検証後も元の型のまま
fn validate_username(s: &str) -> Result<(), Error> {
    if s.is_empty() || s.len() > 39 {
        return Err(Error::InvalidUsername);
    }
    Ok(())
}

// ✅ 良い例：解析して新しい型を返す
// 共通バリデーションtrait
trait Validated: Sized {
    type Error;
    fn validate(self) -> Result<Self, Self::Error>;
}

impl Username {
    fn parse(s: String) -> Result<Self, ValidationError> {
        Username(s).validate()
    }
}

impl Validated for Username {
    type Error = ValidationError;
    
    fn validate(self) -> Result<Self, Self::Error> {
        if self.0.is_empty() || self.0.len() > 39 {
            return Err(ValidationError::InvalidUsername);
        }
        Ok(self)
    }
}
```

### 3. Total Functions（全域関数を使う）

```rust
// ❌ 部分関数：panicする可能性
fn get_user(id: i64) -> User {
    users.get(&id).unwrap() // panic!
}

// ✅ 全域関数：すべての入力に対して結果を返す
fn find_user(id: GitHubUserId) -> Option<User> {
    users.get(&id).cloned()
}
```

## ドメインモデリングパターン

### 1. 状態遷移を型で表現

```rust
// アクティビティの状態遷移
enum ActivityState {
    Active { last_activity: DateTime<Utc> },
    Inactive { since: DateTime<Utc> },
}

impl ActivityState {
    fn from_last_activity(last: DateTime<Utc>, now: DateTime<Utc>) -> Self {
        let days_inactive = (now - last).num_days();
        if days_inactive <= 7 {
            ActivityState::Active { last_activity: last }
        } else {
            ActivityState::Inactive { since: last }
        }
    }
}
```

### 2. ビジネスルールを関数で表現

```rust
// Poke可否の判定ロジック
fn check_poke_permission(
    sender: &Username,
    recipient: &RegisteredUser,
    relation: &FollowRelation,
) -> Result<(), PokeError> {
    match recipient.poke_setting {
        PokeSetting::Disabled => 
            Err(PokeError::RecipientDisabled),
        PokeSetting::Anyone => 
            Ok(()),
        PokeSetting::FollowersOnly => 
            if relation.is_follower(sender) {
                Ok(())
            } else {
                Err(PokeError::NotFollower)
            },
        PokeSetting::MutualOnly => 
            if relation.is_mutual(sender) {
                Ok(())
            } else {
                Err(PokeError::NotMutualFollower)
            },
    }
}
```

### 3. 副作用の分離

```rust
// ユースケース層：純粋関数の組み合わせ
mod use_cases {
    pub fn process_poke_request(
        sender: Username,
        recipient: UserState,
        relation: FollowRelation,
        rate_limit: &RateLimitState,
    ) -> Result<PokeCommand, PokeError> {
        // すべて純粋関数
        let capability = check_poke_capability(&sender, &recipient, &relation)?;
        let rate_check = check_rate_limit(&sender, rate_limit)?;
        let event = create_poke_event(sender, recipient)?;
        
        Ok(PokeCommand::Execute(event))
    }
}

// ハンドラー層：副作用を扱う
impl PokeHandler {
    async fn handle(&self, req: PokeRequest) -> Result<PokeResponse> {
        // 1. データ取得（副作用）
        let sender = self.auth.get_current_user(&req).await?;
        let recipient = self.users.find(&req.recipient).await?;
        let relation = self.github.get_relation(&sender, &req.recipient).await?;
        let rate_limit = self.cache.get_rate_limit(&sender).await?;
        
        // 2. ビジネスロジック（純粋）
        let command = use_cases::process_poke_request(
            sender,
            recipient,
            relation,
            &rate_limit,
        )?;
        
        // 3. コマンド実行（副作用）
        match command {
            PokeCommand::Execute(event) => {
                self.events.save(&event).await?;
                self.notifier.notify(&event).await?;
                Ok(PokeResponse::Success)
            }
        }
    }
}
```

## エラーハンドリング

### Result型を活用した明示的なエラー処理

```rust
// ドメインエラー
#[derive(Debug, thiserror::Error)]
enum DomainError {
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    
    #[error("User not found: {0}")]
    UserNotFound(Username),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

// 関数の戻り値で可能なエラーを明示
async fn find_user_activity(
    username: &Username,
    github: &GitHubClient,
) -> Result<ActivityState, DomainError> {
    let activity = github
        .get_user_activity(username)
        .await
        .map_err(|_| DomainError::UserNotFound(username.clone()))?;
    
    Ok(ActivityState::from_last_activity(
        activity.last_commit,
        Utc::now(),
    ))
}
```

### アプリケーションエラー型設計

```rust
use thiserror::Error;

// アプリケーション全体のエラー型を統一
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("ドメインエラー: {0}")]
    Domain(#[from] DomainError),
    
    #[error("インフラエラー: {0}")]
    Infra(#[from] InfraError),
    
    #[error("ハンドラーエラー: {0}")]
    Handler(#[from] HandlerError),
}

// 層ごとのエラー型を明確に分離
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("無効なユーザー名")]
    InvalidUsername,
    #[error("Pokeが許可されていません")]
    PokeNotAllowed(PokeError),
    #[error("レート制限超過")]
    RateLimitExceeded,
}

#[derive(Debug, thiserror::Error)]
pub enum InfraError {
    #[error("GitHub APIエラー: {0}")]
    GitHubApi(#[from] octocrab::Error),
    #[error("データベースエラー: {0}")]
    Database(String),
    #[error("キャッシュエラー: {0}")]
    Cache(#[from] redis::RedisError),
    #[error("ストレージエラー: {0}")]
    Storage(String),
}

#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
    #[error("無効なリクエスト: {0}")]
    BadRequest(String),
    #[error("リソースが見つかりません: {0}")]
    NotFound(String),
    #[error("認証エラー")]
    Unauthorized,
    #[error("内部サーバーエラー")]
    Internal,
}

// HTTPステータスコードへの変換
impl From<AppError> for StatusCode {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Domain(DomainError::InvalidUsername) => StatusCode::BAD_REQUEST,
            AppError::Domain(DomainError::PokeNotAllowed(_)) => StatusCode::FORBIDDEN,
            AppError::Domain(DomainError::RateLimitExceeded) => StatusCode::TOO_MANY_REQUESTS,
            AppError::Handler(HandlerError::BadRequest(_)) => StatusCode::BAD_REQUEST,
            AppError::Handler(HandlerError::NotFound(_)) => StatusCode::NOT_FOUND,
            AppError::Handler(HandlerError::Unauthorized) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
```

### リトライ戦略
- **方式**: Exponential Backoff with Jitter
- **最大リトライ回数**: 3回
- **初回待機時間**: 100ms
- **最大待機時間**: 5秒
- **適用対象**: GitHub API、Redis、Firestore の一時的エラー

### フォールバック
1. Redis（ホットキャッシュ）→ 失敗時
2. Cloud Storage（コールドキャッシュ）→ 失敗時
3. リアルタイム生成

## テスト戦略

### 1. ドメインロジックの単体テスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_username_validation() {
        // 正常系
        assert!(Username::parse("alice".to_string()).is_ok());
        
        // 異常系
        assert!(Username::parse("".to_string()).is_err());
        assert!(Username::parse("a".repeat(40)).is_err());
    }
    
    #[test]
    fn test_poke_permission_anyone() {
        let recipient = create_test_user(PokeSetting::Anyone);
        let relation = FollowRelation::None;
        
        let result = check_poke_permission(
            &Username::parse("sender".to_string()).unwrap(),
            &recipient,
            &relation,
        );
        
        assert!(result.is_ok());
    }
}
```

### 2. 統合テストでの副作用確認

```rust
#[tokio::test]
async fn test_poke_flow_with_mutual_follow() {
    // テスト用の依存性を注入
    let deps = create_test_dependencies();
    
    // テストデータ設定
    deps.users.insert(create_mutual_follow_user()).await;
    deps.github.set_mutual_follow("alice", "bob").await;
    
    // 実行
    let handler = PokeHandler::new(deps);
    let response = handler.handle(create_poke_request()).await;
    
    // 検証
    assert!(response.is_ok());
    assert_eq!(deps.events.count().await, 1);
}
```

### 3. テスト指標

### カバレッジ目標
- **ドメイン層**: 100%（純粋関数のため完全テスト可能）
- **ユースケース層**: 95%以上
- **エラーハンドリング**: 100%
- **ハンドラー層**: 70%以上
- **インフラ層**: 60%以上（統合テストでカバー）

### テストツール
- **単体テスト**: mockall（トレイトモック）
- **統合テスト**: testcontainers（Docker環境）
- **HTTPテスト**: axum::test
- **E2Eテスト**: docker-compose環境

## 依存性注入パターン

```rust
// 依存性をtraitで抽象化
#[async_trait]
trait UserRepository {
    async fn find(&self, username: &Username) -> Result<Option<UserState>>;
    async fn save(&self, user: &RegisteredUser) -> Result<()>;
}

// アプリケーション層で具体的な実装を注入
struct AppDependencies {
    user_repo: Arc<dyn UserRepository>,
    github_api: Arc<dyn GitHubApi>,
    cache: Arc<dyn Cache>,
    notifier: Arc<dyn Notifier>,
}

// テスト時はモックを注入
#[cfg(test)]
fn create_test_dependencies() -> AppDependencies {
    AppDependencies {
        user_repo: Arc::new(InMemoryUserRepo::new()),
        github_api: Arc::new(MockGitHubApi::new()),
        cache: Arc::new(MockCache::new()),
        notifier: Arc::new(NoOpNotifier),
    }
}
```

## キャッシュ戦略

### 多層キャッシュ
1. **Memorystore (Redis)**
   - TTL: 5分（アクティブユーザー）、1時間（非アクティブ）
   - 用途: GitHub APIレスポンス、生成済みバッジ

2. **Cloud Storage + CDN**
   - TTL: 24時間
   - 用途: 静的バッジファイル
   - Cache-Control: `public, max-age=300, stale-while-revalidate=86400`

3. **アプリケーションメモリ**
   - 共通設定、テンプレート等

### キャッシュキー設計
```
badge:{username}:{type}:{version}
activity:{username}:{date}
```

## GitHub API統合

### GraphQL クエリ
```graphql
query GetUserActivity($username: String!) {
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
```

### レート制限対策
- プライマリ: User Access Token（5,000 points/hour）
- フォールバック: Public API（60 requests/hour）
- キャッシュ活用で実際のAPI呼び出しを最小化

## バッジ生成

### SVGテンプレート
```rust
fn generate_badge_svg(username: &str, days_inactive: i64) -> String {
    let color = if days_inactive <= 7 { "#44cc11" } else { "#e05d44" };
    let status = if days_inactive <= 7 { "active" } else { "inactive" };
    
    format!(r#"
    <svg xmlns="http://www.w3.org/2000/svg" width="120" height="20">
        <rect width="120" height="20" fill="{color}"/>
        <text x="10" y="14" fill="white" font-family="Arial" font-size="12">
            GitPoke: {status}
        </text>
    </svg>
    "#, color = color, status = status)
}
```

### つっつき可能バッジ
- 7日以上非アクティブ時にクリック可能
- JavaScript埋め込みでGitHub.com上で動作
- CORS対応APIエンドポイント

## CI/CD設定

### GitHub Actions - CI
```yaml
name: CI
on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

### GitHub Actions - Deploy
```yaml
name: Deploy
on:
  push:
    branches: [main]
    tags: ['v*']

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      # 環境判定
      - name: Set environment
        run: |
          if [[ $GITHUB_REF == refs/tags/v* ]]; then
            echo "ENVIRONMENT=production" >> $GITHUB_ENV
          else
            echo "ENVIRONMENT=staging" >> $GITHUB_ENV
          fi
      
      # ビルドとデプロイ
      # ... (詳細は architecture.md 参照)
```

## セキュリティ実装

### シークレット管理
- Google Secret Managerで管理
- 環境変数経由でアプリに注入
- ローテーション対応

### CORS設定
```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin("https://github.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any);
```

## 関数合成ヘルパー

```rust
// src/utils/functional.rs
pub fn pipe<A, B, C>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

pub fn pipe3<A, B, C, D>(
    f: impl Fn(A) -> B,
    g: impl Fn(B) -> C,
    h: impl Fn(C) -> D,
) -> impl Fn(A) -> D {
    move |a| h(g(f(a)))
}

// Result型対応のパイプ
pub fn pipe_result<A, B, C, E>(
    f: impl Fn(A) -> Result<B, E>,
    g: impl Fn(B) -> Result<C, E>,
) -> impl Fn(A) -> Result<C, E> {
    move |a| f(a).and_then(|b| g(b))
}

// 非同期関数のパイプ
pub fn pipe_async<A, B, C, Fut1, Fut2>(
    f: impl Fn(A) -> Fut1,
    g: impl Fn(B) -> Fut2,
) -> impl Fn(A) -> impl Future<Output = C>
where
    Fut1: Future<Output = B>,
    Fut2: Future<Output = C>,
{
    move |a| async move { g(f(a).await).await }
}

// 使用例
fn example() {
    let process = pipe3(
        |s: String| s.trim().to_string(),
        |s| Username::parse(s),
        |u| u.map(|username| UserState::Anonymous(username)),
    );
    
    let result = process("  alice  ".to_string());
}
```

## 実装の進め方

1. **ドメイン層から始める**
   - 値オブジェクト（Username, GitHubUserId等）
   - Validated traitの実装
   - エンティティ（UserState, ActivityState等）
   - ドメインロジック（純粋関数）

2. **ユースケース層の実装**
   - ビジネスルールの組み合わせ
   - 関数合成を活用した処理フロー
   - エラーハンドリング
   - 単体テストの作成

3. **インフラ層の実装**
   - 外部APIクライアント
   - データベースアクセス
   - キャッシュ実装
   - エラー型の変換

4. **ハンドラー層で統合**
   - HTTPエンドポイント
   - 依存性の注入
   - エラーのHTTPステータス変換
   - 統合テスト

この順序で実装することで、ビジネスロジックがインフラに依存しない、テストしやすいコードベースを構築できます。

### レート制限
- IPベース: 100 requests/minute
- ユーザーベース: 1000 requests/hour
- Redisでカウンター管理