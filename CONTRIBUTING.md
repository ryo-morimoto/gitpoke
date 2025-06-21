# Contributing to GitPoke 🤝

Thank you for your interest in contributing to GitPoke! We welcome contributions from developers of all skill levels.

## 🚀 Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/gitpoke.git`
3. Create a branch: `git checkout -b {feat,fix,docs,refactor,test,chore}/your-branch-name`
4. Make your changes
5. Test your changes
6. Commit and push
7. Create a Pull Request

## 📋 Development Setup

### Prerequisites

- Node.js 18+ 
- Cloudflare account (for local development)
- GitHub App registration (see setup guide)

### Local Setup

```bash
# Install dependencies
npm install

# Copy environment variables
cp .env.example .env
# Edit .env with your credentials

# Start development server
npm run dev
```

### Environment Variables

```env
GITHUB_APP_ID=your_app_id
GITHUB_APP_PRIVATE_KEY=your_private_key
GITHUB_CLIENT_ID=your_client_id
GITHUB_CLIENT_SECRET=your_client_secret
```

## 🎯 Areas for Contribution

### High Priority
- [ ] GitHub App authentication flow
- [ ] Badge generation with SVG optimization
- [ ] Poke functionality implementation
- [ ] Error handling and resilience patterns

### Medium Priority
- [ ] Caching strategies
- [ ] Rate limiting implementation
- [ ] Security headers and CSP
- [ ] Monitoring and observability

### Low Priority
- [ ] UI/UX improvements
- [ ] Documentation enhancements
- [ ] Internationalization
- [ ] Performance optimizations

## 📝 Code Guidelines

### Code Style

- Use TypeScript for all new code
- Follow existing code conventions
- Use meaningful variable and function names
- Add comments for complex logic

### Architecture Principles

- **Security First**: Always consider security implications
- **Edge-First**: Optimize for Cloudflare Workers environment  
- **Minimal Permissions**: Use least privilege principle
- **Error Resilience**: Handle failures gracefully

### Testing

```bash
# Run tests
npm test

# Run linting
npm run lint

# Type checking
npm run type-check
```

## 🔄 Pull Request Process

### Before Submitting

1. **Test your changes** locally
2. **Run linting** and fix any issues
3. **Update documentation** if needed
4. **Check TODO.md** and update related tasks (for agentic development)

### PR Guidelines

- **Clear title**: Describe what your PR does
- **Detailed description**: Explain the problem and solution
- **Link issues**: Reference related issues
- **Screenshots**: Include visuals for UI changes
- **Test plan**: Describe how you tested your changes

### PR Template

see [PR Template](.github/pull_request_template.md)

## 🐛 Bug Reports

### Before Reporting

1. Check existing issues
2. Test with latest version
3. Gather reproduction steps

### Bug Report Template

see [Bug Report Template](.github/ISSUE_TEMPLATE/bug_report.md)

## 💡 Feature Requests

We welcome feature suggestions! Please:

1. **Check existing issues** first
2. **Describe the problem** you're trying to solve
3. **Propose a solution** if you have one
4. **Consider alternatives** and their trade-offs

### Feature Request Template

see [Feature Request Template](.github/ISSUE_TEMPLATE/feature_request.md)

## 🏗️ Architecture Overview

### Tech Stack
- **Runtime**: Cloudflare Workers
- **Framework**: Hono  
- **Storage**: Cloudflare KV
- **API**: GitHub GraphQL API
- **Auth**: GitHub App

### Key Components
- **Badge Service**: SVG generation and caching
- **GitHub Integration**: API calls and webhooks
- **Poke System**: User interactions and notifications
- **Auth Service**: GitHub App token management

## 📚 Resources

- [GitHub App Documentation](https://docs.github.com/en/apps)
- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [Hono Framework](https://hono.dev/)

## 🔒 Security

- **No secrets in code**: Use environment variables
- **Validate inputs**: Always sanitize user data
- **Rate limiting**: Implement appropriate limits
- **Error handling**: Don't leak sensitive information

For security vulnerabilities, see [Security Policy](SECURITY.md).

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion  
- **Documentation**: Check the `/docs` directory

## 🙏 Recognition

Contributors will be:
- Listed in our README
- Mentioned in release notes
- Invited to our contributor Discord (coming soon)

Thank you for making GitPoke better! 🚀