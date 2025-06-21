# GitPoke 🫱🔥

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

GitPoke helps developers stay motivated by displaying GitHub activity badges. When someone hasn't been coding for a while, visitors can send them a friendly "poke" as a gentle reminder to get back to their projects.

[日本語版 README はこちら](README_ja.md)

## 🎯 Features

- **Activity Badge**: Show your GitHub contribution status in your README
- **Gentle Pokes**: Let visitors send you friendly reminders when you've been away
- **GitHub App Integration**: Secure authentication with minimal permissions
- **Real-time Updates**: Automatic activity status updates
- **Privacy-First**: Uses only public contribution data

## 🚀 Quick Start

### 1. Set up your badge

1. Visit [gitpoke.dev](https://gitpoke.dev)
2. Authenticate with GitHub (one-click GitHub App authorization)
3. Copy your badge URL
4. Add it to your README:

```markdown
![GitPoke](https://gitpoke.dev/badge/your-username.svg)
```

### 2. How it works

- 🟢 **Active** (within 7 days): Shows your normal activity badge
- 🔴 **Inactive** (over 7 days): Badge becomes clickable, allowing visitors to poke you

## 🏗️ Architecture

GitPoke is built with modern edge computing technology:

- **Runtime**: Cloudflare Workers
- **Framework**: Hono
- **Storage**: Cloudflare KV
- **API**: GitHub GraphQL API
- **Auth**: GitHub App with minimal permissions

### Required Permissions

```
Account permissions:
  - Email addresses: Read
  - Profile: Read
Repository permissions:
  - Metadata: Read
```

## 🛠️ Development

### Prerequisites

- Node.js 18+
- Cloudflare account
- GitHub App registration

### Setup

```bash
# Clone the repository
git clone https://github.com/ryo-morimoto/gitpoke.git
cd gitpoke

# Install dependencies
npm install

# Set up environment variables
cp .env.example .env
# Edit .env with your credentials

# Run development server
npm run dev
```

### Environment Variables

```env
GITHUB_APP_ID=your_app_id
GITHUB_APP_PRIVATE_KEY=your_private_key
GITHUB_CLIENT_ID=your_client_id
GITHUB_CLIENT_SECRET=your_client_secret
```

## 📊 Project Status

GitPoke is currently under active development. See [TODO.md](TODO.md) for our roadmap and progress.

### Current Phase: MVP Development

- [x] Project design and architecture
- [ ] GitHub App setup and authentication
- [ ] Badge generation with activity tracking
- [ ] Poke functionality implementation

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Quick Contribution Guide

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔒 Security

For security concerns, please see our [Security Policy](SECURITY.md).

## 📞 Contact

- GitHub Issues: [github.com/ryo-morimoto/gitpoke/issues](https://github.com/ryo-morimoto/gitpoke/issues)
- Twitter: [@your_twitter](https://twitter.com/your_twitter)

## 🙏 Acknowledgments

- Inspired by developers who need gentle reminders to keep coding
- Built for the community that believes in friendly encouragement

---

Made with ❤️ by developers, for developers