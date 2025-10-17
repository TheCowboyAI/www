# SSL Certificates Directory

Place your Cloudflare Origin certificates here:

- `thecowboy.ai.pem` - The certificate file from Cloudflare
- `thecowboy.ai.key` - The private key file from Cloudflare

These certificates are used by nginx in the container build.

## Getting Cloudflare Origin Certificates

1. Log into Cloudflare Dashboard
2. Go to SSL/TLS > Origin Server
3. Create Certificate
4. Save the certificate as `thecowboy.ai.pem`
5. Save the private key as `thecowboy.ai.key`