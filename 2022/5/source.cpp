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

void PartA()
{
    std::ifstream input_file(input_filename);

    std::string line_string;
    if (!std::getline(input_file, line_string)) assert(false);
    std::string_view line = line_string;
    int columns = atoi(line);
    if (!std::getline(input_file, line_string)) assert(false);
    line = line_string;
    int rows = atoi(line);

    std::vector<char> stack[10];
    for (int x = rows-1; x >= 0; --x)
    { 
        if (!std::getline(input_file, line_string)) assert(false);
        line = line_string;
        for (int y = 0; y < columns; ++y)
        {
            int index = y * 4 + 1;
            char ch = line[index];
            if (ch != ' ') {
                if (stack[y].empty()) stack[y].resize(x+1);
                stack[y][x] = ch;
                std::cout << "stack  " << y << " " << ch << "\n";
            }
        }
    }

    if (!std::getline(input_file, line_string)) assert(false);
    if (!std::getline(input_file, line_string)) assert(false);

    for (;;)
    {
        if (!std::getline(input_file, line_string)) break;
        line = line_string;
        std::vector<std::string_view> command = Parse(line, ' ');
        assert(command.size() == 6);
        int count = atoi(command[1]);
        int from = atoi(command[3]) - 1;
        int to = atoi(command[5]) - 1;
        std::cout << "command: " << count << " " << from << " " << to << "\n";

        std::cout << "moving";
        for (int j = 0; j < count; ++j)
        {
            char ch = stack[from].back();
            stack[to].push_back(ch);
            stack[from].pop_back();
            std::cout << " " << ch;
        }
        std::cout << "\n";
    }

    for (int y = 0; y < columns; ++y)
    {
        std::cout << stack[y].back();
    }
    std::cout << "\n";
}

void PartB()
{
    std::ifstream input_file(input_filename);

    std::string line_string;
    if (!std::getline(input_file, line_string)) assert(false);
    std::string_view line = line_string;
    int columns = atoi(line);
    if (!std::getline(input_file, line_string)) assert(false);
    line = line_string;
    int rows = atoi(line);

    std::vector<char> stack[10];
    for (int x = rows-1; x >= 0; --x)
    { 
        if (!std::getline(input_file, line_string)) assert(false);
        line = line_string;
        for (int y = 0; y < columns; ++y)
        {
            int index = y * 4 + 1;
            char ch = line[index];
            if (ch != ' ') {
                if (stack[y].empty()) stack[y].resize(x+1);
                stack[y][x] = ch;
                std::cout << "stack  " << y << " " << ch << "\n";
            }
        }
    }

    if (!std::getline(input_file, line_string)) assert(false);
    if (!std::getline(input_file, line_string)) assert(false);

    for (;;)
    {
        if (!std::getline(input_file, line_string)) break;
        line = line_string;
        std::vector<std::string_view> command = Parse(line, ' ');
        assert(command.size() == 6);
        int count = atoi(command[1]);
        int from = atoi(command[3]) - 1;
        int to = atoi(command[5]) - 1;
        std::cout << "command: " << count << " " << from << " " << to << "\n";

        std::cout << "moving";
        std::vector<char> temp;
        for (int j = 0; j < count; ++j)
        {
            char ch = stack[from].back();
            temp.push_back(ch);
            stack[from].pop_back();
            std::cout << " " << ch;
        }
        for (int j = 0; j < count; ++j)
        {
            char ch = temp.back();
            stack[to].push_back(ch);
            temp.pop_back();
        }
        std::cout << "\n";
    }

    for (int y = 0; y < columns; ++y)
    {
        std::cout << stack[y].back();
    }
    std::cout << "\n";
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
