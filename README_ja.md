# GitPoke 🫱🔥

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

GitPokeは、開発者同士のゆるい絆を繋ぐサービスです。GitHub活動バッジを通じて、転職や環境変化で直接連絡を取りにくくなった開発者仲間の活動をチェックし、長期間活動していない場合に「開発してなくね？うぉぅうぉ」のノリで軽くつっついて、開発への復帰を促します。

[English README is here](README.md)

## 🎯 機能

- **アクティビティバッジ**: GitHubのコントリビューション活動をREADMEにバッジとして表示
- **やさしいつっつき**: 非アクティブな開発者に対して、訪問者がフレンドリーなリマインダーを送信可能
- **GitHub App統合**: 最小権限での安全な認証
- **リアルタイム更新**: アクティビティステータスの自動更新
- **プライバシーファースト**: 公開されているコントリビューションデータのみを使用

## 🚀 クイックスタート

### 1. バッジの設定

1. [gitpoke.dev](https://gitpoke.dev)にアクセス
2. GitHubで認証（ワンクリックでGitHub App認証）
3. バッジURLをコピー
4. READMEに追加:

```markdown
![GitPoke](https://gitpoke.dev/badge/your-username.svg)
```

### 2. 使い方

- 🟢 **アクティブ**（7日以内）: 通常のバッジ表示
- 🔴 **非アクティブ**（7日以上）: バッジがクリック可能になり、つっつきが可能に

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

GitPokeは現在活発に開発中です。現在のロードマップと進捗については[TODO.md](TODO.md)をご覧ください。

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

- やさしいモチベーションを必要とする開発者コミュニティにインスパイアされました
- 時にはフレンドリーなリマインダーが必要な開発者のために、愛を込めて作られました

---

開発者による、開発者のための、❤️を込めて