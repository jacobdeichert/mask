#!/bin/bash
set -e # Exit on error

if [[ -z "$GITHUB_TOKEN" ]]; then
    echo "ERROR: The GITHUB_TOKEN env variable wasn't set."
    exit 1
fi

# The docker entrypoint arg is "inputs.asset"
asset=$1

echo "curl: $(which curl)"
echo "jq: $(which jq)"

echo "UPLOADING ASSET..."
echo "FILE: $asset"
echo "DONE TEST RUN"
