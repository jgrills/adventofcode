#include "helper.h"

#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <string_view>
#include <vector>

#define USE_PART_A     0
#define USE_PART_B     1
#define USE_LONG_INPUT 1
#define VERBOSE        2

#if USE_LONG_INPUT
const char* input_filename = "long";
#else
const char* input_filename = "short";
#endif

#define VV   if constexpr (VERBOSE > 0)
#define VVV  if constexpr (VERBOSE > 1)
#define VVVV if constexpr (VERBOSE > 2)

// Data range
constexpr int c_min = 0;
constexpr int c_size = 256;
struct Data
{
    Data(std::string_view text);
    int data[c_size]{};
};

Data::Data(std::string_view text)
{
}

int PartA(std::string_view line)
{
    int len = line.length();
    for (int i = 3; i < len; ++i)
        if (
            line[i-3] != line[i-2] &&
            line[i-3] != line[i-1] &&
            line[i-3] != line[i-0] &&
            line[i-2] != line[i-1] &&
            line[i-2] != line[i-0] &&
            line[i-1] != line[i-0])
            return i+1;

    assert(false);
    return 0;
}

int PartB(std::string_view line)
{
    int len = line.length();
    for (int i = 13; i < len; ++i)
    {
        bool result = true;
        for (int j = 13; j >= 0; --j)
            for (int k = j - 1; k >= 0; --k)
                result = result && line[i-j] != line[i-k];
        if (result) return i + 1;
    }

    assert(false);
    return 0;

}


void PartA()
{
    std::ifstream input_file(input_filename);
    int total = 0;
    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        std::string_view line = line_string;

        Data d0(line);
        int result = PartA(line);
        VV { std::cout << result << "=result " << total << "=total " << line << "\n"; }
        total += result;
    }
    std::cout << "PartA: " << total << "\n";
}

int PartB(Data &d0)
{
    return 1;
}

void PartB()
{
    std::ifstream input_file(input_filename);
    int total = 0;
    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        std::string_view line = line_string;

        Data d0(line);
        int result = PartB(line);
        VV { std::cout << result << "=result " << total << "=total " << line << "\n"; }
        total += result;
    }
    std::cout << "PartB: " << total << "\n";

}

int main()
{
#if USE_PART_A
    PartA();
#endif
#if USE_PART_B
    PartB();
#endif
    return 0;
}
