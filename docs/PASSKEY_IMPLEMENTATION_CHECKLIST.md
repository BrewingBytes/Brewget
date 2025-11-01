# Passkey Authentication - Implementation Checklist

This document provides a detailed, step-by-step checklist for implementing the passkey authentication system. Use this as your primary task list during development.

## 📋 Pre-Implementation

- [x] ✅ UX Design & Wireframes Completed
- [x] ✅ Technical Architecture Defined
- [x] ✅ Database Schema Designed
- [x] ✅ API Endpoints Specified
- [x] ✅ Security Requirements Documented
- [x] ✅ Documentation Complete

## Phase 1: Backend Development - Database

### Database Migrations

- [ ] **Create migration: Make password optional**
  - [ ] Modify `users` table: `ALTER COLUMN password DROP NOT NULL`
  - [ ] Add `has_passkey` column to `users` table
  - [ ] Create index on `has_passkey` column
  - [ ] Test migration forward and rollback
  - [ ] Document migration in changelog

- [ ] **Create migration: Passkey credentials table**
  - [ ] Create `passkey_credentials` table with all columns
  - [ ] Add foreign key to `users` table with CASCADE delete
  - [ ] Create indexes on `user_id`, `credential_id`, `is_active`
  - [ ] Add unique constraint on `credential_id`
  - [ ] Test migration forward and rollback
  - [ ] Document migration in changelog

- [ ] **Create migration: Sync trigger**
  - [ ] Create `update_user_has_passkey()` function
  - [ ] Create trigger on `passkey_credentials` INSERT/DELETE
  - [ ] Test trigger automatically updates `users.has_passkey`
  - [ ] Document trigger behavior

- [ ] **Create migration: Audit log table (optional)**
  - [ ] Create `authentication_audit_log` table
  - [ ] Add indexes for querying by user, date, method
  - [ ] Test logging functionality
  - [ ] Document audit log usage

- [ ] **Run migrations on development database**
  - [ ] Backup existing database
  - [ ] Run migrations with `sqlx migrate run`
  - [ ] Verify all tables created correctly
  - [ ] Check indexes and constraints

## Phase 2: Backend Development - Models & Database Layer

### Rust Models

- [ ] **Create PasskeyCredential model** (`src/models/passkey_credential.rs`)
  - [ ] Define `PasskeyCredential` struct matching database
  - [ ] Implement `FromRow` trait for sqlx
  - [ ] Add `to_passkey()` method for webauthn-rs conversion
  - [ ] Add `NewPasskeyCredential` struct for inserts
  - [ ] Add documentation and examples

- [ ] **Create request models** (`src/models/request/`)
  - [ ] `PasskeyRegisterStartRequest`
  - [ ] `PasskeyRegisterFinishRequest`
  - [ ] `PasskeyLoginStartRequest`
  - [ ] `PasskeyLoginFinishRequest`
  - [ ] Add Serde derives and validation

- [ ] **Create response models** (`src/models/response/`)
  - [ ] `PasskeyRegisterStartResponse`
  - [ ] `PasskeyLoginStartResponse`
  - [ ] Add new `TranslationKey` variants for passkey errors

- [ ] **Update User model** (`src/models/user.rs`)
  - [ ] Make password field `Option<String>`
  - [ ] Add `has_passkey()` method
  - [ ] Update `NewUser` to support optional password
  - [ ] Add validation for passkey-only vs password accounts

### Database Layer

- [ ] **Create passkey credentials database operations** (`src/database/passkey_credentials.rs`)
  - [ ] `insert()` - Add new credential
  - [ ] `find_by_user_id()` - Get all user's credentials
  - [ ] `find_by_credential_id()` - Get specific credential
  - [ ] `update_counter()` - Update counter after auth
  - [ ] `delete()` - Soft delete credential
  - [ ] `list_active()` - Get all active credentials
  - [ ] Add proper error handling
  - [ ] Add database transaction support

- [ ] **Update users database operations** (`src/database/users.rs`)
  - [ ] Update `insert()` to support optional password
  - [ ] Add `has_passkey()` query helper
  - [ ] Update existing queries as needed

- [ ] **Create audit log operations** (`src/database/audit_log.rs`) (optional)
  - [ ] `log_authentication_attempt()` - Log auth events
  - [ ] `get_user_history()` - Query user's auth history
  - [ ] `get_failed_attempts()` - Security monitoring

## Phase 3: Backend Development - WebAuthn Integration

### Dependencies

