#!/bin/bash

# Define the output file path
OUTPUT_FILE="$HOME/recording.mp4"

# Check if wf-recorder is already running
if pgrep -x "wf-recorder" > /dev/null; then
    # If it's running, terminate the process
    pkill wf-recorder
else
    # If it's not running, remove the existing file (if any) to avoid prompts
    rm -f "$OUTPUT_FILE"
    
    # Start wf-recorder in the background
    wf-recorder -f "$OUTPUT_FILE" &
fi
