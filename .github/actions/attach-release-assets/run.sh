#!/bin/bash
if [[ -z "$GITHUB_TOKEN" ]]; then
    echo "ERROR: the GITHUB_TOKEN env variable wasn't set"
    exit 1
fi

# The docker entrypoint arg is "inputs.asset"
ASSET=$1
if [ ! -f "$ASSET" ]; then
    echo "ERROR: cannot find the asset '$ASSET'"
fi

AUTH_HEADER="Authorization: token ${GITHUB_TOKEN}"
RELEASE_ID=$(jq --raw-output '.release.id' "$GITHUB_EVENT_PATH")
FILENAME=$(basename "${ASSET}")
UPLOAD_URL="https://uploads.github.com/repos/${GITHUB_REPOSITORY}/releases/${RELEASE_ID}/assets?name=${FILENAME}"

echo "Asset URL: $UPLOAD_URL"
echo "Uploading asset '$ASSET'..."

mkdir curl_output
response_code=$(curl \
    -sSL \
    -XPOST \
    -H "${AUTH_HEADER}" \
    --upload-file "${ASSET}" \
    --header "Content-Type:application/octet-stream" \
    --write-out "%{http_code}" \
    --output curl_output \
    "$UPLOAD_URL")

if [ $response_code -ge 400 ]; then
    echo "ERROR: curl upload failed with status code $response_code"
    cat curl_output && rm curl_output
    exit 1
fi

cat curl_output | jq .
rm curl_output

