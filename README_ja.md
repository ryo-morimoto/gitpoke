# GitPoke 🫱🔥

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

GitPokeは、開発者の活動を可視化し、お互いを励まし合うためのサービスです。GitHubの活動状況をバッジで表示し、しばらく活動がない開発者に「最近コード書いてる？」と気軽につっつける機能を提供します。

[English README is here](README.md)

## 🎯 機能

- **アクティビティバッジ**: GitHubの活動状況をREADMEにバッジとして表示
- **つっつき機能**: しばらく活動がない開発者に気軽にリマインダーを送信
- **GitHub App統合**: 必要最小限の権限で安全に認証
- **自動更新**: アクティビティステータスをリアルタイムで更新
- **プライバシー保護**: 公開データのみを使用

## 🚀 クイックスタート

### 1. バッジの設定

1. [gitpoke.dev](https://gitpoke.dev)にアクセス
2. GitHubアカウントで認証（ワンクリック）
3. 生成されたバッジURLをコピー
4. あなたのREADMEに追加:

```markdown
![GitPoke](https://gitpoke.dev/badge/your-username.svg)
```

### 2. 動作の仕組み

- 🟢 **アクティブ**（7日以内）: 通常のバッジを表示
- 🔴 **非アクティブ**（7日以上）: バッジがクリック可能になり、つっつきを送信できます

## 🏗️ アーキテクチャ

GitPokeは最新のエッジコンピューティング技術で構築されています：

- **ランタイム**: Cloudflare Workers
- **フレームワーク**: Hono
- **ストレージ**: Cloudflare KV
- **API**: GitHub GraphQL API
- **認証**: 最小権限のGitHub App

### 必要な権限

```
アカウント権限:
  - メールアドレス: 読み取り
  - プロフィール: 読み取り
リポジトリ権限:
  - メタデータ: 読み取り
```

## 🛠️ 開発

### 前提条件

- Node.js 18以上
- Cloudflareアカウント
- GitHub App登録

### セットアップ

```bash
# リポジトリをクローン
git clone https://github.com/ryo-morimoto/gitpoke.git
cd gitpoke

# 依存関係をインストール
npm install

# 環境変数を設定
cp .env.example .env
# .envファイルを編集して認証情報を設定

# 開発サーバーを起動
npm run dev
```

### 環境変数

```env
GITHUB_APP_ID=your_app_id
GITHUB_APP_PRIVATE_KEY=your_private_key
GITHUB_CLIENT_ID=your_client_id
GITHUB_CLIENT_SECRET=your_client_secret
```

## 📊 プロジェクトステータス

GitPokeは現在開発中です。ロードマップと進捗は[TODO.md](TODO.md)をご覧ください。

### 現在のフェーズ: MVP開発

- [x] プロジェクト設計とアーキテクチャ
- [ ] GitHub Appセットアップと認証
- [ ] アクティビティトラッキング付きバッジ生成
- [ ] つっつき機能の実装

## 🤝 コントリビューション

コントリビューションを歓迎します！詳細は[コントリビューションガイドライン](CONTRIBUTING.md)をご覧ください。

### クイックコントリビューションガイド

1. リポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## 📄 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルをご覧ください。

## 🔒 セキュリティ

セキュリティに関する懸念事項については、[セキュリティポリシー](SECURITY.md)をご覧ください。

## 📞 連絡先

- GitHub Issues: [github.com/ryo-morimoto/gitpoke/issues](https://github.com/ryo-morimoto/gitpoke/issues)
- Twitter: [@your_twitter](https://twitter.com/your_twitter)

## 🙏 謝辞

- コーディングのモチベーション維持に悩む全ての開発者へ
- お互いを励まし合える開発者コミュニティに感謝を込めて

---

Made with ❤️ by developers, for developers
