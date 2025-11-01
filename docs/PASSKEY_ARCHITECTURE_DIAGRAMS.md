# Passkey Authentication - Architecture Diagrams

## System Architecture

```
┌───────────────────────────────────────────────────────────────────┐
│                         User's Device                             │
├───────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────┐         ┌──────────────────┐              │
│  │   Web Browser   │         │  Authenticator   │              │
│  │   (Vue.js App)  │◄────────┤  (Face ID/       │              │
│  │                 │         │   Touch ID/      │              │
│  │  - Registration │         │   Windows Hello) │              │
│  │  - Login        │         │                  │              │
│  │  - Settings     │         │  Private Key     │              │
│  └────────┬────────┘         │  Storage         │              │
│           │                  └──────────────────┘              │
└───────────┼───────────────────────────────────────────────────────┘
            │ HTTPS/JSON
            │ WebAuthn API calls
            ↓
┌───────────────────────────────────────────────────────────────────┐
│                      Backend (Rust/Axum)                          │
├───────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │             Auth Service Routes                           │   │
│  │                                                           │   │
│  │  /auth/passkey/register/start  POST                      │   │
│  │  /auth/passkey/register/finish POST                      │   │
│  │  /auth/passkey/login/start     POST                      │   │
│  │  /auth/passkey/login/finish    POST                      │   │
│  │  /auth/login                   POST (password fallback)  │   │
│  │  /auth/register                POST (password fallback)  │   │
│  └──────────────────┬───────────────────────────────────────┘   │
│                     │                                            │
│  ┌──────────────────▼───────────────────────────────────────┐   │
│  │           WebAuthn Service (webauthn-rs)                 │   │
│  │                                                           │   │
│  │  - Challenge Generation                                  │   │
│  │  - Credential Verification                               │   │
│  │  - Counter Validation                                    │   │
│  │  - Origin Verification                                   │   │
│  └──────────────────┬───────────────────────────────────────┘   │
│                     │                                            │
│  ┌──────────────────▼───────────────────────────────────────┐   │
│  │              Database Layer                              │   │
│  │                                                           │   │
│  │  - User Management                                       │   │
│  │  - Passkey Credentials Storage                           │   │
│  │  - Token Management                                      │   │
│  │  - Audit Logging                                         │   │
│  └──────────────────┬───────────────────────────────────────┘   │
└────────────────────┼──────────────────────────────────────────────┘
                     │
                     ↓
┌───────────────────────────────────────────────────────────────────┐
│                    PostgreSQL Database                            │
├───────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────────┐  ┌────────────────────┐  ┌─────────────────┐ │
│  │    users     │  │ passkey_credentials│  │     tokens      │ │
│  │              │  │                    │  │                 │ │
│  │ - id         │  │ - id               │  │ - id            │ │
│  │ - username   │  │ - user_id (FK)     │  │ - user_id (FK)  │ │
│  │ - password   │  │ - credential_id    │  │ - token         │ │
│  │   (nullable) │  │ - public_key       │  │ - expires_at    │ │
│  │ - email      │  │ - counter          │  └─────────────────┘ │
│  │ - has_passkey│  │ - device_name      │                      │
│  └──────────────┘  │ - created_at       │  ┌─────────────────┐ │
│                    │ - last_used_at     │  │ audit_log       │ │
│                    └────────────────────┘  │                 │ │
│                                            │ - user_id       │ │
│                                            │ - auth_method   │ │
│                                            │ - success       │ │
│                                            │ - timestamp     │ │
│                                            └─────────────────┘ │
└───────────────────────────────────────────────────────────────────┘
```

## Registration Flow (Passkey)

