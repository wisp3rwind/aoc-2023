#! /usr/bin/env sh

set -x
set -e

day=$1

path="day$day"

cp -a "skeleton" "$path"

sed -i -e "s/dayXX/day$day/" "${path}/Cargo.toml"

mkdir "$path/data"
touch "$path/data/test.txt"
touch "$path/data/input.txt"
