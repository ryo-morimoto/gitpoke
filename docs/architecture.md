# GitPoke アーキテクチャ

## 設計思想

### 関数型ドメインモデリング
GitPokeは関数型プログラミングの原則に基づいて設計されています：

- **イミュータブルデータ構造**: すべてのドメインモデルは不変
- **純粋関数**: ビジネスロジックは副作用なし
- **型による正しさの保証**: 不正な状態を型で排除
- **関数合成**: 小さな関数を組み合わせて複雑なロジックを構築

### エラーハンドリング戦略

```rust
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
}
```

## 技術スタック

### 言語・フレームワーク
- **言語**: Rust
- **Webフレームワーク**: axum
- **非同期ランタイム**: tokio

### 主要クレート
| カテゴリ | クレート | 用途 |
|---------|---------|------|
| GitHub API | octocrab | GitHub GraphQL/REST API クライアント |
| データベース | firestore | Cloud Firestore クライアント |
| キャッシュ | redis | Memorystore (Redis) クライアント |
| ストレージ | cloud-storage | Cloud Storage クライアント |
| シリアライゼーション | serde/serde_json | JSON処理 |
| 日時処理 | chrono | タイムスタンプ管理 |
| ロギング | tracing | 構造化ログ |
| エラー処理 | thiserror | エラー型定義 |
| ミドルウェア | tower/tower-http | HTTP処理 |

## インフラストラクチャ

### デプロイメント
- **プラットフォーム**: GCP Cloud Run
- **リージョン**: asia-northeast1（東京）
- **アーキテクチャパターン**: モノリシック（単一コンテナ）

### データストレージ
```
┌─────────────────────────────────────────────────┐
│                  Cloud Run                       │
│                 (GitPoke App)                    │
└─────────────────┬───────────────┬───────────────┘
                  │               │
        ┌─────────▼─────────┐    │    ┌─────────────┐
        │   Memorystore     │    │    │   Cloud     │
        │    (Redis)        │    │    │  Storage    │
        │                   │    │    │   + CDN     │
        │ - Hot cache       │    │    │             │
        │ - API responses   │    │    │ - Badge SVG │
        │ - Rate limiting   │    │    │   cache     │
        └───────────────────┘    │    └─────────────┘
                                 │
                      ┌──────────▼──────────┐
                      │   Cloud Firestore   │
                      │                     │
                      │ - User auth data    │
                      │ - Activity history  │
                      └────────────────────┘
```

## プロジェクト構造

```
gitpoke/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs              # エントリーポイント
│   ├── routes.rs            # ルーティング定義
│   ├── domain/              # ドメイン層（純粋）
│   │   ├── mod.rs
│   │   ├── user.rs          # UserState, Username等の値オブジェクト
│   │   ├── poke.rs          # PokeCapability, PokeEvent
│   │   ├── badge.rs         # BadgeState, Activity判定
│   │   ├── github.rs        # GitHubActivity, FollowRelation
│   │   └── validation.rs    # 共通バリデーションtrait
│   ├── use_cases/           # ユースケース（純粋関数）
│   │   ├── mod.rs
│   │   ├── check_poke.rs    # Poke可否判定ロジック
│   │   ├── generate_badge.rs# バッジ生成ロジック
│   │   └── user_settings.rs # 設定変更ロジック
│   ├── handlers/            # HTTPハンドラー（副作用の境界）
│   │   ├── mod.rs
│   │   ├── auth.rs          # OAuth認証
│   │   ├── badge.rs         # バッジエンドポイント
│   │   ├── poke.rs          # Pokeエンドポイント
│   │   └── user.rs          # ユーザー設定
│   ├── infra/               # インフラ層（副作用）
│   │   ├── mod.rs
│   │   ├── github_api.rs    # GitHub APIクライアント
│   │   ├── user_repository.rs # ユーザーデータ永続化
│   │   ├── event_store.rs   # イベントストア
│   │   ├── cache_service.rs # Redisキャッシュサービス
│   │   └── notification_service.rs # 通知サービス
│   ├── app/                 # アプリケーション設定
│   │   ├── mod.rs
│   │   ├── dependencies.rs  # 依存性注入コンテナ
│   │   └── config.rs        # 設定
│   ├── error.rs             # エラー型定義
│   └── utils/               # ユーティリティ
│       ├── mod.rs
│       └── functional.rs    # 関数合成ヘルパー
├── templates/               # SVGテンプレート
│   └── badge.svg
├── tests/                   # テスト
│   ├── integration/
│   └── e2e/
└── .github/
    └── workflows/           # GitHub Actions
        ├── ci.yml
        └── deploy.yml
```

