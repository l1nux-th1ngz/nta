#!/bin/bash

# inst.sh

# Set variables
_repo="https://raw.githubusercontent.com/l1nux-th1ngz/nta/main/"

downloadNtaCpp() {
    echo "Downloading the latest version of nta.cpp..."
    ntaCppURL="${_repo}nta.cpp"
    curl -O "$ntaCppURL"

    if [ $? -eq 0 ]; then
        echo "Download complete."
    else
        echo "Download failed."
        exit 1
    fi
}

main() {
    echo "inst.sh"
    downloadNtaCpp

    # Additional code for the rest of your script goes here.
}

main
