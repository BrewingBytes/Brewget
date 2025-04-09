# Contributing Guidelines

## Commit Message Format
We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for our commit messages. This leads to more readable messages that are easy to follow when looking through the project history.

### Commit Message Structure
```
<type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

### Types
- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools
- **ci**: Changes to CI configuration files and scripts
- **revert**: Reverts a previous commit

### Branch Naming Convention
Branch names should follow this pattern:
```
<type>/<short-description>
```

Examples:
- feat/user-authentication
- fix/login-validation
- docs/api-endpoints
- refactor/database-queries

### Examples

Good commit messages:
```
feat(auth): add user authentication system
fix(api): handle null response in user service
docs(readme): update installation instructions
```

Bad commit messages:
```
updated code
fixed bug
added new stuff
```
