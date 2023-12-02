#! /bin/bash

g++ -Wall -Wno-sign-compare -o exe -g3 -std=c++17 source.cpp helper.cpp > stdout 2> stderr
result=$?
cat stdout stderr
exit $?
