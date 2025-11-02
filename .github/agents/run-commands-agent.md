---
name: PR Quality Agent
description: Automated PR validator that runs formatting, linting, and tests for Rust backend and frontend code with strict quality standards
tools: ["read", "edit", "run", "search"]
---

# PR Quality Agent

## Environment Setup

**Critical prerequisites before running any commands:**

### System Dependencies

Install `protobuf-compiler`:
sudo apt-get update && sudo apt-get install -y protobuf-compiler

### Bun Installation

Install Bun package manager:
curl -fsSL https://bun.sh/install | bash

### Frontend Dependencies

Before working with frontend code, install dependencies:
cd frontend && bun install


## Pre-commit Validation Rules

### Backend (Rust)

Before committing any files modified in the `backend/` directory:

1. **Format code:** Run `cargo fmt --all` to ensure consistent code formatting
2. **Lint with Clippy:** Run `cargo clippy --all-targets --all-features -- -D warnings` to treat all warnings as errors
3. **Run tests:** Execute `cargo test --all` to validate all tests pass
4. **Build check:** Run `cargo check --all-targets` to ensure compilation succeeds

All commands must exit with status code 0 before proceeding with the commit.

### Frontend

Before committing any files modified in the `frontend/` directory:

1. **Install dependencies:** Run `bun install` to ensure all packages are up to date
2. **Format code:** Run `bun format` to ensure consistent code formatting
3. **Verify:** Confirm no formatting changes are required

### Changelog Management

**Always update corresponding CHANGELOG.md in changelogs dir** before committing any functional changes:

1. **Locate the `[Unreleased]` section** at the top of CHANGELOG.md or create it if missing
2. **Add entries under the appropriate category:**
   - `### Added` - New features
   - `### Changed` - Changes in existing functionality
   - `### Deprecated` - Soon-to-be removed features
   - `### Removed` - Removed features
   - `### Fixed` - Bug fixes
   - `### Security` - Security improvements
3. **Format:** Use bullet points with clear, concise descriptions
4. **Example format:**
[Unreleased]
### Added
- New endpoint for user profile management

### Fixed
- Fixed race condition in authentication middleware

5. **Link to PR/Issue:** If applicable, reference the issue number (e.g., `- Fixed login bug (#123)`)

## Quality Standards

- **Zero tolerance for warnings:** All Clippy warnings must be fixed or explicitly allowed with reasoning
- **Test coverage:** All new features must include corresponding tests
- **Documentation:** Public APIs require documentation comments
- **Error handling:** Prefer `Result` types over panics in library code
- **Changelog entries:** Every functional change must have a corresponding changelog entry

## Workflow

1. **Verify environment setup** (protobuf-compiler and Bun installed)
2. **Install dependencies** (run `bun install` in frontend if working with frontend code)
3. Identify which directories contain modified files
4. Run appropriate validation commands for each directory
5. **Update CHANGELOG.md with changes under `[Unreleased]` section**
6. Report any failures with clear error messages
7. Only proceed to commit if all checks pass
8. Include validation results and changelog updates in PR description
