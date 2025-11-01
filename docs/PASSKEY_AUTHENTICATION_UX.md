# Passkey-First Authentication UX Design

## Overview

This document outlines the user experience design for implementing passkey-first authentication in BrewGet. The design prioritizes passkeys as the primary authentication method while maintaining password fallback for compatibility.

## Design Principles

1. **Passkey First**: Encourage users to create passkeys as the primary authentication method
2. **Progressive Enhancement**: Allow password fallback for users without passkey support
3. **User Choice**: Let users decide their preferred authentication method
4. **Clear Recovery Path**: Provide obvious recovery options for lost passkeys
5. **Backward Compatible**: Support existing password-only users seamlessly

## User Flows

### Flow 1: New User Registration (Passkey First)

```
┌─────────────────────────────────────┐
│   Registration Page (Default)       │
├─────────────────────────────────────┤
│ Welcome to BrewGet!                 │
│                                     │
│ [Email input field]                 │
│ [Username input field]              │
│                                     │
│ ┌─────────────────────────────┐   │
│ │ 🔐 Continue with Passkey    │   │
│ │ (Recommended - More Secure) │   │
│ └─────────────────────────────┘   │
│                                     │
│ Or use a password instead:          │
│ [Show password option]              │
│                                     │
│ [Captcha]                           │
└─────────────────────────────────────┘
           │
           ↓ (User clicks "Continue with Passkey")
┌─────────────────────────────────────┐
│   Passkey Creation Flow             │
├─────────────────────────────────────┤
│ Create Your Passkey                 │
│                                     │
│ A passkey provides quick and secure │
│ access without passwords.           │
│                                     │
│ ✓ Faster login                      │
│ ✓ More secure                       │
│ ✓ No passwords to remember          │
│                                     │
│ [Device authenticator prompt...]    │
│                                     │
│ Optional: Set a backup password     │
│ [ ] Add password for recovery       │
│                                     │
│ [Create Account]                    │
└─────────────────────────────────────┘
           │
           ↓ (Success)
┌─────────────────────────────────────┐
│   Registration Success              │
├─────────────────────────────────────┤
│ ✓ Account Created!                  │
│                                     │
│ Check your email to verify your     │
│ account.                            │
│                                     │
│ [Continue to Login]                 │
└─────────────────────────────────────┘
```

### Flow 2: New User Registration (Password Fallback)

```
┌─────────────────────────────────────┐
│   Registration Page                 │
│   (User clicks "Show password       │
│    option")                         │
├─────────────────────────────────────┤
│ Welcome to BrewGet!                 │
│                                     │
│ [Email input field]                 │
│ [Username input field]              │
│ [Password input field]              │
│ [Confirm Password input field]      │
│                                     │
│ ℹ️ Tip: You can add a passkey       │
│   later for easier sign-in          │
│                                     │
│ [Captcha]                           │
│ [Create Account]                    │
└─────────────────────────────────────┘
           │
           ↓ (Success)
┌─────────────────────────────────────┐
│   Registration Success              │
├─────────────────────────────────────┤
│ ✓ Account Created!                  │
│                                     │
│ Check your email to verify your     │
│ account.                            │
│                                     │
│ [Continue to Login]                 │
└─────────────────────────────────────┘
```

### Flow 3: Existing User Login (Passkey First)

```
┌─────────────────────────────────────┐
│   Login Page (Default)              │
├─────────────────────────────────────┤
│ Welcome back!                       │
│                                     │
│ [Username/Email input field]        │
│                                     │
│ ┌─────────────────────────────┐   │
│ │ 🔐 Sign in with Passkey     │   │
│ └─────────────────────────────┘   │
│                                     │
│ ────── OR ──────                    │
│                                     │
│ [Use password instead]              │
│                                     │
│ [Captcha]                           │
│                                     │
│ Forgot password?                    │
│ Don't have an account? Sign up      │
└─────────────────────────────────────┘
           │
           ↓ (User clicks "Sign in with Passkey")
┌─────────────────────────────────────┐
│   Passkey Authentication            │
├─────────────────────────────────────┤
│ Authenticate with your passkey      │
│                                     │
│ [Device authenticator prompt...]    │
│                                     │
│ Having trouble?                     │
│ [Use password instead]              │
│ [Use recovery options]              │
└─────────────────────────────────────┘
           │
           ↓ (Success)
┌─────────────────────────────────────┐
│   Dashboard                         │
└─────────────────────────────────────┘
```

