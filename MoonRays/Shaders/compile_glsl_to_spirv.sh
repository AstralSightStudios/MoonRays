#!/bin/bash

# This script compiles all files except .bat, .sh, and .spirv files in the current directory to SPIR-V using glslc.

# Check if glslc is available
if ! command -v glslc &> /dev/null
then
    echo "glslc not found. Please install it and make sure it's in your PATH."
    exit 1
fi

# Loop through all files in the current directory
for file in *; do
    # Skip directories
    if [ -d "$file" ]; then
        continue
    fi
    
    # Get the file extension
    ext="${file##*.}"
    
    # Skip .bat, .sh, and .spirv files
    if [[ "$ext" != "bat" && "$ext" != "sh" && "$ext" != "spirv" ]]; then
        # Get the filename without extension
        filename="${file%.*}"
        
        echo "Compiling $file to $filename.spirv..."
        glslc "$file" -o "$filename.spirv"
        if [ $? -ne 0 ]; then
            echo "Failed to compile $file"
            exit 1
        fi
    fi
done

echo "All eligible files compiled successfully."