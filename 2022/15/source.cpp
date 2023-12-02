#include "helper.h"

#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <map>
#include <string>
#include <string_view>
#include <vector>

#define DAY            15
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

struct Data
{
    int sx;
    int sy;
    int bx;
    int by;
    int r;
};

void Day15a()
{
    ifstream input_file(input_filename);
    int total = 0;
    vector<Data> datas;
    for (int pair = 1;; ++pair)
    {
        string s1;
        if (!getline(input_file, s1)) break;
        for (int ci = 0; ci < s1.size(); ++ci) if (s1[ci] == '=') s1[ci] = ' ';
        for (int ci = 0; ci < s1.size(); ++ci) if (s1[ci] == ',') s1[ci] = ' ';
        for (int ci = 0; ci < s1.size(); ++ci) if (s1[ci] == ':') s1[ci] = ' ';
        sv s{s1};
        vector<sv> tokens = Parse(s, ' ');
        Data d{atoi(tokens[3]), atoi(tokens[6]), atoi(tokens[13]), atoi(tokens[16])};
        d.r = abs(d.sx - d.bx) + abs(d.sy - d.by);
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        outln(" ", d.sx, d.sy, "=sensor", d.bx, d.by, "=beacon", d.r, "=r");
        datas.push_back(d);
    }
 
    int minx = datas[0].sx;
    int miny = datas[0].sy;
    int maxx = minx;
    int maxy = miny;
    for (auto& d : datas)
    {
        minx = min(minx, d.sx);
        miny = min(miny, d.sy);
        minx = min(minx, d.bx);
        miny = min(miny, d.by);
        maxx = max(maxx, d.sx);
        maxy = max(maxy, d.sy);
        maxx = max(maxx, d.bx);
        maxy = max(maxy, d.by);
    }

    int width = maxx - minx;
    outln(' ', minx, miny, maxx, maxy, "=field", width, "=width");
    const int y = 2000000;
    int count = 0;
    minx -= width;
    maxx += width;
    for (int x = minx; x <= maxx; ++x)
    {
        char state = '.';
        for (const auto& d : datas)
        {
            if (x == d.bx && y == d.by)
            {
                state = 'B';
                VVV outln(" ", 'B', x);
                break;
            }

            int r = abs(x - d.sx) + abs(y - d.sy);
            if (r <= d.r)
            {
                state = '#';
                VVV outln(" ", '#', x, r, d.r, d.sx, d.sy);
                count += 1;
                break;
            }
        }
        if (state == '.')
        {
            VVV outln(" ", '.', x);
        }
    }
    outln(' ', count, "count");
}

struct Range
{
    int x0;
    int x1;
};

void Day15b()
{
    ifstream input_file(input_filename);
    int total = 0;
    vector<Data> datas;
    for (int pair = 1;; ++pair)
    {
        string s1;
        if (!getline(input_file, s1)) break;
        for (int ci = 0; ci < s1.size(); ++ci) if (s1[ci] == '=') s1[ci] = ' ';
        for (int ci = 0; ci < s1.size(); ++ci) if (s1[ci] == ',') s1[ci] = ' ';
        for (int ci = 0; ci < s1.size(); ++ci) if (s1[ci] == ':') s1[ci] = ' ';
        sv s{s1};
        vector<sv> tokens = Parse(s, ' ');
        Data d{atoi(tokens[3]), atoi(tokens[6]), atoi(tokens[13]), atoi(tokens[16])};
        d.r = abs(d.sx - d.bx) + abs(d.sy - d.by);
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        outln(" ", d.sx, d.sy, "=sensor", d.bx, d.by, "=beacon", d.r, "=r");
        datas.push_back(d);
    }
 
    const int minx = 0;
    const int miny = 0;
#if USE_LONG_INPUT
    const int maxx = 4000000;
    const int maxy = 4000000;
#else
    const int maxx = 20;
    const int maxy = 20;
#endif
    const int64_t tunex = 4000000;
    int ax = -1;
    int ay = -1;

    for (int y = miny; y <= maxy; ++y)
    {
        vector<Range> ranges;
        int datai = 0;
        for (const auto& d : datas)
        {
            ++datai;
            int deltay = abs(y - d.sy);
            int width = d.r - deltay;
            if (width >= 0)
            {
                Range r {d.sx - width, d.sx + width};
                // outln(' ', "  addrange", datai, r.x0, r.x1);
                ranges.push_back(r);
            }
        }

        int rx0 = 0; int rx1 = 0;
        for (;;)
        {
            bool applied = false;
            for (int i = 0; i < ranges.size(); )
            {
                Range& r = ranges[i];
                if (r.x0 <= rx1)
                {
                    if (r.x1 > rx1)
                    {
                        // outln(' ', "  expand", rx0, rx1, "by", r.x0, r.x1);
                        rx1 = r.x1;
                    }
                    ranges.erase(ranges.begin()+i);
                    applied = true;
                }
                else
                    ++i;
            }
            if (!applied)
            {
                if (ranges.empty())
                    break;
                int64_t result = (int64_t)(rx1+1) * tunex + (int64_t)y;
                outln(' ', "  ", rx1+1, y, result, "BROKEN");
                return;
            }
        }
    }
}

int main()
{
    Day15b();
    return 0;
}
