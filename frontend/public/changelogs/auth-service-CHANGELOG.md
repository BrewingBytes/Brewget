# Changelog - Auth Service

All notable changes to the Auth Service will be documented in this file.

## [0.0.12] - 2025-10-31
### Changed
- Replace static message strings with translation keys for frontend localization

## [0.0.11] - 2025-10-30
### Added
- Add logging to backend routes
### Changed
- Update dependencies

## [0.0.10] - 2025-10-30
### Added
- Prevent password reuse by tracking configurable number of passwords with transactional integrity

## [0.0.9] - 2025-10-29
### Added
- Add Cloudflare Turnstile captcha to authentication endpoints

## [0.0.8] - 2025-10-28
### Added
- Add gRPC token verification with persistent connection
- Add comprehensive logging
- Add environment-based API configuration
### Changed
- Create UI User Settings with PrimeVue integration

## [0.0.7] - 2025-09-17
### Added
- Add shared-types to docker build

## [0.0.6] - 2025-09-17
### Added
- Add shared library for errors and responses

## [0.0.5] - 2025-09-21
### Changed
- Connect the auth backend and frontend

## [0.0.4] - 2025-09-21
### Changed
- Improve Docker image sizes

## [0.0.3] - 2025-09-08
### Changed
- Update email service and auth service configurations
- Remove hardcoded strings and use configs

## [0.0.2] - 2025-09-07
### Added
- Create verification email for account registration
- Implement forgot password functionality
- Add basic login/register and logout endpoints

## [0.0.1] - 2025-09-06
### Added
- Initial auth service with database functions
- Add migrations
- Add Docker files and database to app state
