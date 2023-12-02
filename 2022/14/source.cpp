#include "helper.h"

#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <map>
#include <string>
#include <string_view>
#include <vector>

#define DAY            14
#define USE_LONG_INPUT 1
#define VERBOSE        2

using namespace std;
typedef string_view sv;

constexpr int day = DAY;

#if USE_LONG_INPUT
const char* input_filename = "long";
#else
const char* input_filename = "short";
#endif

#define VV   if constexpr (VERBOSE > 0)
#define VVV  if constexpr (VERBOSE > 1)
#define VVVV if constexpr (VERBOSE > 2)

struct Point { int x; int y; };
bool operator!=(const Point& lhs, const Point& rhs)
{
    return lhs.x != rhs.x || lhs.y != rhs.y;
}

char space[2000][2000];

int sandx = 500;
int sandy = 0;
int minx = sandx;
int miny = sandy;
int maxx = miny;
int maxy = miny;

void Draw()
{
    for (int y = miny; y <= maxy; ++y)
    {
        for (int x = minx; x <= maxx; ++x)
        {
            out("", space[y][x]);
        }
        outln("", "");
    }
}

bool DropSand()
{
    int sx = sandx;
    int sy = sandy;

    for (;;)
    {
        if (sy > maxy) return false;
        if (space[sy+1][sx] == ' ')
        {
            sy += 1;
            continue;
        }
        if (space[sy+1][sx-1] == ' ')
        {
            sy += 1;
            sx -= 1;
            continue;
        }
        if (space[sy+1][sx+1] == ' ')
        {
            sy += 1;
            sx += 1;
            continue;
        }
        break;
    }
    if (sx < minx) minx = sx;
    if (sx > maxx) maxx = sx;

    space[sy][sx] = 'o';
    return true;
}

bool DropSandB()
{
    int sx = sandx;
    int sy = sandy;

    for (;;)
    {
        assert(sx >= 1);
        assert(sx <= 2000);
        if (sy > maxy) assert(false);
        if (space[sy+1][sx] == ' ')
        {
            sy += 1;
            continue;
        }
        if (space[sy+1][sx-1] == ' ')
        {
            sy += 1;
            sx -= 1;
            continue;
        }
        if (space[sy+1][sx+1] == ' ')
        {
            sy += 1;
            sx += 1;
            continue;
        }
        break;
    }
    if (sx == sandx && sy == sandy) return false;
    space[sy][sx] = 'o';
    return true;
}


void Day14()
{
    memset(space, ' ', sizeof(space));
    ifstream input_file(input_filename);
    int total = 0;

    space[miny][minx] = '+';
    for (;;)
    {
        string line_string;
        if (!getline(input_file, line_string)) break;
        sv line = line_string;

        vector<Point> points;
        vector<sv> pts = Parse(line, ' ');
        bool first = true;
        for (auto& pt : pts)
        {
            vector<sv> xy = Parse(pt, ',');
            Point p{atoi(xy[0]), atoi(xy[1])};
            if (p.x < minx) minx = p.x;
            if (p.y < miny) miny = p.y;
            if (p.x > maxx) maxx = p.x;
            if (p.y > maxy) maxy = p.y;
            outln(' ', "p:", p.x, p.y);
            points.push_back(p);
        }

        Point back = points.front();
        space[back.y][back.x] = '#';
        for (auto& next : points)
        {
            while (back != next)
            {
                if (next.x > back.x) back.x += 1;
                else if (next.x < back.x) back.x -= 1;
                if (next.y > back.y) back.y += 1;
                else if (next.y < back.y) back.y -= 1;
                space[back.y][back.x] = '#';
            }
        }
    }

    outln(' ', "range:", minx, miny, maxx, maxy);
    Draw();

    int i = 0;
    for (;;)
    {
        if (DropSand())
        {
            ++i;
        }
        else
        {
            break;
        }
    }
    Draw();
    outln(' ', "done", i);
}


void Day14b()
{
    memset(space, ' ', sizeof(space));
    ifstream input_file(input_filename);
    int total = 0;

    space[miny][minx] = '+';
    for (;;)
    {
        string line_string;
        if (!getline(input_file, line_string)) break;
        sv line = line_string;

        vector<Point> points;
        vector<sv> pts = Parse(line, ' ');
        bool first = true;
        for (auto& pt : pts)
        {
            vector<sv> xy = Parse(pt, ',');
            Point p{atoi(xy[0]), atoi(xy[1])};
            if (p.x < minx) minx = p.x;
            if (p.y < miny) miny = p.y;
            if (p.x > maxx) maxx = p.x;
            if (p.y > maxy) maxy = p.y;
            outln(' ', "p:", p.x, p.y);
            points.push_back(p);
        }

        Point back = points.front();
        space[back.y][back.x] = '#';
        for (auto& next : points)
        {
            while (back != next)
            {
                if (next.x > back.x) back.x += 1;
                else if (next.x < back.x) back.x -= 1;
                if (next.y > back.y) back.y += 1;
                else if (next.y < back.y) back.y -= 1;
                space[back.y][back.x] = '#';
            }
        }
    }

    minx -= 10;
    maxx += 10;
    maxy += 2;
    for (int x = 0; x < 2000; ++x) space[maxy][x] = '#';

    outln(' ', "range:", minx, miny, maxx, maxy);
    Draw();

    int i = 0;
    for (;;)
    {
        if (DropSandB())
        {
            ++i;
        }
        else
        {
            ++i;
            break;
        }
    }
    Draw();
    outln(' ', "done", i);
}

int main()
{
    Day14b();
    return 0;
}
