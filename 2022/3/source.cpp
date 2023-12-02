#include <cstring>
#include <string>
#include <iostream>
#include <map>
#include <stdlib.h>
#include <assert.h>

int Lookup(char ch) {
    if (ch >= 'a' && ch <= 'z') return 1 + (ch - 'a');
    if (ch >= 'A' && ch <= 'Z') return 27 + (ch - 'A');
    assert(false);
}

char Reverse(int i) {
    assert(i > 0 && i <= 52);
    if (i <= 26) return 'a' + i - 1;
    return 'A' + i - 27;
}

struct Data {
    Data() { memset(data, 0, sizeof(data)); }
    int data[53];
};

void Add(Data &data, std::string_view text) {
    for (char ch : text) {
        int index = Lookup(ch);
        assert(ch == Reverse(index));
        data.data[index] += 1;
    }
}

int Overlap(Data &d0, Data &d1) {
    for (int i = 1; i <= 52; ++i) {
        if (d0.data[i] && d1.data[i]) return i;
    }
    assert(false);
    return 0;
}

int Overlap(Data &d0, Data &d1, Data &d2) {
    for (int i = 1; i <= 52; ++i) {
        if (d0.data[i] && d1.data[i] && d2.data[i]) return i;
    }
    assert(false);
    return 0;
}

void PartA() {

    int total = 0;
    for (;;) {
        std::string line_string;
        if (!std::getline(std::cin, line_string)) break;
        std::string_view line = line_string;

        int half = line.length() / 2;
        std::string_view t0 = line.substr(0, half);
        std::string_view t1 = line.substr(half);

        Data d0, d1;
        Add(d0, t0);
        Add(d1, t1);

        int overlap = Overlap(d0, d1);
        std::cout << "Line: " << overlap << " " << Reverse(overlap) << " " << half << " " << t0 << " " << t1 << "\n";
        total += overlap;
    }
    std::cout << "Total: " << total << "\n";
}

void PartB() {

    int total = 0;
    for (;;) {
        std::string line_string;
        Data d0, d1, d2;

        if (!std::getline(std::cin, line_string)) break;
        std::cout << "d0: " << line_string << "\n";
        Add(d0, line_string);

        if (!std::getline(std::cin, line_string)) assert(false);
        std::cout << "d1: " << line_string << "\n";
        Add(d1, line_string);

        if (!std::getline(std::cin, line_string)) assert(false);
        std::cout << "d2: " << line_string << "\n";
        Add(d2, line_string);

        int overlap = Overlap(d0, d1, d2);
        std::cout << "Line: " << overlap << " " << Reverse(overlap) << "\n";
        total += overlap;
    }
    std::cout << "Total: " << total << "\n";
}

int main() {
    PartA();
    return 0;
}