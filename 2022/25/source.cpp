#include "helper.h"

#include <assert.h>
#include <algorithm>
#include <charconv>
#include <cstring>
#include <deque>
#include <fstream>
#include <iomanip>
#include <map>
#include <mutex>
#include <queue>
#include <set>
#include <string>
#include <string_view>
#include <thread>
#include <utility>

#include <vector>

#define DAY            25
#define USE_LONG_INPUT 1
#define VERBOSE        1

constexpr int day = DAY;

using namespace std;
typedef string_view sv;

#if USE_LONG_INPUT
const char* input_filename = "long";
#else
const char* input_filename = "short";
#endif

#define VV   if constexpr (VERBOSE > 0)
#define VVV  if constexpr (VERBOSE > 1)
#define VVVV if constexpr (VERBOSE > 2)

int64_t a5digit(char ch)
{
    if (ch == '0') return 0;
    if (ch == '1') return 1;
    if (ch == '2') return 2;
    if (ch == '-') return -1;
    if (ch == '=') return -2;
    assert(false);
    return 0;
}

int64_t a5toi(std::string_view text)
{
    int64_t out{0};
    for (char ch : text)
    {
        int64_t digit = a5digit(ch);
        out = (out * 5) + digit;
    }
    return out;
}

void a5(int64_t value, char result[32])
{
    char local[32];
    int places = 0;
    while (value)
    {
        char &ch = local[places]; 
        int digit = value % 5;
        value = value / 5;
        if (digit == 0) ch = '0';
        else if  (digit == 1) ch = '1';
        else if  (digit == 2) ch = '2';
        else
        {
            if  (digit == 3) ch = '=';
            else if  (digit == 4) ch = '-';
            else assert(false);
            value = value + 1;
        }
        ++places;
    }
    for (int i = 0; i < places; ++i)
    {
        result[0+i] = local[places-(1+i)];
    }
    result[places] = '\0';
}

void Day25(deque<sv> &file_lines)
{
    char buffer[32];
    int64_t total = 0;
    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        int64_t n = a5toi(line);
        a5(n, buffer);
        outln(' ', buffer, line, n);
        total += n;
    }
    a5(total, buffer);
    outln(' ', "total", total, buffer);
}

int main()
{
    unbuffer(cout);
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    deque<sv> file_lines = ParseLines(file_contents);

    Day25(file_lines);
    return 0;
}
