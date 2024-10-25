#!/bin/bash

update_function() {
    echo "Regular update"
    git commit -m "add description && c++ code"
    cd ProgressTracker
    cargo run --release -- .. .
    git add progress.svg
    git commit -m "update"
    cargo clean
    echo "All job finish, prepare to push"
}


if [ "$1" ]; then
    case "$1" in
        update)
            update_function
            ;;
        *)
            echo "Unknown paremeter: $1"
            bash script.sh
            ;;
    esac
else
    echo "Instruction: $0"
    echo "update  |  One regular update, add the new \"README.md\" by yourself."
fi