#!/bin/bash

# NtaUpgrade script in Bash

# Set variables
_repo="https://raw.githubusercontent.com/l1nux-th1ngz/nta/main/"

downloadLatestVersion() {
    echo "Downloading the latest version of Nta..."
    latestVersionURL="${_repo}nta.py"
    curl -O "$latestVersionURL"

    if [ $? -eq 0 ]; then
        echo "Download complete."
    else
        echo "Download failed."
        exit 1
    fi
}

main() {
    echo "Nta Upgrader"
    downloadLatestVersion

    # Additional code for the rest of your script goes here.
}

main
