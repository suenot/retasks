---
number: 1
title: Implement file watcher for two-way synchronization
state: open
labels: [enhancement, priority-medium]
---

## Description

We need to implement a file watcher for two-way synchronization between GitHub issues and local files.

## Requirements

- Watch local directory for file changes
- Detect when a local markdown file is modified
- Parse the file's frontmatter and content
- Sync changes back to GitHub issues
- Handle concurrency properly

## Implementation Details

Currently thinking of using the `hotwatch` crate to monitor file system events. When a file is modified, we'll need to:

1. Read the file
2. Parse frontmatter and body
3. Update the corresponding GitHub issue via the API
4. Handle any errors gracefully

This should work alongside the existing GitHub-to-local sync to provide complete two-way synchronization.

## Tasks

- [ ] Research file watching libraries
- [ ] Implement file change detection
- [ ] Add parser for frontmatter
- [ ] Create GitHub issue update logic
- [ ] Test with various issue types and changes 