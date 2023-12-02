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

#define DAY            9
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

struct Where
{
    int x{0};
    int y{0};
};

bool operator<(const Where &lhs, const Where &rhs)
{
    if (lhs.x < rhs.x) return true;
    if (lhs.x > rhs.x) return false;
    if (lhs.y < rhs.y) return true;
    return false;
}
Where operator+(const Where &lhs, const Where &rhs)
{
    return Where{lhs.x + rhs.x, lhs.y + rhs.y};
}

void Day9a()
{
    std::ifstream input_file(input_filename);
    int total = 0;

    int headx = 0, heady = 0;
    int tailx = 0, taily = 0;

    std::set<Where> wheres;
    Where w0{tailx, taily};
    wheres.insert(w0);

    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        std::string_view line = line_string;

        std::vector<std::string_view> tokens = Parse(line, ' ');
        std::string_view dir = tokens[0];
        int count = atoi(tokens[1]);

        int dirx = 0;
        int diry = 0;
        if (dir == "R") dirx = 1;
        else if (dir == "L") dirx = -1;
        else if (dir == "U") diry = 1;
        else if (dir == "D") diry = -1;
        else assert(false);

        for (int i = 0; i < count; ++i)
        {
            headx += dirx;
            heady += diry;

            int dx = abs(headx - tailx);
            int dy = abs(heady - taily);
            if (dx > 1 || dy > 1)
            {
                if (headx > tailx)
                    ++tailx;
                else if (headx < tailx)
                    --tailx;

                if (heady > taily)
                    ++taily;
                else if (heady < taily)
                    --taily;
            }

            Where w{tailx, taily};
            char add = 'n';
            if (wheres.count(w) == 0)
            {
                add = 'y';
                wheres.insert(w);
            }

            outln(' ', add, ' ', headx, heady, "=head", tailx, taily, "=tail", i, "/", count, "=count",  dir, dirx, diry, "=dir");
        }
    }
    outln(' ', "total", wheres.size());

}

void Day9b()
{
    std::ifstream input_file(input_filename);
    int total = 0;

    std::set<Where> wheres;
    Where w0{0, 0};
    wheres.insert(w0);

    Where knots[10];
    Where& first = knots[0];
    Where& last = knots[9];

    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        std::string_view line = line_string;

        std::vector<std::string_view> tokens = Parse(line, ' ');
        std::string_view dir = tokens[0];
        int count = atoi(tokens[1]);

        Where step{};
        if (dir == "R") step = Where{1,0};
        else if (dir == "L") step = Where{-1,0};
        else if (dir == "U") step = Where{0,1};
        else if (dir == "D") step = Where{0,-1};
        else assert(false);

        for (int i = 0; i < count; ++i)
        {
            first = first + step;

            for (int j = 0; j < 9; ++j)
            {
                Where& head=knots[j];
                Where& tail=knots[j+1];

                int dx = abs(head.x - tail.x);
                int dy = abs(head.y - tail.y);
                if (dx > 1 || dy > 1)
                {
                    if (head.x > tail.x)
                        ++tail.x;
                    else if (head.x < tail.x)
                        --tail.x;

                    if (head.y > tail.y)
                        ++tail.y;
                    else if (head.y < tail.y)
                        --tail.y;
                }
            }

            bool n = wheres.count(last) == 0;
            if (n)
                wheres.insert(last);
            outln(' ', n, last.x, last.y);

        }
    }
    outln(' ', "total", wheres.size());
}

int main()
{
    Day9b();
    return 0;
}
