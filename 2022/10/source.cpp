#include "helper.h"

#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <map>
#include <set>
#include <string>
#include <string_view>
#include <vector>

#define DAY            10
#define USE_LONG_INPUT 1
#define VERBOSE        2

constexpr int day = DAY;

#if USE_LONG_INPUT
const char* input_filename = "long";
#else
const char* input_filename = "short";
#endif

#define VV   if constexpr (VERBOSE > 0)
#define VVV  if constexpr (VERBOSE > 1)
#define VVVV if constexpr (VERBOSE > 2)

using namespace std;

set<int> interesting{20, 60, 100, 140, 180, 220};

int reg = 1;
int cycle = 1;
int result = 0;
void ticka()
{
    if (interesting.count(cycle) != 0)
    {
        outln(' ', "cycle", cycle, reg);
        result += (reg * cycle);
    }
    ++cycle;
}

void Day10a()
{
    std::ifstream input_file(input_filename);
    int total = 0;
    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        string_view line = line_string;

        vector<string_view> tokens = Parse(line, ' ');
        string_view command = tokens[0];
        if (command == "noop")
        {
            //outln(' ', "noop");
            ticka();
        }
        else if (command == "addx")
        {
            int value = atoi(tokens[1]);
            //outln(' ', "addx", value);
            ticka();
            ticka();
            reg += value;
        }
        else assert(false);
    }
    outln(' ', "result", result);
}

char output [12][40];
void tickb()
{
    int row = cycle / 40;
    int column = cycle % 40;

    if (cycle >= 240) return;

    assert(row >= 0 && row < 12);
    assert(column >= 0 && column < 40);

    bool on = abs(reg - column) < 2;
    if (on)
        output[row][column] = '#';
    else
        output[row][column] = ' ';

    // outln(' ', c, row, column, reg, on);
    ++cycle;
}

void Day10b()
{
    cycle = 0;
    memset(output, ' ', sizeof(output));

    std::ifstream input_file(input_filename);
    int total = 0;
    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        string_view line = line_string;

        vector<string_view> tokens = Parse(line, ' ');
        string_view command = tokens[0];
        if (command == "noop")
        {
            //outln(' ', "noop");
            tickb();
        }
        else if (command == "addx")
        {
            int value = atoi(tokens[1]);
            //outln(' ', "addx", value);
            tickb();
            tickb();
            reg += value;
        }
        else assert(false);
    }

#if 1
    for (int y = 0; y < 12; ++y)
    {
        for (int x = 0; x < 40; ++x)
          cout << output[y][x];
        cout << "\n";
    }
#endif
}

int main()
{
    Day10b();
    return 0;
}