### Flow 4: Login (Password Fallback)

```
┌─────────────────────────────────────┐
│   Login Page                        │
│   (User clicks "Use password        │
│    instead")                        │
├─────────────────────────────────────┤
│ Welcome back!                       │
│                                     │
│ [Username/Email input field]        │
│ [Password input field]              │
│                                     │
│ [Captcha]                           │
│ [Sign In]                           │
│                                     │
│ ℹ️ Want faster login? Add a passkey │
│                                     │
│ Forgot password?                    │
│ Don't have an account? Sign up      │
└─────────────────────────────────────┘
```

### Flow 5: Add Passkey in Settings (Post-Registration)

```
┌─────────────────────────────────────┐
│   User Settings                     │
├─────────────────────────────────────┤
│ Security Settings                   │
│                                     │
│ Authentication Methods:             │
│                                     │
│ ┌─────────────────────────────┐   │
│ │ 🔑 Password                 │   │
│ │ Status: Active              │   │
│ │ [Change Password]           │   │
│ └─────────────────────────────┘   │
│                                     │
│ ┌─────────────────────────────┐   │
│ │ 🔐 Passkey                  │   │
│ │ Status: Not Set             │   │
│ │ [Add Passkey] ⭐ Recommended│   │
│ └─────────────────────────────┘   │
│                                     │
│ Account Recovery:                   │
│ [Manage Recovery Options]           │
└─────────────────────────────────────┘
           │
           ↓ (User clicks "Add Passkey")
┌─────────────────────────────────────┐
│   Add Passkey                       │
├─────────────────────────────────────┤
│ Set up a passkey for faster login   │
│                                     │
│ Benefits:                           │
│ ✓ Sign in with Face ID/Touch ID    │
│ ✓ More secure than passwords        │
│ ✓ Works across your devices         │
│                                     │
│ [Device authenticator prompt...]    │
│                                     │
│ [Create Passkey]                    │
│ [Cancel]                            │
└─────────────────────────────────────┘
           │
           ↓ (Success)
┌─────────────────────────────────────┐
│   User Settings                     │
├─────────────────────────────────────┤
│ ✓ Passkey added successfully!       │
│                                     │
│ Authentication Methods:             │
│ ┌─────────────────────────────┐   │
│ │ 🔐 Passkey                  │   │
│ │ Added: 2025-01-15           │   │
│ │ Device: Chrome on MacOS     │   │
│ │ [Remove]                    │   │
│ └─────────────────────────────┘   │
└─────────────────────────────────────┘
```

### Flow 6: Account Recovery (Lost Passkey)

```
┌─────────────────────────────────────┐
│   Login Page                        │
├─────────────────────────────────────┤
│ [User attempts passkey login        │
│  but fails/device lost]             │
│                                     │
│ Having trouble signing in?          │
│ [Use recovery options]              │
└─────────────────────────────────────┘
           │
           ↓
┌─────────────────────────────────────┐
│   Account Recovery Options          │
├─────────────────────────────────────┤
│ Choose a recovery method:           │
│                                     │
│ ┌─────────────────────────────┐   │
│ │ 📧 Email verification       │   │
│ │ Send code to user@email.com │   │
│ └─────────────────────────────┘   │
│                                     │
│ ┌─────────────────────────────┐   │
│ │ 🔑 Use backup password      │   │
│ │ (if set)                    │   │
│ └─────────────────────────────┘   │
│                                     │
│ [Continue]                          │
└─────────────────────────────────────┘
           │
           ↓ (Email verification selected)
┌─────────────────────────────────────┐
│   Email Verification                │
├─────────────────────────────────────┤
│ Enter the 6-digit code sent to:     │
│ user@email.com                      │
│                                     │
│ [___][___][___][___][___][___]     │
│                                     │
│ Didn't receive the code?            │
│ [Resend code]                       │
│                                     │
│ [Verify]                            │
└─────────────────────────────────────┘
           │
           ↓ (Success)
┌─────────────────────────────────────┐
│   Security Settings                 │
├─────────────────────────────────────┤
│ You're signed in!                   │
│                                     │
│ ⚠️ We recommend adding a new        │
│    passkey for this device          │
│                                     │
│ [Add Passkey Now]                   │
│ [Remind Me Later]                   │
└─────────────────────────────────────┘
```

## UI Components & Wireframes

### Component 1: Registration Form (Passkey First)

