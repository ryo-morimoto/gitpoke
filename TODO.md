# GitPoke TODO

## 完了したタスク ✅

### Domain層
- [x] Username型の実装とバリデーション
- [x] PokeCapabilityのビジネスルール実装
- [x] BadgeStateの実装
- [x] GitHubActivityの実装
- [x] FollowRelationの実装
- [x] ValidationErrorの実装
- [x] Domain層の100%テストカバレッジ達成

### 既存コードの修正
- [x] poke.rsのrepositoryフィールドをcontextに変更
- [x] badge.rsのTODOコメントを具体的に修正
- [x] routes.rsの静的ファイル配信TODO削除
- [x] health.rsのヘルスチェックを並列処理化
- [x] auth.rsのrequire_authをミドルウェアに移動
- [x] handlers/poke.rsのエラーメッセージ→ステータスコードマッピング修正
- [x] user.rsのエラーメッセージを日本語に統一、ユーザー検索削除
- [x] handlers/utils.rsの未使用ページネーションコード削除
- [x] check_poke.rsのナンバリングコメント削除
- [x] generate_badge.rsのTTL計算を設定値使用に修正

### アーキテクチャ基盤
- [x] リポジトリ層のトレイトと空実装を作成
  - [x] UserRepository (PostgresUserRepository)
  - [x] PokeRepository (PostgresPokeRepository)
  - [x] BadgeRepository (PostgresBadgeRepository)
- [x] サービス層の構造体と空実装を作成
  - [x] UserService
  - [x] PokeService
  - [x] BadgeService
  - [x] AuthService
- [x] アダプター層の構造体と空実装を作成
  - [x] GitHubAdapter (GitHubApiAdapter)
  - [x] DatabaseAdapter (PostgresAdapter)
  - [x] OAuthAdapter (GitHubOAuthAdapter)
- [x] キャッシュ層のトレイトと空実装を作成
  - [x] CacheAdapter (RedisCache, InMemoryCache)

### エラーハンドリング
- [x] 統一エラー型（AppError）の定義
- [x] 各層のエラー型定義（DomainError, InfraError, HandlerError）
- [x] HTTPステータスコードへのマッピング実装
- [x] エラー型間の変換実装（From trait）

## 実装必要なタスク 🚧

### 1. 基盤部分（最優先）

#### 設定管理
- [ ] `Config::from_env()` - 環境変数から設定を読み込む
  - 各環境変数の読み込み（PORT, GITHUB_APP_ID等）
  - 必須項目のバリデーション
  - デフォルト値の適用
- [ ] `Config::validate()` - 設定の妥当性検証
  - ポート番号の範囲チェック
  - URLの形式チェック
  - ファイルパスの存在確認

#### 依存性注入
- [ ] `AppDependencies::new()` - 依存性コンテナの初期化
  - GitHubAPIクライアントの初期化
  - ユーザーリポジトリの初期化
  - イベントストアの初期化
  - キャッシュサービスの初期化
  - 通知サービスの初期化
  - レート制限サービスの初期化

### 2. インフラストラクチャ層

#### アダプター実装 (`src/infra/adapters/`)
- [ ] `github.rs` - GitHubApiAdapter
  - [ ] `get_user()` - アクセストークンでユーザー情報取得
  - [ ] `get_user_by_username()` - ユーザー名でユーザー情報取得
  - [ ] `verify_token()` - トークンの有効性確認
- [ ] `database.rs` - PostgresAdapter
  - [ ] `execute()` - SQLクエリの実行
  - [ ] `query_one()` - 単一結果の取得
  - [ ] `query_many()` - 複数結果の取得
  - [ ] `transaction()` - トランザクション処理
- [ ] `oauth.rs` - GitHubOAuthAdapter
  - [ ] `exchange_code()` - 認可コードをトークンに交換
  - [ ] `refresh_token()` - トークンのリフレッシュ
  - [ ] `revoke_token()` - トークンの無効化

#### リポジトリ実装 (`src/infra/repositories/`)
- [ ] `user.rs` - PostgresUserRepository
  - [ ] `find_by_id()` - IDでユーザー検索
  - [ ] `find_by_github_id()` - GitHub IDでユーザー検索
  - [ ] `save()` - 新規ユーザー保存
  - [ ] `update()` - ユーザー情報更新
  - [ ] `delete()` - ユーザー削除
