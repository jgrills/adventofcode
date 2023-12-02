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

#define DAY            24
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

struct Elf
{
    int i;
    int x;
    int y;
    int direction;
};
struct Cell
{
    int destinations;
    Elf* elf;
};
Cell m[1000][1000];
vector<Elf> elves;
const Vec2 offsets[5] = { {0,-1}, {0,1}, {-1,0}, {1,0}, {0,0} };

void Print(int x0, int y0, int width, int height)
{
    for (int y = 0; y < height; ++y)
    {
        for (int x = 0; x < width; ++x)
        {
            out("", m[y0+y][x0+x].elf ? '#' : '.');
            assert(m[y0+y][x0+x].destinations == 0);
        }
        outln("", "");
    }
}

void Day23(deque<sv> &file_lines)
{
    int y = 500;
    while (!file_lines.empty())
    {
        int x = 500;
        sv line = file_lines.front(); file_lines.pop_front();
        for (char ch : line)
        {
            if (ch == '.')
            {
            }
            else if (ch == '#')
            {
                elves.push_back(Elf{(int)elves.size(), x, y, 4});
                m[y][x].elf = &elves.back();
            }
            else
                assert(false);
            ++x;
        }
        ++y;
    }

    outln("", "elves:", elves.size());
    outln("", "starting map:");
    // Print(497, 498, 14, 12);

    for (int round = 1; round <= 10; ++round)
    {
        for (Elf& elf : elves)
        {
            elf.direction = 4;
            int neighbors = 0;
            bool open[4] = { true, true, true, true };
            for (int y = -1; y <= 1; ++y)
                for (int x = -1; x <= 1; ++x)
                {
                    if (x != 0 || y != 0)
                    {
                        if (m[elf.y+y][elf.x+x].elf)
                        {
                            if (y == -1) open[0] = false;
                            if (y ==  1) open[1] = false;
                            if (x == -1) open[2] = false;
                            if (x ==  1) open[3] = false;
                            ++neighbors;
                        }
                    }
                }

            if (neighbors != 0)
            {
                for (int o = 0; o < 4; ++o)
                {
                    int try_direction = (o + round-1) % 4;
                    if (open[try_direction])
                    {
                        const Vec2 &off = offsets[try_direction];
                        elf.direction = try_direction;
                        m[elf.y+off.y][elf.x+off.x].destinations += 1;
                        break;
                    }
                }
            }
        }

        for (Elf& elf : elves)
        {
            if (elf.direction < 4)
            {
                const Vec2 &off = offsets[elf.direction];
                if (m[elf.y+off.y][elf.x+off.x].destinations == 1)
                {
                    m[elf.y][elf.x].elf = nullptr;
                    elf.x += off.x;
                    elf.y += off.y;
                    elf.direction = 4;
                    m[elf.y][elf.x].elf = &elf;
                    m[elf.y][elf.x].destinations = 0;
                }
            }
        }

        int x0 = elves[0].x;
        int y0 = elves[0].y;
        int x1 = elves[0].x;
        int y1 = elves[0].y;
        for (Elf& elf : elves)
        {
            x0 = min(x0, elf.x);
            x1 = max(x1, elf.x);
            y0 = min(y0, elf.y);
            y1 = max(y1, elf.y);

            if (elf.direction < 4)
            {
                const Vec2 &off = offsets[elf.direction];
                m[elf.y+off.y][elf.x+off.x].destinations = 0;
                elf.direction = 4;
            }
        }

        int w = (x1 - x0) + 1;
        int h = (y1 - y0) + 1;
        outln(" ", "round", round, x0, y0, w, h, (w * h) - static_cast<int>(elves.size()));
        // Print(497, 498, 14, 12);
    }

    outln(" ", "done");
}


void Day23b(deque<sv> &file_lines)
{
    int y = 500;
    while (!file_lines.empty())
    {
        int x = 500;
        sv line = file_lines.front(); file_lines.pop_front();
        for (char ch : line)
        {
            if (ch == '.')
            {
            }
            else if (ch == '#')
            {
                elves.push_back(Elf{(int)elves.size(), x, y, 4});
                m[y][x].elf = &elves.back();
            }
            else
                assert(false);
            ++x;
        }
        ++y;
    }

    outln("", "elves:", elves.size());
    outln("", "starting map:");
    // Print(497, 498, 14, 12);

    bool any = true;
    for (int round = 1; any; ++round)
    {
        any = false;

        for (Elf& elf : elves)
        {
            elf.direction = 4;
            int neighbors = 0;
            bool open[4] = { true, true, true, true };
            for (int y = -1; y <= 1; ++y)
                for (int x = -1; x <= 1; ++x)
                {
                    if (x != 0 || y != 0)
                    {
                        if (m[elf.y+y][elf.x+x].elf)
                        {
                            if (y == -1) open[0] = false;
                            if (y ==  1) open[1] = false;
                            if (x == -1) open[2] = false;
                            if (x ==  1) open[3] = false;
                            ++neighbors;
                        }
                    }
                }

            if (neighbors != 0)
            {
                for (int o = 0; o < 4; ++o)
                {
                    int try_direction = (o + round-1) % 4;
                    if (open[try_direction])
                    {
                        const Vec2 &off = offsets[try_direction];
                        elf.direction = try_direction;
                        m[elf.y+off.y][elf.x+off.x].destinations += 1;
                        break;
                    }
                }
            }
        }

        for (Elf& elf : elves)
        {
            if (elf.direction < 4)
            {
                const Vec2 &off = offsets[elf.direction];
                if (m[elf.y+off.y][elf.x+off.x].destinations == 1)
                {
                    m[elf.y][elf.x].elf = nullptr;
                    elf.x += off.x;
                    elf.y += off.y;
                    elf.direction = 4;
                    m[elf.y][elf.x].elf = &elf;
                    m[elf.y][elf.x].destinations = 0;
                    any = true;
                }
            }
        }

        int x0 = elves[0].x;
        int y0 = elves[0].y;
        int x1 = elves[0].x;
        int y1 = elves[0].y;
        for (Elf& elf : elves)
        {
            x0 = min(x0, elf.x);
            x1 = max(x1, elf.x);
            y0 = min(y0, elf.y);
            y1 = max(y1, elf.y);

            if (elf.direction < 4)
            {
                const Vec2 &off = offsets[elf.direction];
                m[elf.y+off.y][elf.x+off.x].destinations = 0;
                elf.direction = 4;
            }
        }

        int w = (x1 - x0) + 1;
        int h = (y1 - y0) + 1;
        outln(" ", "round", round, x0, y0, w, h, (w * h) - static_cast<int>(elves.size()));
    }

    outln(" ", "done");
}

int main()
{
    unbuffer(cout);
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    deque<sv> file_lines = ParseLines(file_contents);

    Day23b(file_lines);
    return 0;
}
