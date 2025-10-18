# SSL Certificates Directory

This directory should contain SSL certificates for the website, but the actual certificate files are NOT committed to git for security reasons.

## Required Files (NOT in Git)

Place these files here manually or via secure deployment:

- `thecowboy.ai.key` - Private key (NEVER commit this!)
- `thecowboy.ai.pem` - SSL certificate

## Security Notice

⚠️ **NEVER commit private keys or certificates to git!**

These files are:
- Excluded via .gitignore
- Should be deployed separately via secure channels
- Should have restricted file permissions (600)

## Deployment

Certificates should be:
1. Stored securely (e.g., in a password manager or secrets vault)
2. Deployed via secure copy (scp) or configuration management
3. Referenced in NixOS configuration but not embedded

## For Development

For local development, you can:
- Use self-signed certificates
- Use `trunk serve` without HTTPS
- Use a reverse proxy with proper certificates