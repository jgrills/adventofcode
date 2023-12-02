#! /bin/bash

g++ -Wall -Wno-sign-compare -o exe -g3 -std=c++2a source.cpp helper.cpp -lpthread > stdout 2> stderr
result=$?
cat stdout stderr
exit $result
