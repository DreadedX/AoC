#!/bin/bash
mkdir -p build
g++ -Wall -Wextra -Og -g 1/main.cpp -std=c++20 -o build/challenge1
g++ -Wall -Wextra -Og -g 2/main.cpp -std=c++20 -o build/challenge2
