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

#### GitHub API (`src/infrastructure/github_api.rs`)
- [ ] GitHubクライアントの実装
- [ ] ユーザーアクティビティ取得
- [ ] フォロー関係の確認
- [ ] レート制限の処理

#### リポジトリ実装
- [ ] `src/infrastructure/repositories/user_repository.rs`
  - Firestoreとの連携
  - ユーザーのCRUD操作
- [ ] `src/infrastructure/repositories/event_store.rs`
  - Pokeイベントの保存
  - 日付ベースのクエリ

#### サービス実装
- [ ] `src/infrastructure/services/cache_service.rs`
  - Redis連携
  - キャッシュの読み書き
- [ ] `src/infrastructure/services/notification_service.rs`
  - 通知送信（メール/Webhook）
- [ ] `src/infrastructure/services/rate_limiter.rs`
  - レート制限の実装

### 3. 認証システム

#### GitHub OAuth (`src/handlers/auth.rs`)
- [ ] `build_github_oauth_url()` - OAuth URL構築
- [ ] `exchange_code_for_token()` - トークン交換
- [ ] `get_github_user()` - ユーザー情報取得
- [ ] `create_or_update_user()` - ユーザー作成/更新
- [ ] `create_user_session()` - セッション作成

### 4. ビジネスロジック

#### ユーザー管理
- [ ] `get_user_stats()` - ユーザー統計情報の取得
- [ ] `find_user_by_github_id()` - GitHub IDでユーザー検索
- [ ] `delete_user_poke_events()` - ユーザーのPokeイベント削除

#### Poke機能
- [ ] IPベースのレート制限実装
- [ ] Pokeイベントの永続化

#### バッジ生成
- [ ] 実際のSVG生成ロジック
  - テキスト幅の動的計算
  - shields.io風のデザイン
- [ ] インタラクティブバッジのJavaScript

### 5. テスト追加

#### Use Cases層
- [ ] `check_poke.rs`のテスト
- [ ] `generate_badge.rs`のテスト

#### Handlers層
- [ ] 各ハンドラーの統合テスト

### 6. デプロイメント準備

#### Docker
- [ ] マルチステージビルドの最適化
- [ ] 本番用設定の追加

#### CI/CD
- [ ] GitHub Actionsワークフローの拡張
- [ ] Cloud Runへの自動デプロイ

## 技術的負債 💳

- [ ] エラーハンドリングの統一
- [ ] ロギングの実装
- [ ] メトリクスの実装
- [ ] OpenTelemetryの統合

## ドキュメント 📝

- [ ] API仕様書の作成
- [ ] デプロイメントガイド
- [ ] 開発者向けドキュメント