- [ ] **Add dependencies to Cargo.toml**
  ```toml
  webauthn-rs = "0.5"
  webauthn-rs-proto = "0.5"
  ```
  - [ ] Run `cargo update`
  - [ ] Verify compilation

### Configuration

- [ ] **Update Config struct** (`src/config.rs`)
  - [ ] Add `rp_id` field (relying party ID)
  - [ ] Add `rp_origin` field (frontend URL)
  - [ ] Add `rp_name` field ("BrewGet")
  - [ ] Add `build_webauthn()` method
  - [ ] Load from environment variables
  - [ ] Add validation

- [ ] **Update environment variables** (`.env`)
  - [ ] Add `WEBAUTHN_RP_ID=localhost`
  - [ ] Add `WEBAUTHN_RP_ORIGIN=http://localhost:5173`
  - [ ] Add `WEBAUTHN_RP_NAME=BrewGet`
  - [ ] Document in `.env.example`

### App State

- [ ] **Update AppState** (`src/app_state.rs`)
  - [ ] Add `passkey_registrations` HashMap (challenge storage)
  - [ ] Add `passkey_authentications` HashMap (challenge storage)
  - [ ] Add `pending_users` HashMap (registration state)
  - [ ] Add storage methods with TTL
  - [ ] Add cleanup methods
  - [ ] Consider using Redis for production

## Phase 4: Backend Development - API Endpoints

### Passkey Registration Routes

- [ ] **Create passkey register routes** (`src/routes/passkey_register.rs`)
  - [ ] Create router with `/start` and `/finish` endpoints
  - [ ] Implement `passkey_register_start()`
    - [ ] Verify captcha
    - [ ] Validate username and email
    - [ ] Check for existing user
    - [ ] Generate WebAuthn challenge
    - [ ] Store temporary state
    - [ ] Return challenge to client
  - [ ] Implement `passkey_register_finish()`
    - [ ] Retrieve stored challenge
    - [ ] Verify credential signature
    - [ ] Create user in database
    - [ ] Store passkey credential
    - [ ] Send activation email
    - [ ] Clean up temporary state
  - [ ] Add comprehensive error handling
  - [ ] Add logging

### Passkey Login Routes

- [ ] **Create passkey login routes** (`src/routes/passkey_login.rs`)
  - [ ] Create router with `/start` and `/finish` endpoints
  - [ ] Implement `passkey_login_start()`
    - [ ] Verify captcha
    - [ ] Find user by username
    - [ ] Check user has passkeys
    - [ ] Get user's passkey credentials
    - [ ] Generate WebAuthn challenge
    - [ ] Store temporary state
    - [ ] Return challenge to client
  - [ ] Implement `passkey_login_finish()`
    - [ ] Retrieve stored challenge
    - [ ] Verify credential signature
    - [ ] Validate counter (replay protection)
    - [ ] Update counter in database
    - [ ] Generate JWT token
    - [ ] Store token
    - [ ] Return token to client
  - [ ] Add comprehensive error handling
  - [ ] Add logging

### Update Existing Routes

- [ ] **Update register route** (`src/routes/register.rs`)
  - [ ] Support optional password field
  - [ ] Add validation for passkey-only vs password registration
  - [ ] Update error messages

- [ ] **Update login route** (`src/routes/login.rs`)
  - [ ] Handle users with no password (passkey-only)
  - [ ] Update error messages

### Route Registration

- [ ] **Update main routes** (`src/routes.rs`)
  - [ ] Add passkey register router under `/auth/passkey/register`
  - [ ] Add passkey login router under `/auth/passkey/login`
  - [ ] Ensure CORS configuration includes new routes
  - [ ] Test all routes are accessible

## Phase 5: Backend Development - Testing

### Unit Tests

- [ ] **Test User model changes**
  - [ ] Test optional password field
  - [ ] Test `has_passkey()` method
  - [ ] Test validation logic

- [ ] **Test PasskeyCredential model**
  - [ ] Test `to_passkey()` conversion
  - [ ] Test serialization/deserialization

- [ ] **Test database operations**
  - [ ] Test passkey credential CRUD operations
  - [ ] Test counter updates
  - [ ] Test transaction rollback

### Integration Tests

- [ ] **Test passkey registration flow**
  - [ ] Test successful registration
  - [ ] Test duplicate username/email
  - [ ] Test invalid credentials
  - [ ] Test challenge expiration

