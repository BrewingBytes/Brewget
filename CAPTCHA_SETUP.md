# Captcha Setup Guide

This application uses Cloudflare Turnstile for captcha protection on authentication endpoints (login, register, and forgot password).

## Getting Your Turnstile Keys

1. Sign up for a free Cloudflare account at https://www.cloudflare.com/
2. Navigate to the Turnstile section in your Cloudflare dashboard
3. Create a new site and get your:
   - **Site Key** (public key for frontend)
   - **Secret Key** (private key for backend)

## Configuration

### Frontend Configuration

Set the `VITE_TURNSTILE_SITE_KEY` environment variable:

**Option 1: Using .env file (recommended)**
Create a `.env` file in the `frontend/` directory (or copy `.env.example`):
```bash
VITE_TURNSTILE_SITE_KEY=your_site_key_here
```

**Option 2: Using environment variable**
```bash
export VITE_TURNSTILE_SITE_KEY=your_site_key_here
```

The site key defaults to the test key `1x00000000000000000000AA` if not set.

### Backend Configuration

Set the `TURNSTILE_SECRET` environment variable:

**For Docker Compose:**
Edit `docker-compose.yaml` and update the auth-service environment:
```yaml
environment:
  - TURNSTILE_SECRET=your_secret_key_here
```

**For Kubernetes:**
Edit `k8s/02-secrets.yaml` and update the turnstile-secret value:
```yaml
stringData:
  turnstile-secret: your_secret_key_here
```

The secret is automatically mounted to the auth-service deployment via the `TURNSTILE_SECRET` environment variable.

**For Local Development:**
Set the environment variable:
```bash
export TURNSTILE_SECRET=your_secret_key_here
```

## Testing Keys

Cloudflare provides test keys that always pass or fail for development:

**Always Passes:**
- Site Key: `1x00000000000000000000AA`
- Secret Key: `1x0000000000000000000000000000000AA`

**Always Fails:**
- Site Key: `2x00000000000000000000AB`
- Secret Key: `2x0000000000000000000000000000000AA`

**Always Requires Challenge:**
- Site Key: `3x00000000000000000000FF`
- Secret Key: `3x0000000000000000000000000000000FF`

The default configuration in this repository uses the "Always Passes" test keys.

## Production Setup

⚠️ **Important:** Replace test keys with real Cloudflare Turnstile keys before deploying to production.

1. Get your real keys from the Cloudflare Turnstile dashboard
2. Update the frontend site key in `AuthGlass.vue`
3. Update the backend secret key in your environment configuration
4. Restart your services

## Troubleshooting

### Captcha verification fails

- Check that the secret key matches the site key
- Ensure the secret key is correctly set in the backend environment
- Verify that the backend can reach `https://challenges.cloudflare.com`

### Widget not displaying

- Verify the site key is correct in the frontend
- Check browser console for any JavaScript errors
- Ensure `vue-turnstile` package is installed

### Widget appears but doesn't work

- Clear browser cache and cookies
- Try using test keys to isolate the issue
- Check network tab to see if verification requests are being made

## Documentation

- [Cloudflare Turnstile Docs](https://developers.cloudflare.com/turnstile/)
- [vue-turnstile Package](https://www.npmjs.com/package/vue-turnstile)
