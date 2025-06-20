# TODO

## 設計タスク (Design Tasks) - Critical for MVP

- [ ] Rate Limiting実装設計書の作成
  - [ ] Durable Objectsを使用した具体的な実装方法
  - [ ] エンドポイント別のレート制限値の定義
  - [ ] IPベース/ユーザーベースの制限ロジック設計
  - [ ] レート制限ヘッダーの実装仕様
- [ ] Badge Click Interaction実装設計書の作成
  - [ ] SVG内でのJavaScript実装方法
  - [ ] GitHub上でのCORS対応設計
  - [ ] 匿名ユーザーのPoke処理フロー
  - [ ] クリック追跡とアナリティクス設計
- [ ] キャッシュ戦略実装設計書の作成
  - [ ] キャッシュ無効化パターンの定義
  - [ ] キャッシュキーの命名規則
  - [ ] 多層キャッシュ戦略（KV vs エッジキャッシュ）
  - [ ] stale-while-revalidateの実装詳細
- [ ] D1データベースマイグレーション実行戦略の作成
  - [ ] マイグレーションランナーの実装設計
  - [ ] ロールバック手順と安全性チェック
  - [ ] ゼロダウンタイムマイグレーション戦略

## 設計タスク (Design Tasks) - Production Readiness

- [ ] Token Refresh/Rotation戦略設計書の作成
  - [ ] 自動トークンリフレッシュの実装
  - [ ] トークンローテーションスケジュール
  - [ ] 期限切れトークンのハンドリング
- [ ] エラー復旧とレジリエンスパターン設計書の作成
  - [ ] GitHub API呼び出しのサーキットブレーカー
  - [ ] リトライ戦略（指数バックオフ）
  - [ ] グレースフルデグラデーション設計
- [ ] セキュリティヘッダーとCSP実装設計書の作成
  - [ ] Badge SVG用の完全なCSPポリシー
  - [ ] エンドポイント別CORS設定
  - [ ] XSS/CSRF防御実装

## 実装タスク (Implementation Tasks)

- [ ] Initial project setup
- [ ] Add documentation
- [ ] Write tests

## ICEBOX

## COMPLETED
