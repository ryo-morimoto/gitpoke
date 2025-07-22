# GitPoke 命名規則

このドキュメントは、GitPokeプロジェクトで使用する命名規則を定義します。
一貫性のある命名により、コードの可読性と保守性を向上させます。

## 📁 ファイル名

### ドメイン層 (`src/domain/`)

| 種別 | 命名パターン | 例 | 説明 |
|------|-------------|-----|------|
| 値オブジェクト | `{名詞}.rs` | `user.rs`, `poke.rs` | 単数形の名詞を使用 |
| 共通機能 | `{機能}s.rs` | `validations.rs` | 複数の関連機能をまとめる場合は複数形 |

### ユースケース層 (`src/use_cases/`)

| 種別 | 命名パターン | 例 | 説明 |
|------|-------------|-----|------|
| アクション | `{動詞}_{対象}.rs` | `check_poke.rs`, `generate_badge.rs` | 動詞で始まるアクション名 |

### ハンドラー層 (`src/handlers/`)

| 種別 | 命名パターン | 例 | 説明 |
|------|-------------|-----|------|
| HTTPハンドラー | `{リソース}.rs` | `auth.rs`, `badge.rs`, `poke.rs` | REST リソース名 |

### インフラ層 (`src/infra/`)

| 種別 | 命名パターン | 例 | 説明 |
|------|-------------|-----|------|
| リポジトリ | `{エンティティ}_repository.rs` | `user_repository.rs` | データ永続化の抽象化 |
| APIクライアント | `{サービス}_api.rs` | `github_api.rs` | 外部APIとの通信 |
| サービス | `{機能}_service.rs` | `cache_service.rs`, `notification_service.rs` | 外部サービスとの連携 |
| ストア | `{対象}_store.rs` | `event_store.rs` | イベントやログの保存 |

## 🏷️ 型名

### 基本的な型

```rust
// 値オブジェクト（単数形）
struct Username(String);
struct GitHubUserId(i64);
struct Email(String);

// エンティティ（単数形）
struct User { ... }
struct PokeEvent { ... }
struct Badge { ... }

// コレクション（複数形）
struct Users(Vec<User>);
struct PokeEvents(Vec<PokeEvent>);

// 状態を表すenum（〜State）
enum UserState { 
    Anonymous(Username),
    Registered(RegisteredUser),
}
enum ActivityState {
    Active { last_activity: DateTime<Utc> },
    Inactive { since: DateTime<Utc> },
}

// 能力・可能性を表すenum（〜Capability, 〜Ability）
enum PokeCapability {
    CanPoke { from: Username, to: Username },
    CannotPoke(PokeError),
}
```

### エラー型

```rust
// 層別のエラー型（〜Error）
enum DomainError { ... }
enum InfraError { ... }
enum HandlerError { ... }
enum ValidationError { ... }

// 特定機能のエラー（{機能}Error）
enum PokeError { ... }
enum BadgeError { ... }
```

### Trait

```rust
// 能力を表すtrait（形容詞的な名前）
trait Validated { ... }
trait Cacheable { ... }
trait Serializable { ... }

// リポジトリパターン（〜Repository）
trait UserRepository { ... }
trait EventRepository { ... }

// サービスパターン（〜Service）
trait NotificationService { ... }
trait CacheService { ... }
```

## 📝 関数名

### ドメイン層・ユースケース層（純粋関数）

| アクション | 命名パターン | 例 | 用途 |
|-----------|-------------|-----|------|
| 検証 | `validate_{対象}` | `validate_username()` | 値の妥当性チェック |
| 計算 | `calculate_{結果}` | `calculate_activity_days()` | 値の計算 |
| 判定 | `determine_{状態}` | `determine_user_state()` | 状態の判定 |
| チェック | `check_{条件}` | `check_poke_capability()` | 条件の確認 |
| 生成 | `generate_{成果物}` | `generate_badge_svg()` | 新しい値の生成 |
| 変換 | `{from}_to_{to}` | `username_to_string()` | 型変換 |
| 解析 | `parse_{対象}` | `parse_username()` | 文字列から型への変換 |

