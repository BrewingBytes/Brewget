# Changelog - Frontend

All notable changes to the Frontend will be documented in this file.

## [0.0.10] - 2024-12-21
### Added
- Add version display in settings page footer
- Add changelog modal with tabs for all services
- Add logout button in settings page
- Add translations for cookie consent modal
- Version fetching from backend services /health endpoints

### Changed
- Cookie consent now uses i18n translations

## [0.0.9] - 2024-12-20
### Added
- Implement i18n with browser language detection
- Support for multiple languages (en, es, fr, de, ro)
- Backend language constraints integration

## [0.0.8] - 2024-12-18
### Added
- Implement cookie consent modal at bottom with blocking overlay

## [0.0.7] - 2024-11-22
### Added
- Add Cloudflare Turnstile captcha to authentication endpoints

## [0.0.6] - 2024-11-20
### Fixed
- Fix SPA routing by replacing busybox httpd with nginx in frontend container

## [0.0.5] - 2024-11-18
### Added
- Create UI User Settings page with PrimeVue
- Add settings management (language, currency, alarm, night mode)
- Add settings persistence

## [0.0.4] - 2024-11-15
### Added
- Add navbar to frontend with navigation

## [0.0.3] - 2024-11-14
### Fixed
- Fix frontend connection to API in production

## [0.0.2] - 2024-11-14
### Changed
- Connect the auth backend and frontend
- Add authentication flow

## [0.0.1] - 2024-11-12
### Added
- Initial PrimeVue frontend implementation
- Add forgot password UI
- Login and registration pages
- Basic Vue 3 application structure with TypeScript
