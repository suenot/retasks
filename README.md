# RetTasks - GitHub Issues Sync

RetTasks is a command-line tool that synchronizes GitHub issues with a local directory. It allows you to manage your GitHub issues offline as Markdown files and automatically syncs changes back to GitHub.

## Features

* **Two-way synchronization**: Changes made to GitHub issues are synchronized to local files, and vice versa
* **Continuous monitoring**: Watch mode automatically syncs changes in both directions
* **Markdown format**: Issues are stored as Markdown files with YAML frontmatter
* **Command-line interface**: Simple CLI with configurable options

## Installation

### Prerequisites

* Rust and Cargo installed on your system
* A GitHub Personal Access Token with permissions to access and modify issues

### Building from source

```bash
git clone https://github.com/yourusername/retasks.git
cd retasks
cargo build --release
```

The compiled binary will be located at `target/release/retasks`.

## Usage

### Basic Usage

For a one-time synchronization:

```bash
retasks --token YOUR_GITHUB_TOKEN --repo username/repository
```

### Watch Mode

To continuously monitor changes and sync automatically:

```bash
retasks --token YOUR_GITHUB_TOKEN --repo username/repository --watch
```

### Available Options

* `--issues-dir DIR`: Sets the directory for storing issues (default: `./issues`)
* `--watch`: Enables watch mode for continuous synchronization
* `--token TOKEN`: GitHub API token (required)
* `--repo OWNER/REPO`: GitHub repository in format `owner/repo` (required)
* `--interval SECONDS`: Sync interval in seconds when using watch mode (default: 300)

## File Format

Each GitHub issue is stored as a separate Markdown file with YAML frontmatter. The filename format is `issue-{number}.md`.

Example:

```markdown
---
number: 123
title: Bug in authentication module
state: open
labels: [bug, high-priority]
---

Detailed description of the issue in Markdown format...

Additional notes and steps to reproduce.
```

## Security

Your GitHub token is sensitive information. Never commit it to version control. Consider using environment variables or a secure configuration file to store your token.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 