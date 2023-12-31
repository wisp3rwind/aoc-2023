#! /usr/bin/env sh

set -x
set -e

day=$1

if [ -z $day ]; then
    exit 1
fi

path="day$day"

cp -a "skeleton" "$path"

sed -i -e "s/dayXX/day$day/" "${path}/Cargo.toml"
sed -i -e "s/dayXX/day$day/" "${path}/src/main.rs"

sed -i -e "s/#\\(\"day$day\\)/\\1/" "Cargo.toml"

mkdir "$path/data"
touch "$path/data/test1.txt"
touch "$path/data/test2.txt"
touch "$path/data/input.txt"