- [ ] **Test passkey login flow**
  - [ ] Test successful authentication
  - [ ] Test invalid credentials
  - [ ] Test counter validation
  - [ ] Test user without passkey

- [ ] **Test password fallback**
  - [ ] Test users with both passkey and password
  - [ ] Test users with only password
  - [ ] Test users with only passkey

### Backend Code Quality

- [ ] **Run formatter**
  ```bash
  cd backend
  cargo fmt
  ```

- [ ] **Run linter**
  ```bash
  cd backend
  cargo clippy -- -D warnings
  ```

- [ ] **Run all tests**
  ```bash
  cd backend
  cargo test
  ```

- [ ] **Check for security issues**
  - [ ] Run `cargo audit`
  - [ ] Review dependency versions
  - [ ] Check for SQL injection vulnerabilities

## Phase 6: Frontend Development - WebAuthn Service

### Dependencies

- [ ] **Install required packages**
  - [ ] Check if any WebAuthn libraries needed
  - [ ] Update package.json if needed

### WebAuthn Service

- [ ] **Create WebAuthn service** (`src/services/webauthn.ts`)
  - [ ] Implement `checkPasskeySupport()`
  - [ ] Implement `registerPasskey()`
  - [ ] Implement `authenticateWithPasskey()`
  - [ ] Implement `credentialToJSON()`
  - [ ] Implement `assertionToJSON()`
  - [ ] Add error handling
  - [ ] Add type definitions

- [ ] **Create base64 utilities** (`src/utils/base64.ts`)
  - [ ] Implement `base64URLStringToBuffer()`
  - [ ] Implement `bufferToBase64URLString()`
  - [ ] Add unit tests

### Auth Service Updates

- [ ] **Update auth service** (`src/services/auth/index.ts`)
  - [ ] Add `passkeyRegisterStart()`
  - [ ] Add `passkeyRegisterFinish()`
  - [ ] Add `passkeyLoginStart()`
  - [ ] Add `passkeyLoginFinish()`
  - [ ] Update error handling
  - [ ] Add TypeScript types

## Phase 7: Frontend Development - UI Components

### Capability Detection

- [ ] **Create capability detection composable** (`src/composables/usePasskeySupport.ts`)
  - [ ] Check WebAuthn availability
  - [ ] Check platform authenticator availability
  - [ ] Provide reactive support status
  - [ ] Handle browser compatibility

### Registration Flow

- [ ] **Update AuthGlass component** (`src/components/AuthGlass.vue`)
  - [ ] Add passkey support detection
  - [ ] Add "Continue with Passkey" button
  - [ ] Add "Show password option" toggle
  - [ ] Update registration form logic
  - [ ] Add passkey registration flow
  - [ ] Add loading states
  - [ ] Add error handling
  - [ ] Update styling for new UI

- [ ] **Create PasskeyRegisterModal component** (optional)
  - [ ] Show passkey benefits
  - [ ] Handle WebAuthn registration
  - [ ] Show success/error states
  - [ ] Add device name input

### Login Flow

- [ ] **Update AuthGlass login view**
  - [ ] Add "Sign in with Passkey" button
  - [ ] Add "Use password instead" toggle
  - [ ] Update login form logic
  - [ ] Add passkey login flow
  - [ ] Add loading states
  - [ ] Add error handling
  - [ ] Show helpful error messages

### Settings Integration

- [ ] **Create PasskeyManagement component** (`src/components/settings/PasskeyManagement.vue`)
  - [ ] Show list of registered passkeys
  - [ ] Show device names and dates
  - [ ] Add "Add Passkey" button
  - [ ] Add "Remove Passkey" button
  - [ ] Handle passkey registration in settings
  - [ ] Add confirmation dialogs

- [ ] **Update SettingsView** (`src/views/SettingsView.vue`)
  - [ ] Add security section
  - [ ] Integrate PasskeyManagement component
  - [ ] Show authentication methods status
  - [ ] Add helpful tips and links

### Recovery UI

- [ ] **Create recovery components**
  - [ ] Email OTP input component
  - [ ] Recovery options selector
  - [ ] Add to login flow
  - [ ] Handle recovery state

## Phase 8: Frontend Development - Store Updates

### Auth Store

- [ ] **Update auth store** (`src/stores/auth.ts`)
  - [ ] Add `registerWithPasskey()` method
  - [ ] Add `loginWithPasskey()` method
  - [ ] Add `addPasskey()` method (for settings)
  - [ ] Add `removePasskey()` method
  - [ ] Add `listPasskeys()` method
  - [ ] Update state management
  - [ ] Add error handling

