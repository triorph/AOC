#!/bin/bash
set -euo pipefail
# TODO:
# - write the script to download, using this URL format and curl:
#   https://adventofcode.com/2022/day/10/input
# Other considerations:
if [[ -z $SESSION ]]; then
  echo 'Need to provide a session token for auth in the environment variable $SESSION'
  exit 1
fi
if [[ $(which curl) == "" ]]; then
  echo "Need to have curl installed for this script to run"
  exit 1
fi
for day in day*; do
  if [[ ! -f "./$day/data/input_data.txt" ]]; then
    echo "Downloading file for day $day"
    curl \
      -s -b session="$SESSION" \
      -A "AOC download script for triorph@gmail.com" \
      -o "$day/data/input_data.txt" \
      "https://adventofcode.com/2024/day/${day:3:2}/input"

  else
    echo "File already exists for day $day"
  fi
done