### リポジトリ層

```rust
// 基本的なCRUD操作
async fn find(&self, id: &Id) -> Result<Option<T>>;
async fn find_by_{field}(&self, value: &Type) -> Result<Option<T>>;
async fn find_all(&self) -> Result<Vec<T>>;
async fn save(&self, entity: &T) -> Result<()>;
async fn update(&self, entity: &T) -> Result<()>;
async fn delete(&self, id: &Id) -> Result<()>;

// 特定条件での検索
async fn find_by_username(&self, username: &Username) -> Result<Option<User>>;
async fn find_active_users(&self) -> Result<Vec<User>>;
```

### ハンドラー層

```rust
// HTTPハンドラー（RESTful）
async fn get_{resource}()    // GET /resource
async fn list_{resources}()  // GET /resources
async fn create_{resource}() // POST /resources
async fn update_{resource}() // PUT /resources/:id
async fn delete_{resource}() // DELETE /resources/:id

// 例
async fn get_badge(Path(username): Path<String>) -> Result<Response>;
async fn create_poke(Json(req): Json<PokeRequest>) -> Result<Response>;
```

## 🔧 変数名

### 基本ルール

```rust
// ローカル変数（snake_case）
let user_name = "alice";
let is_active = true;
let poke_count = 42;

// 定数（SCREAMING_SNAKE_CASE）
const MAX_USERNAME_LENGTH: usize = 39;
const DEFAULT_CACHE_TTL: u64 = 300;
const GITHUB_API_BASE_URL: &str = "https://api.github.com";

// スタティック変数（SCREAMING_SNAKE_CASE）
static USERNAME_REGEX: Lazy<Regex> = Lazy::new(...);
```

### 命名の原則

1. **明確性**: 変数の用途が名前から分かる
   ```rust
   // ❌ 悪い例
   let d = 7;
   let temp = fetch_user();
   
   // ✅ 良い例
   let days_inactive = 7;
   let current_user = fetch_user();
   ```

2. **一貫性**: 同じ概念には同じ名前を使用
   ```rust
   // ユーザー名は常に username（user_name, name, uname は使わない）
   let username = Username::parse("alice")?;
   ```

3. **スコープに応じた詳細度**
   ```rust
   // 短いスコープでは短い名前でもOK
   users.iter().map(|u| u.username.clone())
   
   // 長いスコープでは詳細な名前
   let authenticated_user = get_current_user(&session)?;
   ```

## 📦 モジュール名

```rust
// 機能別にグループ化（複数形）
mod handlers;
mod services;
mod models;
mod utils;

// 単一の責務を持つモジュール（単数形）
mod auth;
mod cache;
mod error;
```

## 🎯 プロジェクト固有の規則

### GitPoke特有の用語

| 用語 | 使用する名前 | 使用しない名前 |
|------|-------------|----------------|
| つつく | `poke` | `nudge`, `ping`, `notify` |
| 活動状態 | `activity` | `status`, `state`（文脈による） |
| バッジ | `badge` | `icon`, `image`, `svg` |
| 非アクティブ期間 | `days_inactive` | `inactive_days`, `inactivity_period` |

### 一貫性のためのルール

1. **GitHub関連**
   - ユーザー名: `username`（`user_name`, `login`は使わない）
   - ユーザーID: `github_id`（`user_id`は内部IDと混同を避ける）

2. **時間関連**
   - 作成日時: `created_at`
   - 更新日時: `updated_at`
   - 最終活動: `last_activity`

3. **Result型の変数名**
   ```rust
   // Result型を返す関数の戻り値は result または res
   let result = validate_username(&input)?;
   
   // 展開後は意味のある名前
   let username = validate_username(&input)?;
   ```

この命名規則に従うことで、GitPokeのコードベース全体で一貫性を保ち、
新しい開発者も既存のパターンを理解しやすくなります。