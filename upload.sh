#!bin/bash

# This is a script used to update the repository to github

if [ $# -ne 1 ]; then
  echo "Usage: $0 <path>"
  exit 1
fi
path="$1"

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
git add $path
current_time=$(date +"%Y-%m-%d %H:%M:%S")
commit_message="$(basename "$path") - $current_time"
git commit -m "$commit_message"
git push
