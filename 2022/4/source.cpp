#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <string_view>
#include <vector>

#define USE_PART_A     1
#define USE_PART_B     0
#define USE_LONG_INPUT 1
#define VERBOSE        1

#if USE_LONG_INPUT
const char* input_filename = "long";
#else
const char* input_filename = "short";
#endif

#define VV if constexpr (VERBOSE > 0)
#define VVV if constexpr (VERBOSE > 1)

// Maximum value expected in the data
constexpr int c_max = 256;

int atoi(std::string_view input)
{
    int out;
    const std::from_chars_result result = std::from_chars(input.data(), input.data() + input.size(), out);
    assert(result.ec == std::errc());
    assert(result.ptr == input.data() + input.length());
    return out;
}

struct SplitResult
{
    std::string_view before;
    std::string_view after;
};
SplitResult Split(std::string_view text, char separator)
{
    const int index = text.find(separator);
    if (index == std::string_view::npos)
        return SplitResult{ text, std::string_view{} };

    std::string_view before = text.substr(0, index);
    std::string_view after = text.substr(index+1);
    return SplitResult{ before, after };
}

bool isspace(char ch, char separator)
{
    return (std::isspace(ch) && ch != separator);
}

std::vector<std::string_view> Parse(std::string_view text, char separator)
{
    std::vector<std::string_view> result;
    while (!text.empty())
    {
        while (!text.empty() && isspace(text.front(), separator))
            text.remove_prefix(1);

        if (!text.empty())
        {
            auto [first, rest] = Split(text, separator);
            while (!first.empty() && isspace(first.back(), separator))
                first.remove_suffix(1);

            result.push_back(first);
            text = rest;
        }
    }

    return result;
}

struct Data
{
    Data(std::string_view text);
    int data[c_max]{};
};

Data::Data(std::string_view text)
{
    auto [t0, t1] = Split(text, '-');
    int n0 = atoi(t0);
    int n1 = atoi(t1);
    VVV { std::cout << n0 << "=start " << n1 << "=stop" << " num\n"; }
    for (int i = n0; i <= n1; ++i) data[i] = 1;
}

bool Contains(Data &d0, Data &d1)
{
    for (int i = 0; i < c_max; ++i)
        if (d1.data[i] && !d0.data[i])
            return false;
    return true;
}

bool Intersects(Data &d0, Data &d1)
{
    for (int i = 0; i < c_max; ++i)
        if (d0.data[i] && d1.data[i])
            return true;
    return false;
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
        auto [t0, t1] = Split(line, ',');
        Data d0(t0), d1(t1);
        bool result = Contains(d0, d1) || Contains(d1, d0);
        VV { std::cout << result << "=result " << total << "=total " << t0 << " " << t1 << "\n"; }
        total += result;
    }
    std::cout << "PartA: " << total << "\n";
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
        auto [t0, t1] = Split(line, ',');
        Data d0(t0), d1(t1);
        bool result = Intersects(d0, d1);
        VV { std::cout << result << "=result " << total << "=total " << t0 << " " << t1 << "\n"; }
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
