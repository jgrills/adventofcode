.PHONY: test
default: short

prog: source.cpp
	g++ -o prog -std=c++17 source.cpp

.PHONY: test
test: prog
	./prog < input

.PHONY: short
short: prog
	./prog < short