- [ ] `poke.rs` - PostgresPokeRepository
  - [ ] `find_by_id()` - IDでPoke検索
  - [ ] `find_by_sender_and_recipient()` - 送信者と受信者でPoke検索
  - [ ] `save()` - 新規Poke保存
  - [ ] `count_by_recipient()` - 受信者別のPoke数カウント
  - [ ] `list_recent()` - 最近のPokeリスト取得
- [ ] `badge.rs` - PostgresBadgeRepository
  - [ ] `get_poke_count()` - ユーザーのPoke数取得
  - [ ] `generate_badge()` - バッジ生成

#### キャッシュ実装 (`src/infra/cache/`)
- [ ] RedisCache
  - [ ] `get()` - キャッシュ取得
  - [ ] `set()` - キャッシュ設定
  - [ ] `delete()` - キャッシュ削除
  - [ ] `exists()` - キャッシュ存在確認
  - [ ] `expire()` - 有効期限設定
- [ ] InMemoryCache
  - [ ] 同上のメソッド実装（HashMap使用）

### 3. 認証システム

#### GitHub OAuth (`src/handlers/auth.rs`)
- [ ] `build_github_oauth_url()` - OAuth URL構築
- [ ] `exchange_code_for_token()` - トークン交換
- [ ] `get_github_user()` - ユーザー情報取得
- [ ] `create_or_update_user()` - ユーザー作成/更新
- [ ] `create_user_session()` - セッション作成

### 4. サービス層の実装 (`src/app/services/`)

#### UserService
- [ ] `get_user()` - ユーザー情報取得
- [ ] `get_user_by_github_id()` - GitHub IDでユーザー検索
- [ ] `create_user()` - 新規ユーザー作成
- [ ] `update_user()` - ユーザー情報更新
- [ ] `delete_user()` - ユーザー削除

#### PokeService
- [ ] `send_poke()` - Poke送信（バリデーション含む）
- [ ] `get_poke()` - Poke情報取得
- [ ] `get_pokes_between()` - ユーザー間のPoke履歴
- [ ] `count_received_pokes()` - 受信Poke数カウント
- [ ] `get_recent_pokes()` - 最近のPoke一覧

#### BadgeService
- [ ] `generate_badge()` - バッジSVG生成
- [ ] `get_poke_count()` - Poke数取得（キャッシュ活用）

#### AuthService
- [ ] `authenticate_github_user()` - GitHubユーザー認証
- [ ] `refresh_token()` - トークンリフレッシュ
- [ ] `validate_token()` - トークン検証
- [ ] `revoke_token()` - トークン無効化

### 5. 統合タスク

#### 依存性の接続
- [ ] AppDependenciesでの各サービス/リポジトリの初期化
- [ ] ハンドラーへのサービス注入
- [ ] データベース接続プールの実装
- [ ] Redis接続の実装

#### エラーハンドリング
- [ ] カスタムエラー型の実装
- [ ] エラーのHTTPステータスへのマッピング
- [ ] エラーログの実装

### 6. テスト追加

#### 単体テスト
- [ ] 各リポジトリのモックテスト
- [ ] 各サービスのビジネスロジックテスト
- [ ] アダプターのモックテスト

#### 統合テスト
- [ ] API エンドポイントのE2Eテスト
- [ ] 認証フローのテスト
- [ ] Poke送信フローのテスト

### 7. デプロイメント準備

#### Docker
- [ ] マルチステージビルドの最適化
- [ ] 本番用設定の追加

#### CI/CD
- [ ] GitHub Actionsワークフローの拡張
- [ ] Cloud Runへの自動デプロイ

## 技術的負債 💳

- [ ] ロギングの実装（tracing）
- [ ] メトリクスの実装（prometheus）
- [ ] OpenTelemetryの統合
- [ ] データベースマイグレーション管理

## ドキュメント 📝

- [ ] API仕様書の作成（OpenAPI）
- [ ] デプロイメントガイド
- [ ] 開発者向けドキュメント
- [ ] アーキテクチャ決定記録（ADR）の更新