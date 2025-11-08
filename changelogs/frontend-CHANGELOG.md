# Changelog - Frontend

All notable changes to the Frontend will be documented in this file.

## [Unreleased]
### Added
- Add transactions feature with full CRUD operations
- Add custom categories management for transactions
- Add transaction filtering by wallet
- Add i18n translations for transactions and categories
- Add transaction types (Income, Expense, Transfer) with built-in categories
- Add transaction list view with date grouping
- Add "View Transactions" button to wallet cards
- Add transaction service layer for API communication
- Add transaction and custom category stores
- Add `/transactions` route with support for wallet filtering

## [0.0.16] - 2025-11-08
### Added
- Add wallets feature
- Add i18n translations for wallets

## [0.0.15] - 2025-11-02
### Added
- Add manage passkeys UI to allow users to view and delete their passkeys
- Add authentication activity audit viewer in the settings page

### Fixed
- Fix issue with settings not loading at all

## [0.0.11] - 2025-11-02
### Fixed
- Fix Turnstile captcha not resetting on failed login/register attempts
- Fix login taking too long due to waiting for settings load
- Fix token expiration handling to properly log out user on invalid/expired token

### Added
- Add passkey support for passwordless authentication

## [0.0.10] - 2025-10-31
### Added
- Add version display in settings page footer
- Add changelog modal with tabs for all services
- Add logout button in settings page
- Add translations for cookie consent modal
- Version fetching from backend services /health endpoints

### Changed
- Cookie consent now uses i18n translations

## [0.0.9] - 2025-10-31
### Added
- Implement i18n with browser language detection
- Support for multiple languages (en, es, fr, de, ro)
- Backend language constraints integration

## [0.0.8] - 2025-10-31
### Added
- Implement cookie consent modal at bottom with blocking overlay

## [0.0.7] - 2025-10-29
### Added
- Add Cloudflare Turnstile captcha to authentication endpoints

## [0.0.6] - 2025-10-29
### Fixed
- Fix SPA routing by replacing busybox httpd with nginx in frontend container

## [0.0.5] - 2025-10-28
### Added
- Create UI User Settings page with PrimeVue
- Add settings management (language, currency, alarm, night mode)
- Add settings persistence

## [0.0.4] - 2025-09-22
### Added
- Add navbar to frontend with navigation

## [0.0.3] - 2025-09-21
### Fixed
- Fix frontend connection to API in production

## [0.0.2] - 2025-09-21
### Changed
- Connect the auth backend and frontend
- Add authentication flow

## [0.0.1] - 2025-09-20
### Added
- Initial PrimeVue frontend implementation
- Add forgot password UI
- Login and registration pages
- Basic Vue 3 application structure with TypeScript
