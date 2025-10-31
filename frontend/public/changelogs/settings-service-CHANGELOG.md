# Changelog - Settings Service

All notable changes to the Settings Service will be documented in this file.

## [0.0.6] - 2024-12-19
### Added
- Add i18n support with browser language detection and backend constraints
### Changed
- Replace static message strings with translation keys for frontend localization

## [0.0.5] - 2024-12-15
### Added
- Add logging to backend routes
### Changed
- Update dependencies

## [0.0.4] - 2024-11-18
### Added
- Add gRPC token verification with persistent connection
- Add comprehensive logging
- Add environment-based API configuration
### Changed
- Create UI User Settings with PrimeVue integration

## [0.0.3] - 2024-11-17
### Added
- Add shared-types to docker build

## [0.0.2] - 2024-11-17
### Added
- Add shared library for errors and responses

## [0.0.1] - 2024-11-16
### Added
- Initial settings service with user settings management
- Database support with SQLX
- RESTful API for settings CRUD operations
- Integration with auth service via gRPC
