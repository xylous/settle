#!/usr/bin/env bash

# Create a directory with $1 or 1000 nonempty files and then run `settle` on it

mkdir test || exit 1
cd test

num="1000"

if [[ ! -z "$1" ]]; then
    num="$1"
fi

echo "testing on ${num} files..."

for i in $(seq "${num}"); do
    j=$((i + 1))
    y=$((i - 1))
    touch "foo$i.md"
    echo "[foo$j](foo$j.md)" > "foo$i.md"
    echo "[foo$y](foo$y.md)" > "foo$i.md"
done

settle generate
settle backlinks
settle build

cd .. && rm -rf test
