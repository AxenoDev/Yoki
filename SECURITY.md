# Security Policy

## Supported Versions

Takumi is under active development. Security fixes are applied to the latest version on the `master` branch.

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in Takumi, please report it responsibly. **Do not open a public GitHub issue** for security-related problems.

Instead, report the vulnerability through one of the following channels:

- **[GitHub Security Advisories](https://github.com/AxenoDev/Takumi/security/advisories/new)** (preferred)
- Open a private security report via GitHub's "Report a vulnerability" button on the [Security tab](https://github.com/AxenoDev/Takumi/security)

If neither option is available, you may open a GitHub issue asking for a private contact method without disclosing any vulnerability details.

### What to Include

Please provide as much information as possible to help us understand and reproduce the issue:

- A description of the vulnerability and its potential impact
- Steps to reproduce the issue
- Affected versions or commits
- Any proof-of-concept code or exploit details (if applicable)
- Suggested fix or mitigation (if you have one)

### Response Timeline

We aim to acknowledge reports within **48 hours** and provide an initial assessment within **7 days**. You will be kept informed of our progress toward a fix.

### Disclosure Policy

- We ask that you do not publicly disclose the vulnerability until a fix has been released and users have had reasonable time to update.
- We will credit reporters in the release notes or advisory, unless you prefer to remain anonymous.
- We follow coordinated disclosure practices and will work with you on an appropriate timeline.

## Security Best Practices for Operators

When running Takumi in production:

- Keep the proxy updated to the latest version
- Do not expose the proxy port to the public internet without additional network-level protections
- Run the proxy with the minimum required system privileges
- Monitor logs for unusual connection patterns

Thank you for helping keep Takumi and its users safe.
