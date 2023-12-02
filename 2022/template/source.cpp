#include "helper.h"

#include <algorithm>
#include <array>
#include <assert.h>
#include <charconv>
#include <cstring>
#include <deque>
#include <fstream>
#include <map>
#include <queue>
#include <set>
#include <string_view>
#include <string>
#include <vector>

#define USE_LONG_INPUT 1
#define VERBOSE        1

using namespace std;
typedef string_view sv;

#define VV1 if constexpr (VERBOSE >= 1)
#define VV2 if constexpr (VERBOSE >= 2)
#define VV3 if constexpr (VERBOSE >= 3)
const Vec3 steps[]{
    {1,0,0},
    {-1,0,0},
    {0,1,0},
    {0,-1,0},
    {0,0,1},
    {0,0,-1}
};

int8_t filled[32][32][32];
bool Valid(const Vec3 &v)
{
    if (v.x < 0 || v.y < 0 || v.z < 0 || v.x > 31 || v.y > 31 || v.z > 31)
        return false;
    return true;
}
int8_t& Filled(const Vec3 &v)
{
    return filled[v.z][v.y][v.x];
}

void Flood(const Vec3 &start)
{
    int count = 0;
    assert(Valid(start) && Filled(start) == 0);
    deque<Vec3> search;
    search.push_back(start);
    while (!search.empty())
    {
        Vec3 v = search.front(); search.pop_front();
        if (Valid(v) && Filled(v) == 0)
        {
            Filled(v) = 2;
            ++count;
            for (const Vec3& s : steps)
            {
                Vec3 nv = v + s;
                if (Valid(nv)) search.push_back(nv);
            }
        }
    }
    outln(' ', start, "flooded", count);
}

void PartA(deque<sv> lines)
{
    vector<Vec3> points;
    while (!lines.empty())
    {
        sv line = lines.front(); lines.pop_front();
        vector<sv> tokens = Parse(line, ',');
        Vec3 v{atoi(tokens[0]), atoi(tokens[1]), atoi(tokens[2])};
        Filled(v) = 1; 
        points.push_back(v);
    }

    Vec3 pmin = points.front();
    Vec3 pmax = pmin;
    for (const Vec3&p : points)
    {
        pmin = min(pmin, p);
        pmax = max(pmax, p);
    }
    outln(' ', pmin, "=min", pmax, "=max");

    int sides = 0;
    for (const Vec3 &p : points)
        for (const Vec3 &s : steps)
        {
            Vec3 t = p + s;
            if (!Valid(t) || Filled(t) == 0) sides += 1;
        }
    outln(' ', "sides", sides);
}

void PartB(deque<sv> lines)
{
    vector<Vec3> points;
    while (!lines.empty())
    {
        sv line = lines.front(); lines.pop_front();
        vector<sv> tokens = Parse(line, ',');
        Vec3 v{atoi(tokens[0]), atoi(tokens[1]), atoi(tokens[2])};
        Filled(v) = 1; 
        points.push_back(v);
    }

    Vec3 pmin = points.front();
    Vec3 pmax = pmin;
    for (const Vec3&p : points)
    {
        pmin = min(pmin, p);
        pmax = max(pmax, p);
    }
    outln(' ', pmin, "=min", pmax, "=max");

    for (int z : array<int,2>{pmin.z, pmax.z})
        for (int y : array<int,2>{pmin.y, pmax.y})
            for (int x : array<int,2>{pmin.x, pmax.x})
            {
                Vec3 v{x,y,z};
                if (Filled(v) == 0) Flood(v);
            }

    int sides = 0;
    for (const Vec3 &p : points)
        for (const Vec3 &s : steps)
        {
            Vec3 t = p + s;
            if (!Valid(t) || Filled(t) == 2) sides += 1;
        }
    outln(' ', "sides", sides);
}


void overloaded(char c) { }
void overloaded(signed char c) { }
void overloaded(unsigned char c) { }
void overloaded(int c) { }
void overloaded(signed int c) { }
void overloaded(unsigned int c) { }

int main()
{
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    // SpaceCharacters(file_contents, "=");
    // StripCharacters(file_contents, ":,");
    deque<sv> file_lines = ParseLines(file_contents);

    PartB(file_lines);
    return 0;
}
