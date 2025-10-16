# Passkey Authentication Implementation

This document provides details about the passkey (WebAuthn) authentication implementation in BrewGet.

## Overview

BrewGet now supports passwordless authentication using passkeys (WebAuthn). Users can:
- Create accounts with **only a passkey** (no password required)
- Create accounts with **both password and passkey**
- Login using either password or passkey

## Architecture

### Backend (Rust)

The backend uses the `webauthn-rs` library to implement the WebAuthn Relying Party (RP) server.

#### Key Components:

1. **Database Schema**
   - Modified `users` table: `password` field is now nullable
   - New `passkey_credentials` table stores WebAuthn credentials

2. **API Endpoints**
   - `POST /register/passkey/start` - Initiates passkey registration
   - `POST /register/passkey/finish` - Completes passkey registration
   - `POST /login/passkey/start` - Initiates passkey authentication
   - `POST /login/passkey/finish` - Completes passkey authentication
   - `POST /register` - Updated to support optional password field

3. **Models**
   - `PasskeyCredential` - Database model for storing credentials
   - `NewPasskeyCredential` - Model for inserting new credentials
   - Updated `User` and `NewUser` to support optional passwords

### Frontend (Vue.js)

The frontend uses `@simplewebauthn/browser` to handle browser-side WebAuthn operations.

#### Key Components:

1. **WebAuthn Utility** (`src/utils/webauthn.ts`)
   - Browser capability detection
   - Passkey registration wrapper
   - Passkey authentication wrapper

2. **Auth Store** (`src/stores/auth.ts`)
   - `registerWithPasskey()` - Handles passkey registration flow
   - `loginWithPasskey()` - Handles passkey authentication flow

3. **UI Components**
   - Updated `AuthGlass.vue` to include passkey checkbox option
   - Conditional display of password field based on passkey selection

## Configuration

### Environment Variables

Add the following environment variables to your auth-service configuration:

#### For Docker Deployment (using docker-compose)
```bash
# WebAuthn Relying Party Configuration
RP_ID=localhost                      # For development. Use your domain in production (e.g., "example.com")
RP_ORIGIN=http://localhost           # Frontend URL through nginx (port 80)
```

#### For Local Development (services running directly)
```bash
# WebAuthn Relying Party Configuration
RP_ID=localhost                      # For development. Use your domain in production (e.g., "example.com")
RP_ORIGIN=http://localhost:5173      # Frontend dev server URL (typically Vite default port)
```

**Important Notes:**
- `RP_ID` should be the domain name without protocol or port
- `RP_ORIGIN` must include the protocol and match exactly where the frontend is hosted
- For production, use your actual domain (e.g., `RP_ID=brewget.com`, `RP_ORIGIN=https://brewget.com`)
- The `RP_ORIGIN` must match what appears in the browser's address bar when accessing the app

### Database Migration

The database migration (`2025-10-16-160000_add_passkey_support`) will:
1. Make the `password` column in `users` table nullable
2. Create the `passkey_credentials` table
3. Add necessary indexes

Run migrations with:
```bash
cd backend/auth-service
diesel migration run
```

## User Flows

### Registration with Passkey

1. User enters username and email
2. User checks "Register with passkey only" checkbox
3. User clicks "Register with Passkey"
4. Backend generates WebAuthn challenge
5. Browser prompts user to create passkey (biometric, security key, etc.)
6. Credential is stored in database
7. User receives confirmation email (if email verification is enabled)

### Login with Passkey

1. User enters username
2. User checks "Sign in with passkey" checkbox
3. User clicks "Sign In with Passkey"
4. Backend generates authentication challenge
5. Browser prompts user to authenticate with their passkey
6. Upon success, user receives JWT token and is logged in

### Registration with Password (Traditional)

1. User enters username, email, and password
2. User clicks "Register" (passkey checkbox unchecked)
3. Account created with password
4. User can optionally add a passkey later (future enhancement)

## Security Considerations

1. **Passkey Storage**: Credentials are stored securely in the database with:
   - Credential ID (unique identifier)
   - Public key (for verification)
   - Counter (for replay attack prevention)

2. **Challenge-Response**: Each authentication attempt uses a unique challenge to prevent replay attacks

3. **Origin Validation**: WebAuthn automatically validates that credentials can only be used on the registered origin

4. **User Verification**: Passkeys require user verification (biometric, PIN, etc.) by default

## Browser Compatibility

Passkeys are supported in:
- Chrome/Edge 67+
- Firefox 60+
- Safari 13+
- Opera 54+

The frontend automatically detects browser support and only shows the passkey option when supported.

## Troubleshooting

### Common Issues

1. **"WebAuthn is not supported"**
   - Ensure you're using HTTPS (or localhost for development)
   - Check browser compatibility
   - Verify browser has WebAuthn enabled

2. **Registration fails with origin mismatch**
   - Ensure `RP_ORIGIN` matches the exact URL in the browser (including protocol and port)
   - For Docker deployment: Use `http://localhost` (nginx proxies on port 80)
   - For local development: Use `http://localhost:5173` (or your Vite dev server port)
   - Check that `RP_ID` matches the domain (should be just `localhost` for local dev)

3. **Authentication fails**
   - Verify the credential was registered for this user
   - Check that the credential hasn't been deleted from the authenticator
   - Ensure counter validation is working properly

## Future Enhancements

Potential improvements for the passkey implementation:

1. **Add Passkey to Existing Account**: Allow users to add passkeys to password-protected accounts
2. **Multiple Passkeys**: Support multiple passkeys per user (different devices)
3. **Passkey Management UI**: Allow users to view and delete their registered passkeys
4. **Credential Naming**: Let users name their passkeys (e.g., "iPhone", "YubiKey")
5. **Resident Keys**: Support for discoverable credentials (no username needed)
6. **Attestation**: Store and verify attestation data for enhanced security

## Testing

To test the passkey implementation:

1. Start the backend with proper environment variables
2. Start the frontend
3. Navigate to registration page
4. Try registering with passkey enabled
5. Verify passkey creation prompt appears
6. Complete registration
7. Try logging in with the passkey
8. Verify authentication works

**Note**: Testing requires an actual browser and authenticator. Automated testing of WebAuthn flows is complex and typically requires specialized tools like virtual authenticators.

## References

- [WebAuthn Specification](https://www.w3.org/TR/webauthn/)
- [webauthn-rs Documentation](https://docs.rs/webauthn-rs/)
- [SimpleWebAuthn Documentation](https://simplewebauthn.dev/)
- [Passkeys.dev](https://passkeys.dev/) - Comprehensive guide to passkeys