## ドメインモデリング

### 中心となる型と関数

```rust
// 関数合成ユーティリティ
mod utils {
    pub fn pipe<A, B, C>(
        f: impl Fn(A) -> B,
        g: impl Fn(B) -> C,
    ) -> impl Fn(A) -> C {
        move |a| g(f(a))
    }
    
    pub fn pipe_async<A, B, C, F1, F2>(
        f1: F1,
        f2: F2,
    ) -> impl Fn(A) -> impl Future<Output = C>
    where
        F1: Fn(A) -> impl Future<Output = B>,
        F2: Fn(B) -> impl Future<Output = C>,
    {
        move |a| async move { f2(f1(a).await).await }
    }
}

// バッジ生成は純粋関数
fn generate_badge(
    activity: &GitHubActivity,
    user_state: &UserState,
) -> BadgeSvg {
    match (activity.days_since_last_activity(), user_state) {
        (days, UserState::Registered(_)) if days > 7 => 
            BadgeSvg::interactive(activity),
        _ => BadgeSvg::static_only(activity),
    }
}

// Poke可否判定も純粋関数
fn check_poke_capability(
    sender: &Username,
    recipient: &UserState,
    follow_relation: &FollowRelation,
) -> PokeCapability {
    match recipient {
        UserState::Anonymous(_) => 
            PokeCapability::CannotPoke(PokeError::RecipientNotRegistered),
        UserState::Registered(user) => 
            check_permission(sender, user, follow_relation),
    }
}

// 副作用は境界で処理
async fn handle_poke_request(
    sender: Username,
    recipient: Username,
    deps: &AppDependencies,
) -> Result<PokeResult, AppError> {
    // 1. データ取得（副作用）
    let recipient_state = deps.user_repo.find(&recipient).await?;
    let follow_relation = deps.github_api.get_follow_relation(&sender, &recipient).await?;
    
    // 2. ビジネスロジック（純粋）
    let capability = check_poke_capability(&sender, &recipient_state, &follow_relation);
    
    // 3. 結果に基づく処理（副作用）
    match capability {
        PokeCapability::CanPoke { from, to } => {
            let poke_event = PokeEvent::new(from, to);
            deps.event_store.save(poke_event).await?;
            deps.notifier.notify(poke_event).await?;
            Ok(PokeResult::Success)
        }
        PokeCapability::CannotPoke(error) => 
            Ok(PokeResult::Failed(error)),
    }
}
```

## データフロー（関数型アプローチ）

### バッジ表示フロー
```rust
// 純粋な変換パイプライン
Request → extract_username
        → load_or_fetch_activity  // IO境界
        → determine_user_state     // IO境界
        → generate_badge           // 純粋関数
        → render_svg              // 純粋関数
        → cache_and_respond       // IO境界
```

### Pokeフロー
```rust
// イベントドリブンアプローチ
Request → authenticate_sender      // IO境界
        → validate_recipient       // IO境界
        → check_follow_relation    // IO境界
        → check_poke_capability    // 純粋関数
        → create_poke_event       // 純粋関数
        → persist_and_notify      // IO境界
```

### 型安全性の例

