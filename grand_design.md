# GitPoke - グランドデザイン設計

## 🎯 サービス概要と設計思想

### **コアコンセプト**
**GitPoke** は開発者同士のゆるい絆を繋ぐサービス。転職や環境変化で直接連絡を取りにくくなった開発者仲間のGitHub活動をチェックし、長期間活動していない場合に「開発してなくね？うぉぅうぉ」のノリで軽くつっついて、開発への復帰を促す。

### **設計思想**
- **最小摩擦**: 認証は必要だが、ワンクリックで完了する体験
- **軽いコミュニケーション**: 重たい連絡手段ではなく、気軽なつっつき
- **段階的エンゲージメント**: Badge表示→つっつき体験→通知設定の自然な流れ

---

## 🔄 ユーザーフロー設計

### **設計意図**: GitHub.comから離れない自然な体験

#### **設定する人（Badge作成者）**
```
gitpoke.dev → GitHub App認証 → Badge URL取得 → README貼り付け
```
**意図**: GitHub App認証で最小権限かつ安全にBadgeを作成

#### **つつく人（閲覧者）**
```
GitHub User Page → Badge発見 → クリック → 即座にフィードバック
```
**意図**: GitHub.com上で完結、ページ遷移による離脱を防ぐ

### **技術的実現方法**
- **SVG内JavaScript**: Badge内に実行ロジック埋め込み
- **CORS対応API**: GitHub.comからの直接API呼び出し
- **オーバーレイ通知**: GitHub.com上に非侵入的なフィードバック表示

---

## 🏗️ アーキテクチャ設計

### **設計意図**: セキュリティと効率性の両立

#### **GitHub App戦略採用**
**なぜ**: OAuth Appの制約（読み書き両方の権限）を回避し、最小権限で安全に運用

```typescript
// 必要最小権限
Account permissions:
  - Email addresses: Read
  - Profile: Read
Repository permissions:
  - Metadata: Read
```

#### **GraphQL API活用**
**なぜ**: REST APIでは取得困難なContributionCalendar情報を効率的に取得

```graphql
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
```

#### **Repository Pattern採用**
**なぜ**: 実装切り替えを容易にし、将来のスケール対応

```typescript
interface GitHubRepository {
  getDaysSinceLastActivity(username: string): Promise<number>;
}

// GitHub App User Access Token使用
const githubRepo = new GitHubAppRepository(env);
```

---

## 🚀 段階的実装戦略

### **設計意図**: リスク最小化と早期価値提供

#### **Phase 1: MVP（2週間）**
**目標**: GitHub App認証とBadge機能を完全動作

- GitHub App作成・設定
- User Access Token生成フロー
- GraphQL APIによるContribution取得
- SVG Badge生成

**なぜこの順序**: 最小機能で価値検証、GitHub Appの複雑さに早期対応

#### **Phase 2: つっつき機能（1週間）**
**目標**: GitHub.com上完結のつっつき体験

- POKE API（CORS対応）
- 通知設定（KV保存）
- Webhook処理（認証取り消し対応）

**なぜこの機能**: サービスの差別化要素、エンゲージメント向上

#### **Phase 3: スケール対応（必要時）**
**移行判断基準**: 
- 月間ユーザー > 100人 OR
- User Access Token管理の複雑化

**対応内容**: Token refresh機能、キャッシュ最適化、監視強化

---

## 🎨 UI/UX設計原則

### **設計意図**: 開発者にとって自然で楽しい体験

#### **Badge表示**
```markdown
![GitPoke](https://gitpoke.dev/badge/username.svg)
```

**視覚的区別**:
- 🟢 Active（7日以内）: 通常表示
- 🔴 Inactive（7日以上）: クリック可能、視覚的にアクション喚起

**なぜこの設計**: 一目で状態把握、アクション可能性の明示

#### **認証体験**
```
Badge作成 → GitHub App認証（ワンクリック） → 即座にBadge生成
```

**なぜページ遷移最小**: GitHub開発者の日常的なワークフローを妨げない

---

## 📊 技術スタック選択理由

### **Runtime**: Cloudflare Workers
**なぜ**: エッジ分散でグローバル低遅延、GitHub App認証との相性良好

### **Framework**: Hono
**なぜ**: Cloudflare Workers最適化、TypeScript完全対応、軽量高速

### **Storage**: Cloudflare KV
**なぜ**: User Access Token管理に最適、グローバル分散、TTL対応

### **API Strategy**: GitHub GraphQL API
**なぜ**: ContributionCalendar取得に必須、効率的なデータ取得

---

## 🔒 セキュリティと制約

### **User Access Token管理**
**課題**: 8時間で期限切れ、refresh token必要
**対策**: 
- 自動refresh機能
- Token無効化時のWebhook対応
- 暗号化保存

**なぜこの対策**: ユーザー体験を損なわずセキュリティ確保

### **Rate Limiting対策**
**制限**: 5,000 points/hour（認証済みユーザー）
**対策**: 
- 適応的キャッシュ（非アクティブユーザーほど長期キャッシュ）
- Public APIフォールバック

**なぜこの対策**: サービス安定性とコスト効率の両立

---

## 💰 ビジネス戦略

### **設計意図**: 持続可能な成長モデル

#### **段階的マネタイゼーション**
1. **Phase 1-2**: 完全無料（ユーザー獲得優先）
2. **Phase 3**: Freemium導入

**なぜ段階的**: 価値証明後のマネタイゼーション、GitHub App認証の摩擦を価値で相殺

#### **収益モデル**
- **Free**: 基本機能、GitHub App認証
- **Pro ($3/month)**: 高度な統計、カスタマイゼーション、Webhook通知
- **Enterprise**: 組織管理、分析ダッシュボード

**なぜこの価格設定**: 開発者向け適正価格、GitHub App運用コストを考慮

---

## 🎯 成功指標とKPI

### **Phase 1目標**
- 週間Badge作成数 > 30（GitHub App認証の摩擦を考慮）
- User Access Token生成成功率 > 95%
- Badge表示成功率 > 99%

### **Phase 2目標**
- 月間つっつき実行数 > 150
- Token refresh成功率 > 98%
- 通知設定完了率 > 70%

### **Phase 3移行判断**
- 月間アクティブユーザー > 100人
- Token管理の運用負荷増大

**なぜこの指標**: GitHub App特有の複雑さと事業成長のバランス点

---

## 🔮 将来展望

### **技術進化シナリオ**
1. **Organization対応**: チーム機能、組織レベルの活動監視
2. **Real-time通知**: Webhook活用、即座のつっつき通知
3. **マルチプラットフォーム**: GitLab、Bitbucket対応

### **機能拡張可能性**
- 詳細分析ダッシュボード
- GitHub Actions連携
- Slack/Discord統合

**なぜ段階的**: 現在の価値証明→需要確認→投資の順序

---

## 📋 設計原則まとめ

### **技術原則**
1. **セキュリティファースト**: GitHub App最小権限、User Access Token適切管理
2. **GitHub.com内完結**: 自然なユーザー体験
3. **段階的複雑化**: 必要性確認後の機能追加

### **事業原則**
1. **価値証明優先**: GitHub App認証の摩擦を上回る価値提供
2. **開発者ファースト**: 技術者に愛される体験
3. **持続可能性**: 運用コストを考慮した現実的な収益モデル

GitPokeは**GitHub App認証による安全性**と**開発者同士の絆**を両立し、**個人開発の継続**を支援するサービスです 🫱🔥