```
┌──────────────────────────────────────────┐
│  BrewGet                                 │
│  Create your account                     │
├──────────────────────────────────────────┤
│                                          │
│  @ [email@example.com              ]    │
│                                          │
│  👤 [username                       ]    │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  🔐  Continue with Passkey     │     │
│  │                                │     │
│  │  ⭐ Recommended - More Secure  │     │
│  └────────────────────────────────┘     │
│                                          │
│  ────────────── OR ──────────────        │
│                                          │
│  [▼ Show password option]                │
│                                          │
│  [Turnstile Captcha Widget]              │
│                                          │
│  Already have an account? [Sign in]      │
│                                          │
└──────────────────────────────────────────┘
```

### Component 2: Registration Form (Password Expanded)

```
┌──────────────────────────────────────────┐
│  BrewGet                                 │
│  Create your account                     │
├──────────────────────────────────────────┤
│                                          │
│  @ [email@example.com              ]    │
│                                          │
│  👤 [username                       ]    │
│                                          │
│  🔒 [password                       ]    │
│                                          │
│  🔒 [confirm password               ]    │
│                                          │
│  ☑ Add a passkey for easier sign-in     │
│     (optional)                           │
│                                          │
│  [Turnstile Captcha Widget]              │
│                                          │
│  [Create Account]                        │
│                                          │
│  Already have an account? [Sign in]      │
│                                          │
└──────────────────────────────────────────┘
```

### Component 3: Login Form (Passkey First)

```
┌──────────────────────────────────────────┐
│  BrewGet                                 │
│  Welcome back!                           │
├──────────────────────────────────────────┤
│                                          │
│  👤 [username or email             ]    │
│                                          │
│  ┌────────────────────────────────┐     │
│  │  🔐  Sign in with Passkey      │     │
│  └────────────────────────────────┘     │
│                                          │
│  ────────────── OR ──────────────        │
│                                          │
│  [Use password instead]                  │
│                                          │
│  [Turnstile Captcha Widget]              │
│                                          │
│  [Forgot password?]                      │
│                                          │
│  Don't have an account? [Sign up]        │
│                                          │
└──────────────────────────────────────────┘
```

### Component 4: Login Form (Password Expanded)

```
┌──────────────────────────────────────────┐
│  BrewGet                                 │
│  Welcome back!                           │
├──────────────────────────────────────────┤
│                                          │
│  👤 [username or email             ]    │
│                                          │
│  🔒 [password                       ]    │
│                                          │
│  [Turnstile Captcha Widget]              │
│                                          │
│  [Sign In]                               │
│                                          │
│  💡 Want faster login? Add a passkey     │
│                                          │
│  [Try passkey instead] | [Forgot password?]│
│                                          │
│  Don't have an account? [Sign up]        │
│                                          │
└──────────────────────────────────────────┘
```

### Component 5: Settings - Security Section

```
┌──────────────────────────────────────────┐
│  Settings > Security                     │
├──────────────────────────────────────────┤
│                                          │
│  Authentication Methods                  │
│                                          │
│  ┌────────────────────────────────┐     │
│  │ 🔐 Passkeys                    │     │
│  │                                │     │
│  │ [No passkeys set up]           │     │
│  │                                │     │
│  │ [+ Add Passkey] ⭐             │     │
│  │                                │     │
│  │ Passkeys provide secure,       │     │
│  │ passwordless authentication    │     │
│  └────────────────────────────────┘     │
│                                          │
│  ┌────────────────────────────────┐     │
│  │ 🔑 Password                    │     │
│  │                                │     │
│  │ Status: Active                 │     │
│  │ Last changed: 2024-12-01       │     │
│  │                                │     │
│  │ [Change Password]              │     │
│  └────────────────────────────────┘     │
│                                          │
│  ┌────────────────────────────────┐     │
│  │ 📧 Account Recovery            │     │
│  │                                │     │
│  │ Email: user@example.com ✓      │     │
│  │                                │     │
│  │ [Manage Recovery Options]      │     │
│  └────────────────────────────────┘     │
│                                          │
└──────────────────────────────────────────┘
```

## Recovery Flows Documentation

### Lost Passkey Recovery Options

When a user loses access to their passkey, they have multiple recovery options:

#### Option 1: Email Verification Code (OTP)
1. User clicks "Having trouble signing in?" on login page
2. System sends 6-digit OTP to registered email
3. User enters OTP to verify identity
4. System grants access and prompts to set up new passkey
5. User can add new passkey for current device