```
┌─────────┐                 ┌─────────┐                 ┌──────────┐
│ Browser │                 │ Backend │                 │ Database │
└────┬────┘                 └────┬────┘                 └────┬─────┘
     │                           │                           │
     │ 1. POST /passkey/         │                           │
     │    register/start         │                           │
     ├──────────────────────────►│                           │
     │ {username, email,         │                           │
     │  captchaToken}            │                           │
     │                           │                           │
     │                           │ 2. Verify captcha         │
     │                           │    Validate inputs        │
     │                           │    Check uniqueness       │
     │                           ├──────────────────────────►│
     │                           │                           │
     │                           │ 3. Generate challenge     │
     │                           │    Store temp state       │
     │                           │                           │
     │ 4. Challenge response     │                           │
     │◄──────────────────────────┤                           │
     │ {userId, creationOptions} │                           │
     │                           │                           │
     │ 5. Create credential      │                           │
     │    (WebAuthn API)         │                           │
     │    - User authenticates   │                           │
     │    - Generate key pair    │                           │
     │    - Store private key    │                           │
     │      on device            │                           │
     │                           │                           │
     │ 6. POST /passkey/         │                           │
     │    register/finish        │                           │
     ├──────────────────────────►│                           │
     │ {userId, credential,      │                           │
     │  deviceName}              │                           │
     │                           │                           │
     │                           │ 7. Verify credential      │
     │                           │    signature              │
     │                           │                           │
     │                           │ 8. Create user + passkey  │
     │                           │    in transaction         │
     │                           ├──────────────────────────►│
     │                           │                           │
     │                           │ 9. Send activation email  │
     │                           │                           │
     │ 10. Success response      │                           │
     │◄──────────────────────────┤                           │
     │ {translation_key:         │                           │
     │  "ACCOUNT_CREATED"}       │                           │
     │                           │                           │
```

## Login Flow (Passkey)

```
┌─────────┐                 ┌─────────┐                 ┌──────────┐
│ Browser │                 │ Backend │                 │ Database │
└────┬────┘                 └────┬────┘                 └────┬─────┘
     │                           │                           │
     │ 1. POST /passkey/         │                           │
     │    login/start            │                           │
     ├──────────────────────────►│                           │
     │ {username, captchaToken}  │                           │
     │                           │                           │
     │                           │ 2. Verify captcha         │
     │                           │    Find user              │
     │                           ├──────────────────────────►│
     │                           │                           │
     │                           │ 3. Get user's passkeys    │
     │                           │◄──────────────────────────┤
     │                           │                           │
     │                           │ 4. Generate challenge     │
     │                           │    Store temp state       │
     │                           │                           │
     │ 5. Challenge response     │                           │
     │◄──────────────────────────┤                           │
     │ {requestOptions}          │                           │
     │                           │                           │
     │ 6. Authenticate           │                           │
     │    (WebAuthn API)         │                           │
     │    - User authenticates   │                           │
     │    - Sign challenge       │                           │
     │      with private key     │                           │
     │                           │                           │
     │ 7. POST /passkey/         │                           │
     │    login/finish           │                           │
     ├──────────────────────────►│                           │
     │ {username, credential}    │                           │
     │                           │                           │
     │                           │ 8. Verify signature       │
     │                           │    Validate counter       │
     │                           │                           │
     │                           │ 9. Update counter         │
     │                           │    Generate JWT token     │
     │                           ├──────────────────────────►│
     │                           │                           │
     │ 10. Token response        │                           │
     │◄──────────────────────────┤                           │
     │ {token: "eyJ..."}         │                           │
     │                           │                           │
     │ 11. Store token           │                           │
     │     Redirect to dashboard │                           │
     │                           │                           │
```

## Password Fallback Flow

```
┌─────────┐                 ┌─────────┐                 ┌──────────┐
│ Browser │                 │ Backend │                 │ Database │
└────┬────┘                 └────┬────┘                 └────┬─────┘
     │                           │                           │
     │ User clicks               │                           │
     │ "Use password instead"    │                           │
     │                           │                           │
     │ 1. POST /auth/login       │                           │
     ├──────────────────────────►│                           │
     │ {username, password,      │                           │
     │  captchaToken}            │                           │
     │                           │                           │
     │                           │ 2. Verify captcha         │
     │                           │    Find user              │
     │                           ├──────────────────────────►│
     │                           │                           │
     │                           │ 3. Verify password hash   │
     │                           │    Generate JWT token     │
     │                           ├──────────────────────────►│
     │                           │                           │
     │ 4. Token response         │                           │
     │◄──────────────────────────┤                           │
     │ {token: "eyJ..."}         │                           │
     │                           │                           │
     │ 5. Store token            │                           │
     │    Redirect to dashboard  │                           │
     │                           │                           │
     │ 6. Show prompt:           │                           │
     │    "Want faster login?    │                           │
     │     Add a passkey"        │                           │
     │                           │                           │
```

