# Pomodoro Timer

A lightweight, cross-platform Pomodoro timer built with Tauri (Rust backend) and SolidJS frontend. Features native desktop notifications and a clean, minimal interface.

## Features

- **Standard Pomodoro Technique**: 25-minute work sessions, 5-minute short breaks, 15-minute long breaks
- **Native Desktop Notifications**: Get notified when sessions complete
- **Cross-platform**: Runs on Linux, Windows, and macOS
- **Lightweight**: Built in tauri

## Tech Stack

- **Backend**: Rust with Tauri framework
- **Frontend**: SolidJS with TypeScript
- **Build Tool**: Vite
- **Notifications**: Native platform notifications (libnotify on Linux, Toast on Windows, Notification Center on macOS)

## Prerequisites

- [Bun](https://bun.sh/) (latest stable)
- [Rust](https://rustup.rs/) (latest stable)
- Platform-specific dependencies:
  - **Linux**: `webkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`
  - **Windows**: WebView2 (usually pre-installed on Windows 10/11)
  - **macOS**: Xcode Command Line Tools

## Setup Instructions

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd pomodro-timer
   ```

2. **Install dependencies:**

   ```bash
   bun install
   ```

3. **Run in development mode:**

   ```bash
   bun run dev
   ```

4. **Build for production:**

   ```bash
   bun run build
   ```

5. **Generate platform-specific bundles:**
   ```bash
   bun run tauri build
   ```

## Usage

1. **Start a Work Session**: Click "Work (25m)" to begin a 25-minute focus session
2. **Take Breaks**: Use "Short Break (5m)" or "Long Break (15m)" for rest periods
3. **Timer Controls**: Pause, resume, or stop the timer as needed
4. **Notifications**: Desktop notifications will alert you when sessions complete

## Development

The project follows Tauri's architecture:

- **Frontend** (`src/`): SolidJS components and styling
- **Backend** (`src-tauri/src/`): Rust timer logic and system integration
- **Configuration** (`src-tauri/tauri.conf.json`): App settings and permissions

## Code Quality

This project uses linting and formatting tools to maintain code quality:

### TypeScript/SolidJS
- **ESLint**: Linting with TypeScript and SolidJS rules
- **Prettier**: Code formatting

### Rust
- **Clippy**: Advanced linting for Rust code
- **rustfmt**: Code formatting for Rust

### Available Scripts

```bash
# Check all code quality tools
bun run check:all

# Fix all auto-fixable issues
bun run fix:all

# TypeScript/Frontend linting
bun run lint              # Check for lint issues
bun run lint:fix          # Fix auto-fixable lint issues
bun run format            # Format code with Prettier
bun run format:check      # Check if code is formatted

# Rust linting
bun run lint:rust         # Run Clippy on Rust code
bun run format:rust       # Format Rust code
bun run format:rust:check # Check if Rust code is formatted
```

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- Install ESLint and Prettier extensions for VS Code

## Release Strategy

This project uses semantic versioning with automated releases:

- **`main`**: Development branch with RC prereleases (e.g., `1.2.0-rc.1`)
- **`release/production`**: Production releases (e.g., `1.2.0`)

Releases are automatically generated using conventional commits:
- `feat:` → minor version bump
- `fix:` → patch version bump  
- `BREAKING CHANGE:` → major version bump

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
