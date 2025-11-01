# Changelog - Email Service

All notable changes to the Email Service will be documented in this file.

## [0.0.10] - 2025-11-01
### Changed
- Enhanced tracing and logging with structured fields for better observability
- Added instrumentation spans to all service methods for distributed tracing
- Improved error logging with detailed context (email, username, SMTP details)
- Added SMTP response code logging for successful email deliveries

## [0.0.9] - 2025-10-31
### Added
- Add HTTP health endpoint on dedicated port (8001) for version reporting
- Add axum and serde dependencies for HTTP health endpoint

## [0.0.8] - 2025-10-31
### Changed
- Replace static message strings with translation keys for frontend localization

## [0.0.7] - 2025-10-30
### Added
- Add logging to backend routes
### Changed
- Update dependencies

## [0.0.6] - 2025-09-17
### Added
- Add shared-types to docker build

## [0.0.5] - 2025-09-17
### Added
- Add shared library for errors and responses

## [0.0.4] - 2025-09-21
### Changed
- Improve Docker image sizes

## [0.0.3] - 2025-09-08
### Changed
- Update email service configurations
- Remove hardcoded strings and use configs

## [0.0.2] - 2025-09-07
### Added
- Add forgot password email functionality
- Create verification email for account registration

## [0.0.1] - 2025-09-07
### Added
- Initial email service with gRPC support
- SMTP configuration for sending emails
- Email templates using Handlebars