## Phase 9: Localization

### Translation Keys

- [ ] **Add new translation keys** (`src/locales/en.json`)
  - [ ] Passkey-related UI text
  - [ ] Error messages
  - [ ] Success messages
  - [ ] Help text and tooltips
  - [ ] Settings labels

- [ ] **Update other locales**
  - [ ] `src/locales/fr.json`
  - [ ] `src/locales/de.json`
  - [ ] `src/locales/ro.json`
  - [ ] `src/locales/es.json`

### Backend Translations

- [ ] **Add new TranslationKey variants** (`backend/auth-service/src/models/response.rs`)
  - [ ] `PASSKEY_NOT_SUPPORTED`
  - [ ] `PASSKEY_REGISTRATION_FAILED`
  - [ ] `PASSKEY_AUTHENTICATION_FAILED`
  - [ ] `NO_PASSKEY_CONFIGURED`
  - [ ] `PASSKEY_ADDED_SUCCESSFULLY`
  - [ ] `PASSKEY_REMOVED_SUCCESSFULLY`
  - [ ] `REGISTRATION_SESSION_EXPIRED`
  - [ ] `AUTHENTICATION_SESSION_EXPIRED`

## Phase 10: Testing

### Manual Testing

- [ ] **Test registration flows**
  - [ ] Register with passkey (Chrome)
  - [ ] Register with passkey (Safari)
  - [ ] Register with passkey (Firefox)
  - [ ] Register with password fallback
  - [ ] Register with both passkey and password

- [ ] **Test login flows**
  - [ ] Login with passkey (Chrome)
  - [ ] Login with passkey (Safari)
  - [ ] Login with passkey (Firefox)
  - [ ] Login with password fallback
  - [ ] Test "Use password instead" toggle

- [ ] **Test settings integration**
  - [ ] Add passkey from settings
  - [ ] Remove passkey from settings
  - [ ] View passkey list
  - [ ] Test with multiple passkeys

- [ ] **Test recovery flows**
  - [ ] Email OTP recovery
  - [ ] Password fallback
  - [ ] Edge cases

- [ ] **Test browser compatibility**
  - [ ] Chrome 108+ (Desktop)
  - [ ] Chrome (Mobile)
  - [ ] Safari 16+ (Desktop)
  - [ ] Safari (iOS)
  - [ ] Firefox 119+ (Desktop)
  - [ ] Edge 108+ (Desktop)
  - [ ] Older browsers (fallback to password)

- [ ] **Test device compatibility**
  - [ ] Windows (Windows Hello)
  - [ ] macOS (Touch ID)
  - [ ] iOS (Face ID/Touch ID)
  - [ ] Android (Biometric)
  - [ ] USB Security Keys

### Automated Testing

- [ ] **Frontend unit tests**
  - [ ] WebAuthn service tests
  - [ ] Base64 utility tests
  - [ ] Component tests

- [ ] **Frontend E2E tests**
  - [ ] Registration flow test
  - [ ] Login flow test
  - [ ] Settings integration test

- [ ] **Backend tests**
  - [ ] Already covered in Phase 5

### Performance Testing

- [ ] **Measure authentication speed**
  - [ ] Passkey login time
  - [ ] Password login time
  - [ ] Compare and document

- [ ] **Load testing**
  - [ ] Test concurrent registrations
  - [ ] Test concurrent logins
  - [ ] Monitor database performance

### Security Testing

- [ ] **Verify security requirements**
  - [ ] HTTPS requirement enforced
  - [ ] Origin validation working
  - [ ] Counter validation working
  - [ ] Rate limiting working
  - [ ] Audit logging working

- [ ] **Test attack scenarios**
  - [ ] Replay attack prevention
  - [ ] Cross-origin attacks
  - [ ] Man-in-the-middle (HTTPS)
  - [ ] Brute force protection

## Phase 11: Documentation & Polish

### User Documentation

- [ ] **Create user guide**
  - [ ] What are passkeys
  - [ ] How to set up passkeys
  - [ ] How to use passkeys
  - [ ] Troubleshooting guide
  - [ ] FAQ

- [ ] **Update existing docs**
  - [ ] Update README with passkey info
  - [ ] Update development guide
  - [ ] Update deployment guide

### Developer Documentation

- [ ] **Update API documentation**
  - [ ] Document new endpoints
  - [ ] Add request/response examples
  - [ ] Document error codes

