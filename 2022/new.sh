#!/bin/sh
day=$1
type=$2
default=$3
test=$4

echo "Creating file from template..."
sed -e "s/DAY/$day/g" -e "s/TYPE/$type/" -e "s/DEFAULT/$default/" -e "s/TEST/$test/" ./template.rs > ./src/bin/day${day}.rs

echo "Downloading input..."
source ../.env
mkdir -p input/$1
curl -s "https://adventofcode.com/2022/day/$1/input" -H "Cookie: session=${SESSION}" > input/$1/input

echo "Done!"