## Recovery Flow (Lost Passkey - Email OTP)

```
┌─────────┐                 ┌─────────┐                 ┌──────────┐
│ Browser │                 │ Backend │                 │ Email    │
└────┬────┘                 └────┬────┘                 └────┬─────┘
     │                           │                           │
     │ User clicks               │                           │
     │ "Having trouble?"         │                           │
     │                           │                           │
     │ 1. POST /auth/recovery/   │                           │
     │    request-otp            │                           │
     ├──────────────────────────►│                           │
     │ {email}                   │                           │
     │                           │                           │
     │                           │ 2. Generate 6-digit OTP   │
     │                           │    Store with expiry      │
     │                           │                           │
     │                           │ 3. Send OTP email         │
     │                           ├──────────────────────────►│
     │                           │                           │
     │ 2. OTP sent confirmation  │                           │
     │◄──────────────────────────┤                           │
     │                           │                           │
     │ 3. User receives email    │                           │
     │    Enters OTP code        │                           │
     │                           │                           │
     │ 4. POST /auth/recovery/   │                           │
     │    verify-otp             │                           │
     ├──────────────────────────►│                           │
     │ {email, otp}              │                           │
     │                           │                           │
     │                           │ 5. Verify OTP             │
     │                           │    Generate JWT token     │
     │                           │                           │
     │ 6. Token response +       │                           │
     │    Prompt to add passkey  │                           │
     │◄──────────────────────────┤                           │
     │                           │                           │
```

## Data Model

```
┌─────────────────────────────────────────────────────────────────┐
│                          users                                  │
├─────────────────────────────────────────────────────────────────┤
│ id (UUID, PK)                                                   │
│ username (VARCHAR(50), UNIQUE, NOT NULL)                        │
│ password (TEXT, NULLABLE) ◄─── Made optional for passkey-only  │
│ email (VARCHAR(255), UNIQUE, NOT NULL)                          │
│ has_passkey (BOOLEAN, DEFAULT FALSE) ◄─── New field            │
│ is_verified (BOOLEAN, DEFAULT FALSE)                            │
│ is_active (BOOLEAN, DEFAULT TRUE)                               │
│ created_at (TIMESTAMPTZ)                                        │
│ updated_at (TIMESTAMPTZ)                                        │
│ last_login_at (TIMESTAMPTZ)                                     │
└────────────────────┬────────────────────────────────────────────┘
                     │
                     │ 1:N
                     │
┌────────────────────▼────────────────────────────────────────────┐
│                   passkey_credentials                           │
├─────────────────────────────────────────────────────────────────┤
│ id (UUID, PK)                                                   │
│ user_id (UUID, FK → users.id)                                   │
│ credential_id (BYTEA, UNIQUE, NOT NULL)                         │
│ public_key (BYTEA, NOT NULL)                                    │
│ counter (BIGINT, NOT NULL, DEFAULT 0) ◄─── Anti-replay         │
│ aaguid (BYTEA)                                                  │
│ credential_device_type (TEXT)                                   │
│ credential_backed_up (BOOLEAN)                                  │
│ device_name (TEXT) ◄─── User-friendly name                     │
│ user_agent (TEXT)                                               │
│ created_at (TIMESTAMPTZ)                                        │
│ last_used_at (TIMESTAMPTZ)                                      │
│ is_active (BOOLEAN, DEFAULT TRUE)                               │
└─────────────────────────────────────────────────────────────────┘

Notes:
- password field is nullable to support passkey-only accounts
- has_passkey flag enables quick checks without joining tables
- counter field prevents replay attacks (must increment)
- Multiple passkeys per user supported for multi-device usage
```

## Security Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Security Layers                              │
└─────────────────────────────────────────────────────────────────┘