- [ ] **Update code documentation**
  - [ ] Add inline comments
  - [ ] Update function documentation
  - [ ] Add usage examples

### UI Polish

- [ ] **Improve user experience**
  - [ ] Add loading animations
  - [ ] Add success animations
  - [ ] Improve error messages
  - [ ] Add helpful tooltips
  - [ ] Test accessibility (a11y)

- [ ] **Design review**
  - [ ] Get feedback on UI/UX
  - [ ] Make adjustments
  - [ ] Ensure consistency

## Phase 12: Deployment Preparation

### Environment Setup

- [ ] **Update production environment variables**
  - [ ] Set correct `WEBAUTHN_RP_ID`
  - [ ] Set correct `WEBAUTHN_RP_ORIGIN`
  - [ ] Verify HTTPS is configured
  - [ ] Update `.env.example`

### Database Migration

- [ ] **Prepare migration plan**
  - [ ] Backup production database
  - [ ] Test migrations on staging
  - [ ] Create rollback plan
  - [ ] Document migration steps

### Monitoring

- [ ] **Set up monitoring**
  - [ ] Log passkey registration events
  - [ ] Log passkey authentication events
  - [ ] Track success/failure rates
  - [ ] Set up alerts for errors

### Rollout Strategy

- [ ] **Plan gradual rollout**
  - [ ] Enable for beta users first
  - [ ] Monitor metrics
  - [ ] Fix any issues
  - [ ] Gradually increase percentage
  - [ ] Full rollout

## Phase 13: Deployment

### Staging Deployment

- [ ] **Deploy to staging**
  - [ ] Run database migrations
  - [ ] Deploy backend
  - [ ] Deploy frontend
  - [ ] Test all flows

- [ ] **Staging validation**
  - [ ] Smoke tests
  - [ ] E2E tests
  - [ ] Performance tests
  - [ ] Security scan

### Production Deployment

- [ ] **Deploy to production**
  - [ ] Schedule maintenance window
  - [ ] Backup database
  - [ ] Run migrations
  - [ ] Deploy backend
  - [ ] Deploy frontend
  - [ ] Verify deployment

- [ ] **Post-deployment**
  - [ ] Monitor logs
  - [ ] Check error rates
  - [ ] Monitor performance
  - [ ] Gather user feedback

## Phase 14: Post-Launch

### Monitoring

- [ ] **Track key metrics**
  - [ ] Passkey adoption rate
  - [ ] Authentication success rate
  - [ ] Average login time
  - [ ] Error rates
  - [ ] User feedback

### Iteration

- [ ] **Gather feedback**
  - [ ] User surveys
  - [ ] Support tickets
  - [ ] Analytics data

- [ ] **Make improvements**
  - [ ] Address issues
  - [ ] Optimize UX
  - [ ] Improve documentation
  - [ ] Add features as needed

### Future Enhancements

- [ ] **Consider additional features**
  - [ ] Passkey sync across devices (iCloud, Google)
  - [ ] Conditional mediation (auto-login)
  - [ ] Security key attestation
  - [ ] Enterprise SSO integration
  - [ ] Biometric preferences
  - [ ] Multi-device registration flow

---

## Progress Tracking

**Phase 1 (Database)**: 0/5 complete
**Phase 2 (Models)**: 0/4 complete  
**Phase 3 (WebAuthn)**: 0/3 complete
**Phase 4 (API)**: 0/4 complete
**Phase 5 (Backend Tests)**: 0/3 complete
**Phase 6 (Frontend WebAuthn)**: 0/3 complete
**Phase 7 (Frontend UI)**: 0/5 complete
**Phase 8 (Frontend Store)**: 0/1 complete
**Phase 9 (Localization)**: 0/2 complete
**Phase 10 (Testing)**: 0/5 complete
**Phase 11 (Documentation)**: 0/3 complete
**Phase 12 (Prep)**: 0/4 complete
**Phase 13 (Deploy)**: 0/2 complete
**Phase 14 (Post-Launch)**: 0/2 complete

**Total Progress**: 0/50 sections complete (0%)

---

## Notes

- Each checkbox should be completed in order within its phase
- Some phases can be done in parallel (e.g., backend and frontend)
- Test frequently during development
- Update this checklist as you complete items
- Document any deviations or decisions

## Support

If you get stuck:
1. Review the relevant documentation
2. Check the architecture diagrams
3. Refer to the implementation guide
4. Ask for help in team channels

Good luck! 🚀
