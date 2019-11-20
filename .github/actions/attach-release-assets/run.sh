#!/bin/bash
if [[ -z "$GITHUB_TOKEN" ]]; then
    echo "ERROR: the GITHUB_TOKEN env variable wasn't set"
    exit 1
fi

# A file glob of assets to upload. The docker entrypoint arg is "inputs.assets".
ASSETS_GLOB=$1
AUTH_HEADER="Authorization: token ${GITHUB_TOKEN}"
RELEASE_ID=$(jq --raw-output '.release.id' "$GITHUB_EVENT_PATH")

# Upload each asset file to the GitHub Release
for asset_file in $ASSETS_GLOB; do
    filename=$(basename "$asset_file")
    upload_url="https://uploads.github.com/repos/${GITHUB_REPOSITORY}/releases/${RELEASE_ID}/assets?name=${filename}"

    echo "Uploading asset: $asset_file"

    touch curl_log
    response_code=$(curl \
        -sSL \
        -XPOST \
        -H "${AUTH_HEADER}" \
        --upload-file "${asset_file}" \
        --header "Content-Type:application/octet-stream" \
        --write-out "%{http_code}" \
        --output curl_log \
        "$upload_url")

    if [ $response_code -ge 400 ]; then
        echo "ERROR: curl upload failed with status code $response_code"
        cat curl_log && rm curl_log
        exit 1
    fi
done


