#!/bin/bash

# Example script to run Retasks in watch mode
# Make sure to set up your environment variables first

# Verify that environment variables are set
if [ -z "$GITHUB_TOKEN" ] || [ -z "$GITHUB_REPO" ]; then
    echo "ERROR: GITHUB_TOKEN and GITHUB_REPO environment variables must be set"
    echo "Example:"
    echo "  export GITHUB_TOKEN=your_token_here"
    echo "  export GITHUB_REPO=username/repository"
    exit 1
fi

# Set default values for optional variables
ISSUES_DIR=${ISSUES_DIR:-"./issues"}
SYNC_INTERVAL=${SYNC_INTERVAL:-300}

# Create issues directory if it doesn't exist
mkdir -p "$ISSUES_DIR"

# Run Retasks in watch mode
echo "Starting Retasks for $GITHUB_REPO..."

# If running from the project directory
cargo run -- \
    --token "$GITHUB_TOKEN" \
    --repo "$GITHUB_REPO" \
    --issues-dir "$ISSUES_DIR" \
    --interval "$SYNC_INTERVAL" \
    --watch

# If running the compiled binary (uncomment this section instead)
# ./target/release/retasks \
#     --token "$GITHUB_TOKEN" \
#     --repo "$GITHUB_REPO" \
#     --issues-dir "$ISSUES_DIR" \
#     --interval "$SYNC_INTERVAL" \
#     --watch 