Layer 1: Transport Security
┌─────────────────────────────────────────────────────────────────┐
│  HTTPS/TLS 1.3                                                  │
│  - Required for WebAuthn                                        │
│  - Protects data in transit                                     │
│  - Certificate validation                                       │
└─────────────────────────────────────────────────────────────────┘

Layer 2: Origin Validation
┌─────────────────────────────────────────────────────────────────┐
│  WebAuthn Origin Binding                                        │
│  - Credentials bound to specific origin                         │
│  - Prevents phishing attacks                                    │
│  - Automatic browser enforcement                                │
└─────────────────────────────────────────────────────────────────┘

Layer 3: Cryptographic Security
┌─────────────────────────────────────────────────────────────────┐
│  Public Key Cryptography                                        │
│  - Private key never leaves device                              │
│  - Challenge-response authentication                            │
│  - Signature verification on server                             │
│  - Counter-based replay protection                              │
└─────────────────────────────────────────────────────────────────┘

Layer 4: Application Security
┌─────────────────────────────────────────────────────────────────┐
│  Backend Validation                                             │
│  - Input sanitization                                           │
│  - Rate limiting (5 attempts/min)                               │
│  - CAPTCHA verification                                         │
│  - SQL injection prevention (parameterized queries)             │
└─────────────────────────────────────────────────────────────────┘

Layer 5: Monitoring & Audit
┌─────────────────────────────────────────────────────────────────┐
│  Security Logging                                               │
│  - All authentication attempts logged                           │
│  - Failed login monitoring                                      │
│  - Anomaly detection                                            │
│  - Alert on suspicious patterns                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Browser Capability Detection

```
┌─────────────────────────────────────────────────────────────────┐
│                    On Page Load                                 │
└─────────────────────────────────────────────────────────────────┘
                          │
                          ▼
              ┌──────────────────────┐
              │ Check if             │
              │ window.PublicKey     │
              │ Credential exists?   │
              └──────────┬───────────┘
                         │
                    ┌────┴────┐
                    │         │
                   YES       NO
                    │         │
                    ▼         ▼
        ┌─────────────────┐  ┌─────────────────┐
        │ Check platform  │  │ Show password-  │
        │ authenticator   │  │ only UI         │
        │ available?      │  │                 │
        └────────┬────────┘  │ No passkey      │
                 │           │ features        │
            ┌────┴────┐      └─────────────────┘
            │         │
           YES       NO
            │         │
            ▼         ▼
┌──────────────────┐  ┌──────────────────┐
│ Show passkey-    │  │ Show passkey UI  │
│ first UI         │  │ with security    │
│                  │  │ key option       │
│ Recommend Face   │  │                  │
│ ID/Touch ID      │  │ (USB/NFC keys)   │
└──────────────────┘  └──────────────────┘
```

## Deployment Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        CDN / Edge                               │
│                   (Static Assets)                               │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Load Balancer                               │
│                   (SSL Termination)                             │
└─────────────────────────┬───────────────────────────────────────┘
                          │
              ┌───────────┴───────────┐
              │                       │
              ▼                       ▼
┌──────────────────────┐    ┌──────────────────────┐
│  Auth Service Pod 1  │    │  Auth Service Pod 2  │
│                      │    │                      │
│  - WebAuthn Server   │    │  - WebAuthn Server   │
│  - Session Cache     │    │  - Session Cache     │
│  - API Endpoints     │    │  - API Endpoints     │
└──────────┬───────────┘    └───────────┬──────────┘
           │                            │
           └────────────┬───────────────┘
                        │
                        ▼
           ┌────────────────────────┐
           │   Redis / In-Memory    │
           │   Challenge Storage    │
           │   (5 min TTL)          │
           └────────────┬───────────┘
                        │
                        ▼
           ┌────────────────────────┐
           │    PostgreSQL          │
           │    (Primary + Replica) │
           │                        │
           │  - users               │
           │  - passkey_credentials │
           │  - audit_log           │
           └────────────────────────┘
```

---

These diagrams provide visual representations of:
1. Overall system architecture
2. Registration and login flows
3. Recovery mechanisms
4. Data models and relationships
5. Security layers
6. Browser capability detection
7. Deployment architecture

Use these diagrams alongside the other documentation for a complete understanding of the passkey authentication system.
