# Changelog - Infrastructure

All notable changes to the infrastructure and deployment will be documented in this file.

## [Unreleased]
### Added
- Create systemd service for minikube tunnel to ensure it restarts automatically if it dies
- Deploy script now sets up minikube tunnel as a managed systemd service
- Cleanup script now properly stops and removes the minikube tunnel service

### Changed
- Replace background minikube tunnel process with systemd service for better reliability
