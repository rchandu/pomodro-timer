# Contributing to Pomodoro Timer

## Commit Message Format

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for automated versioning and release management.

### Commit Message Structure

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature (triggers minor version bump)
- **fix**: A bug fix (triggers patch version bump)
- **perf**: A performance improvement (triggers patch version bump)
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools and libraries
- **ci**: Changes to CI configuration files and scripts

### Breaking Changes

To trigger a major version bump, add `BREAKING CHANGE:` in the footer or add `!` after the type:

```
feat!: remove deprecated API endpoint

BREAKING CHANGE: The old /api/v1 endpoint has been removed. Use /api/v2 instead.
```

### Examples

```bash
# Feature (minor version bump)
git commit -m "feat: add notification sound settings"

# Bug fix (patch version bump)
git commit -m "fix: timer not resetting after break"

# Performance improvement (patch version bump)
git commit -m "perf: optimize timer rendering"

# Documentation
git commit -m "docs: update installation instructions"

# Breaking change (major version bump)
git commit -m "feat!: redesign timer configuration API"
```

### Scopes

You can optionally add a scope to provide more context:

- `ui`: User interface changes
- `timer`: Timer functionality
- `settings`: Application settings
- `notifications`: Notification system
- `build`: Build system changes

Examples:
```bash
git commit -m "feat(ui): add dark mode toggle"
git commit -m "fix(timer): resolve pause button state issue"
```

## Branching Strategy

### Branch Types

- **master**: Development branch - creates pre-release versions (v1.2.3-rc.1)
- **release**: Stable release branch - creates stable versions (v1.2.3)
- **production**: Production branch - creates stable versions (v1.2.3)

### Workflow

1. Create feature branches from `master`
2. Make commits using conventional commit format
3. Create PR to `master` for development releases
4. Merge `master` to `release` or `production` for stable releases

### Release Process

- **Push to master**: Creates pre-release (e.g., v1.2.3-rc.1)
- **Push to release/production**: Creates stable release (e.g., v1.2.3)

Releases are automatically created based on commit messages:
- `fix:` commits → patch version (1.0.1)
- `feat:` commits → minor version (1.1.0)
- `BREAKING CHANGE` → major version (2.0.0)

## Development Setup

1. Install dependencies:
   ```bash
   bun install
   ```

2. Start development server:
   ```bash
   bun run dev
   ```

3. Run tests and linting:
   ```bash
   bun run check:all
   ```

4. Format code:
   ```bash
   bun run fix:all
   ```

## Skipping Releases

To skip creating a release for a commit, add `[skip ci]` to the commit message or use the `no-release` scope:

```bash
git commit -m "chore(no-release): update README formatting"
```