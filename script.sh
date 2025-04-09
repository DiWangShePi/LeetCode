#!/bin/bash

add_newest_file() {
    TARGET_DIR="./solutions"

    new_problem=$(git ls-files --others --exclude-standard "$TARGET_DIR")

    if [ -n "$new_problem" ]; then
        echo "Adding new untracked files:"
        echo "$new_problem"
        while IFS= read -r file; do
            git add "$file"
        done <<< "$new_problem"
        git commit -m "add description && c++ solution"
    else
        echo "No untracked files to add in $TARGET_DIR"
    fi
}

choose_function() {
    TARGET_DIR="./solutions"
    BASE_URL="https://github.com/DiWangShePi/LeetCode/tree/main/solutions"

    folders=($(find "$TARGET_DIR" -mindepth 1 -maxdepth 1 -type d))

    if [ ${#folders[@]} -lt 2 ]; then
        echo "Not enough folders to choose from."
        return
    fi

    count=$((RANDOM % 2 + 2))  
    selected=($(printf "%s\n" "${folders[@]}" | shuf -n $count))

    echo "Randomly selected $count folder(s):"
    for folder in "${selected[@]}"; do
        folder_name=$(basename "$folder")
        echo "$BASE_URL/$folder_name/README.md"
    done
}


update_function() {
    echo "Regular update"

    add_newest_file

    if [ -d "progress tracker" ]; then
        cd 'progress tracker'
        cargo run --release
        cargo clean
        cd ..
    else
        echo "'progress tracker' folder not found"
    fi

    if [ -d "docs" ]; then
        cd docs
        git add progress.svg index.html
        git commit -m "update"
        cd ..
    else
        echo "'docs' folder not found"
    fi

    echo "All jobs finished, prepare to push"
}

if [ "$1" ]; then
    case "$1" in
        update)
            update_function
            ;;
        choose)
            choose_function
            ;;
        *)
            echo "Unknown parameter: $1"
            bash "$0"
            ;;
    esac
else
    echo "Instruction: $0"
    echo "update   |  One regular update, add the new \"README.md\" by yourself."
    echo "choose   |  Randomly pick 2 or 3 subfolders under ./solutions"
fi

