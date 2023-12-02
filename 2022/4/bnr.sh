#! /bin/bash

while true; do
    clear
    touch .timestamp
    g++ -o executable -g3 -std=c++17 source.cpp && ./executable
    while [[ .timestamp -nt source.cpp && .timestamp -nt short && .timestamp -nt long ]]; do
      sleep 0.05
    done
done
