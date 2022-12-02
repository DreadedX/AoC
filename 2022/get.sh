#!/bin/bash
source ../.env
mkdir -p input/$1
curl "https://adventofcode.com/2022/day/$1/input" -H "Cookie: session=${SESSION}" > input/$1/input
