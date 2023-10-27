#!/bin/bash

# Compile the C++ code
g++ -o nta_upgrader nta_upgrader.cpp -lcurl

if [ $? -eq 0 ]; then
    # Run the compiled program
    ./nta_upgrader

    # Clean up the executable
    rm nta_upgrader
else
    echo "Compilation failed."
fi