```rust
// コンパイル時に不正な状態を防ぐ

// ❌ これはコンパイルエラー
let poke = PokeEvent {
    from: Username(""),  // 空のユーザー名は作れない
    to: registered_user,
};

// ✅ 正しい使い方
let username = Username::parse("alice".to_string())?;
let poke = PokeEvent::new(username, recipient)?;

// ❌ 認証なしでPokeはできない
fn handle_poke(req: Request) -> Result<Response> {
    let sender = extract_authenticated_user(&req)?; // 必須
    // ...
}
```

## CI/CD

### GitHub Actions構成
- **CI**: `main`ブランチとPRでテスト実行
- **CD**: 
  - `main`プッシュ → ステージング環境
  - `v*`タグ → 本番環境

### ビルド最適化
- Rust cache（Swatinem/rust-cache）
- Docker layer cache
- cargo-chefによる依存関係キャッシュ

## セキュリティ

### 認証・認可

#### GitHub OAuth
- **スコープ**: なし（公開情報のみで十分）
- **フロー**: Web Application Flow
- **セッション**: HTTPOnly Cookieで管理

#### アカウントシステム
```rust
// 型による不正状態の排除
#[derive(Clone, Debug, PartialEq)]
struct GitHubUserId(i64);

#[derive(Clone, Debug, PartialEq)]
struct Username(String);

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

// ユーザーの状態を型で表現
#[derive(Clone, Debug)]
enum UserState {
    Anonymous(Username),  // アカウント未作成
    Registered(RegisteredUser),
}

#[derive(Clone, Debug)]
struct RegisteredUser {
    github_id: GitHubUserId,
    username: Username,
    poke_setting: PokeSetting,
    created_at: DateTime<Utc>,
}

// Poke受信設定（デフォルトはAnyone）
#[derive(Clone, Debug, PartialEq)]
enum PokeSetting {
    Anyone,
    FollowersOnly,
    MutualOnly,
    Disabled,
}

// Poke可能性を型で表現
#[derive(Clone, Debug)]
enum PokeCapability {
    CanPoke { from: Username, to: Username },
    CannotPoke(PokeError),
}

#[derive(Clone, Debug, thiserror::Error)]
enum PokeError {
    #[error("受信者が登録されていません")]
    RecipientNotRegistered,
    #[error("受信者がPokeを無効にしています")]
    RecipientDisabled,
    #[error("フォロワーではありません")]
    NotFollower,
    #[error("相互フォローではありません")]
    NotMutualFollower,
    #[error("レート制限に達しました")]
    RateLimited,
}
```

### Poke受信制御

1. **明示的オプトイン**
   - gitpoke.devでアカウント作成 = poke受信可能
   - アカウントなし = poke受信不可（バッジ表示のみ）

2. **プライバシー設定**
   - デフォルト: 全員から受信
   - 設定変更: フォロワーのみ/相互のみ/無効化

3. **通知の透明性**
   ```json
   {
     "from": "@username",
     "repository": "github.com/user/repo",
     "timestamp": "2024-01-20T12:34:56Z"
   }
   ```

### レート制限

| 対象 | 制限 | 実装 |
|------|------|------|
| 同一IPからのpoke | 10回/分 | Redisカウンター |
| 同一ユーザーへのpoke | 1回/日 | Redisセット |
| バッジ生成 | 100回/分/IP | CDNキャッシュ |
| GitHub API | 5000回/時 | 共有レート |

### セキュリティ考慮事項

#### 解決済み
- ✅ なりすましpoke → OAuth認証必須
- ✅ 通知爆撃 → レート制限
- ✅ 勝手な通知 → オプトイン制
- ✅ プライバシー → 段階的権限設定

#### 既知の制限
- ⚠️ バッジなりすまし → 完全防御は不可（公開情報のため）
  - 対策: 通知にリファラー情報含める
- ⚠️ APIコスト → フォロー関係確認が高コスト
  - 対策: キャッシュ戦略（1時間TTL）

### 将来の拡張
- ブロックリスト機能
- 通知チャンネル選択（Email/Webhook）
- GDPR対応（データ削除）