**Implementation Notes:**
- OTP valid for 10 minutes
- Maximum 3 attempts before cooldown
- Rate limit: 1 code per 60 seconds
- Requires email verification during registration

#### Option 2: Backup Password (if set)
1. User clicks "Use password instead" during passkey login
2. System prompts for password
3. User enters backup password
4. System grants access
5. Optional: User can add passkey for current device

**Implementation Notes:**
- Password is optional during passkey registration
- Strongly recommend setting backup password
- Password follows same validation rules as password-only accounts

#### Option 3: Account Recovery via Support (last resort)
1. User contacts support with account details
2. Support verifies identity through email verification
3. Support manually resets authentication methods
4. User receives password reset link
5. User sets new password and optionally new passkey

### Best Practices for Recovery

1. **Multiple Passkeys**: Encourage users to register passkeys on multiple devices
2. **Backup Password**: Recommend setting a backup password during registration
3. **Verified Email**: Require email verification before allowing passkey-only accounts
4. **Clear Instructions**: Provide clear UI hints about recovery options
5. **Device Management**: Allow users to see and manage all registered passkeys

## Security Considerations

### Passkey Security
- Use WebAuthn Level 2 specification
- Store credentials securely in database (encrypted)
- Support attestation for device verification
- Implement rate limiting on authentication attempts
- Log all passkey creation and usage events

### Password Security (when used)
- Maintain current password hashing (bcrypt/argon2)
- Enforce password strength requirements
- Store password history to prevent reuse
- Support account lockout after failed attempts

### Session Management
- Use same JWT token system for both auth methods
- Implement proper token rotation
- Set appropriate token expiration times
- Support revocation of all sessions

## Browser/Device Support

### Passkey Support Matrix
- ✅ Chrome 108+ (Desktop & Mobile)
- ✅ Safari 16+ (iOS/macOS)
- ✅ Firefox 119+ (Desktop)
- ✅ Edge 108+ (Desktop & Mobile)
- ⚠️  Fallback to password for unsupported browsers

### Detection Strategy
1. Check WebAuthn API availability: `window.PublicKeyCredential`
2. Check platform authenticator: `PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable()`
3. Show appropriate UI based on support level
4. Gracefully fallback to password if not supported

## User Education

### In-App Messaging
- **First Login**: "Did you know? You can sign in faster with a passkey!"
- **Settings**: "Add a passkey for secure, passwordless access"
- **Password Reset**: "Consider setting up a passkey to avoid password resets"

### Tooltips & Help Text
- Explain what passkeys are in simple terms
- Highlight benefits: faster, more secure, no passwords to remember
- Provide links to browser-specific setup guides
- Show device compatibility information

## Success Metrics

### Key Performance Indicators
1. Passkey adoption rate among new registrations
2. Passkey adoption rate among existing users
3. Login success rate (passkey vs password)
4. Average login time (passkey vs password)
5. Account recovery request volume
6. User satisfaction scores

### Target Goals (6 months post-launch)
- 60% of new users register with passkey
- 30% of existing users add passkey
- 95%+ passkey login success rate
- <2 second average login time with passkey
- 50% reduction in password reset requests

## Implementation Phases

### Phase 1: Backend Infrastructure (Week 1-2)
- Database schema for passkey credentials
- WebAuthn server implementation
- API endpoints for registration/authentication
- Update user model for optional password

### Phase 2: Frontend UI (Week 2-3)
- Registration flow with passkey-first UI
- Login flow with passkey-first UI
- Settings page for passkey management
- Browser compatibility detection

### Phase 3: Recovery Flows (Week 3-4)
- Email OTP verification system
- Recovery UI components
- Support documentation

### Phase 4: Testing & Documentation (Week 4)
- End-to-end testing
- Cross-browser testing
- User documentation
- Developer documentation

### Phase 5: Gradual Rollout (Week 5+)
- Beta testing with subset of users
- Monitor metrics and gather feedback
- Address issues and iterate
- Full production release

## Technical References

- [WebAuthn Specification](https://www.w3.org/TR/webauthn-2/)
- [FIDO2 Guidelines](https://fidoalliance.org/specs/)
- [Passkey Best Practices](https://web.dev/passkey-form-autofill/)
- [WebAuthn Awesome List](https://github.com/herrjemand/awesome-webauthn)
