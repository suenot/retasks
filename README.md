# Retasks - GitHub Issues Sync

Retasks is a command-line tool that synchronizes GitHub issues with a local directory. It allows you to manage your GitHub issues offline as Markdown files and automatically syncs changes back to GitHub.

## Features

* **Two-way synchronization**: Changes made to GitHub issues are synchronized to local files, and vice versa
* **Continuous monitoring**: Watch mode automatically syncs changes in both directions
* **Markdown format**: Issues are stored as Markdown files with YAML frontmatter
* **Command-line interface**: Simple CLI with configurable options
* **Asynchronous operations**: Uses Tokio for efficient async I/O operations when interacting with the GitHub API

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

### Using Environment Variables

For security and convenience, you can use environment variables instead of passing the token directly on the command line:

```bash
# Set environment variables
export GITHUB_TOKEN=your_token_here
export GITHUB_REPO=username/repository
export ISSUES_DIR=./my-issues  # Optional, defaults to ./issues
export SYNC_INTERVAL=600       # Optional, defaults to 300 seconds

# Run with environment variables
retasks --token "$GITHUB_TOKEN" --repo "$GITHUB_REPO" --issues-dir "$ISSUES_DIR" --interval "$SYNC_INTERVAL" --watch
```

You can create a script like the example provided in `examples/run_sync.sh` to simplify this process.

### Available Options

* `--issues-dir DIR`: Sets the directory for storing issues (default: `./issues`)
* `--watch`: Enables watch mode for continuous synchronization
* `--token TOKEN`: GitHub API token (required)
* `--repo OWNER/REPO`: GitHub repository in format `owner/repo` (required)
* `--interval SECONDS`: Sync interval in seconds when using watch mode (default: 300)

## How It Works

Retasks uses the following process for synchronization:

1. **GitHub to Local**: 
   - Retrieves issues from the GitHub repository using the GitHub API
   - Converts each issue into a Markdown file with YAML frontmatter
   - Saves files to the local directory

2. **Local to GitHub**:
   - Monitors the local directory for file changes (in watch mode)
   - When a file is modified, parses the YAML frontmatter and Markdown content
   - Updates the corresponding issue on GitHub via the API

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

## Implementation Details

Retasks is built with the following technologies:

* **Rust**: For performance, safety, and concurrency
* **Clap**: For command-line argument parsing
* **Octorust**: For GitHub API integration
* **Hotwatch**: For file system monitoring
* **Tokio**: For asynchronous operations
* **Serde**: For serialization/deserialization of data

The application follows a modular design with:
- Clear separation between GitHub API interaction and local file operations
- Asynchronous handling of network requests
- Concurrent monitoring of local file changes and periodic GitHub synchronization
- Proper error handling and reporting

## Security

Your GitHub token is sensitive information. Never commit it to version control. Consider using environment variables as shown above instead of hardcoding your token.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 