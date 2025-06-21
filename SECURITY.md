# Security Policy

## Supported Versions

As a personal project, GitPoke follows a simple support policy:

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |
| Others  | Best effort        |

## Reporting a Vulnerability

Security is important to us. If you discover a vulnerability, please report it responsibly.

### How to Report

1. **For critical security issues**: Please use GitHub's private vulnerability reporting feature (Settings → Security → Report a vulnerability)
2. **For minor security concerns**: Open an issue with the `security` label

### What to Include

- **Description** of the vulnerability
- **Steps to reproduce** the issue
- **Potential impact**
- **Suggested fix** (if you have one)

### Response Expectations

As this is a personal project maintained in spare time:

- I'll try to respond within a week
- Critical issues will be prioritized
- Fixes will be implemented as time permits
- Your patience and understanding are appreciated

## Security Best Practices

### For Contributors
- **Never commit secrets**: Check for API keys, tokens, or passwords before committing
- **Validate inputs**: Sanitize all user inputs to prevent injection attacks
- **Update dependencies**: Run `npm audit` regularly and fix vulnerabilities
- **Use environment variables**: Keep sensitive configuration out of code

### For Users
- **Review permissions**: Understand what access you're granting to the GitHub App
- **Revoke access**: Remove the app if you're no longer using it
- **Report issues**: Let us know if you see anything suspicious

## GitHub App Security

GitPoke uses GitHub App authentication with minimal required permissions:

### Required Permissions
```
Account permissions:
  - Email addresses: Read
  - Profile: Read
Repository permissions:
  - Metadata: Read
```

### Security Measures
- **Token Encryption**: All tokens are encrypted at rest
- **Token Rotation**: Automatic token refresh before expiration
- **Webhook Validation**: All webhook payloads are verified
- **Rate Limiting**: Implemented to prevent abuse

## Infrastructure Security

### Cloudflare Workers
- **Edge Security**: Built-in DDoS protection
- **Environment Isolation**: Secure runtime environment
- **Secrets Management**: Environment variables for sensitive data

### Data Storage
- **Cloudflare KV**: Encrypted at rest and in transit
- **TTL Policies**: Automatic data expiration
- **Access Control**: Restricted to authorized services only

## Privacy Considerations

- **Public Data Only**: We only access public GitHub contribution data
- **Minimal Collection**: We collect only necessary information
- **Transparent Usage**: Clear documentation of data usage
- **User Control**: Users can revoke access at any time

## Known Security Considerations

### Current Implementation
- **Public data only**: GitPoke only accesses public GitHub contribution data
- **Minimal permissions**: We request the least amount of access needed
- **Token handling**: User tokens are encrypted and have automatic expiration
- **Rate limiting**: Built-in protection against abuse

### Future Improvements
- Enhanced rate limiting per user
- Additional input validation
- Security headers optimization
- Regular dependency updates

## Acknowledgments

Contributors who help improve GitPoke's security will be thanked in our release notes.

---

**Remember**: Good security practices benefit everyone. Thank you for helping keep GitPoke secure!