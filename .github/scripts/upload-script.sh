#!/bin/sh
#
# Upload binary artifacts to github releases.
# Mostly taken from https://github.com/skx/github-action-publish-binaries
#

# Check if GITHUB_TOKEN secret is set
test -z "$GITHUB_TOKEN" && echo "Set the GITHUB_TOKEN env variable!" && exit 1

# Ensure that there is a pattern specified.
test -z "$1" && echo "Artifacts should be specified!" && exit 1

# Have we found any artifacts?
found=
for file in $*; do
    if [ -e "$file" ]; then
        found=1
    fi
done

# Abort if missing.
test -z "${found}" && echo "Artifacts are missing!" && exit 1

# Prepare the headers for our curl-command.
AUTH_HEADER="Authorization: token ${GITHUB_TOKEN}"

# Create the correct Upload URL.
RELEASE_ID=$(jq --raw-output '.release.id' "$GITHUB_EVENT_PATH")

# For every file
for file in $*; do

    echo "Processing file ${file}"

    test ! -e "$file" && echo " file not found - skipping." && continue

    test ! -s "$file" && echo " file is empty - skipping." && continue


    FILENAME=$(basename "${file}")

    UPLOAD_URL="https://uploads.github.com/repos/${GITHUB_REPOSITORY}/releases/${RELEASE_ID}/assets?name=${FILENAME}"
    echo "Upload URL: ${UPLOAD_URL}"

    # Generate a temporary file.
    tmp=$(mktemp)

    # Upload the artifact - capturing HTTP response-code in our output file.
    response=$(curl \
        -sSL \
        -XPOST \
        -H "${AUTH_HEADER}" \
        --upload-file "${file}" \
        --header "Content-Type:application/octet-stream" \
        --write-out "%{http_code}" \
        --output $tmp \
        "${UPLOAD_URL}")

    # If the curl-command returned a non-zero response we must abort
    if [ "$?" -ne 0 ]; then
        echo " curl command did not return zero. Aborting..."
        cat $tmp
        rm $tmp
        exit 1
    fi

    # If upload is not successful, we must abort
    if [ $response -ge 400 ]; then
        echo " upload was not successful. Aborting..."
        echo " HTTP status is $response"
        cat $tmp
        rm $tmp
        exit 1
    fi

    # Show pretty output, since we already have jq
    cat $tmp | jq .
    rm $tmp

done