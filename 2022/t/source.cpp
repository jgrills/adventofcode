#include "helper.h"

#include <algorithm>
#include <assert.h>
#include <charconv>
#include <cstring>
#include <deque>
#include <fstream>
#include <map>
#include <queue>
#include <string_view>
#include <string>
#include <vector>

#define USE_LONG_INPUT 0
#define VERBOSE        1

using namespace std;
typedef string_view sv;

#define VV1 if constexpr (VERBOSE >= 1)
#define VV2 if constexpr (VERBOSE >= 2)
#define VV3 if constexpr (VERBOSE >= 3)

void PartA(deque<sv> lines)
{
    while (!lines.empty())
    {
        sv line = lines.front(); lines.pop_front();
        outln("", '>', line, '<');
    }
    outln(' ', "Done");
}

int main()
{
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    // SpaceCharacters(file_contents, "=");
    // StripCharacters(file_contents, ":,");
    deque<sv> file_lines = ParseLines(file_contents);

    PartA(file_lines);
    return 0;
}
