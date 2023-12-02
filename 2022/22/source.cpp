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

#define DAY            22
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

const int block_dimension = 50;
constexpr int MY = 300;
constexpr int MX = 200;

struct Row
{
    char buffer[MX];
    int s0{-1};
    int s1{-1};
};
struct Col
{
    int s0{-1};
    int s1{-1};
};

struct Map
{
    Row rows[MY];
    Col cols[MX];
    int x{0};
    int y{0};
};
Map m;

const Vec2 dirvec[4] = { { 1, 0 }, { 0, 1 }, { -1, 0 }, { 0, -1 } };
const int R[4]{1, 2, 3, 0};
const int L[4]{3, 0, 1, 2};
const sv ESWN[4] = { "E", "S", "W", "N" };
Vec2 block_indices[] = { { 1, 0, }, {2, 0}, {1, 1}, {0, 2}, {1 , 2}, {0, 3} };

enum class Side { right, bottom, left, top };

struct Edge
{
    int from_block;
    Side from_side;
    int to_block;
    Side  to_side;
    bool flipped;
};

const Edge edges[] =
{
    { 1, Side::top,    6, Side::left, false },
    { 1, Side::left,   4, Side::left, true },

    { 2, Side::top,    6, Side::bottom, false },
    { 2, Side::right,  5, Side::right, true },
    { 2, Side::bottom, 3, Side::right, false },

    { 3, Side::right,  2, Side::bottom, false },
    { 3, Side::left, 4, Side::top, false },

    { 4, Side::top,  3, Side::left, false },
    { 4, Side::left, 1, Side::left, true },

    { 5, Side::right,  2, Side::right, true },
    { 5, Side::bottom, 6, Side::right, false },

    { 6, Side::right,  5, Side::bottom, false },
    { 6, Side::bottom, 2, Side::top, false },
    { 6, Side::left,  1, Side::top, false }
};

void Range(int face, int dir_ind, bool flip, Vec2 &p0, Vec2 &p1)
{
        Vec2 topleft = block_indices[face] * Vec2{block_dimension, block_dimension};
        switch (dir_ind)
        {
            case Side::right:
                p0 = topleft + Vec2{block_dimension, 0};
                p1 = topleft + Vec2{block_dimension, block_dimension};
                break;
            case Side::bottom:
                p0 = topleft + Vec2{0, block_dimension};
                p1 = topleft + Vec2{block_dimension, block_dimension};
                break;
            case Side::left:
                p0 = topleft;
                p1 = topleft + Vec2{0, block_dimension};
                break;
            case Side::top:
                p0 = topleft;
                p1 = topleft + Vec2{block_dimension, 0};
                break;
        }
        if (flip)
            std::swap(p0, p1);
}

void Wrapper()
{
    for (const Edge& e : edges)
    {
        
    }
};

void Day22(deque<sv> &file_lines)
{
    int& y = m.y;

    sv path = file_lines.back();
    file_lines.pop_back();
    file_lines.pop_back();

    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        assert(line.size() < MX);

        Row &row = m.rows[y];
        for (int x = 0; x < line.size(); ++x)
        {
            Col &col = m.cols[x];
            char ch = line[x];
            row.buffer[x] = ch;
            if (ch == '.' || ch == '#') 
            {
                if (row.s0 < 0)
                    row.s0 = x;
                if (col.s0 < 0)
                    col.s0 = y;
                col.s1 = y+1;
                row.s1 = x+1;
            }
        }
        ++y;
        assert(y < MY);
    }

    for (int yy = 0; yy < y; ++yy)
    {
        Row &r = m.rows[yy];
        outln(" ", "row", yy, r.s0, r.s1, r.buffer);
    }

    for (int xx = 0; xx < m.x; ++xx)
    {
        Col &c = m.cols[xx];
        outln(" ", "col", xx, c.s0, c.s1);
    }

    Vec2 at{m.rows[0].s0, 0};
    int dir = 0;
    while (!path.empty())
    {
        outln(" ", "at", at.y, at.x, ESWN[dir]);
        int olddir = dir;
        char f = path.front();
        if (f == 'R')
        {
            path.remove_prefix(1);
            dir = R[dir];
            outln(" ", "R", olddir, dir);
        }
        else if (f == 'L')
        {
            path.remove_prefix(1);
            dir = L[dir];
            outln(" ", "R", olddir, dir);
        }
        else
        {
            int steps = ParseInt(path);
            outln(" ", "steps", steps);

            for (int i = 0; i < steps; ++i)
            {
                Vec2 next = at;
                next += dirvec[dir];
                const Row& r = m.rows[at.y];
                const Col& c = m.cols[at.x];
                int r0 = m.rows[at.y].s0;
                int r1 = m.rows[at.y].s1;
                int c0 = m.cols[at.x].s0;
                int c1 = m.cols[at.x].s1;
                if (next.x == r1) next.x = r0;
                if (next.x < r0) next.x = r1-1;
                if (next.y == c1) next.y = c0;
                if (next.y < c0) next.y = c1-1;
                assert(next.x >= r0 && next.x < r1);
                assert(next.y >= c0 && next.y < c1);
                char ch = m.rows[next.y].buffer[next.x];
                assert(ch == '.' || ch == '#');
                if (ch == '#') break;
                at = next;
            }
        }
    }
    int score = (at.y+1) * 1000 + (at.x+1) * 4 + dir;
    outln(" ", "done", at.y+1, at.x+1, dir, score);
}

void Day22b(deque<sv> &file_lines)
{
    int& y = m.y;

    sv path = file_lines.back();
    file_lines.pop_back();
    file_lines.pop_back();

    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        assert(line.size() < MX);

        Row &row = m.rows[y];
        for (int x = 0; x < line.size(); ++x)
        {
            Col &col = m.cols[x];
            char ch = line[x];
            row.buffer[x] = ch;
            if (ch == '.' || ch == '#') 
            {
                if (row.s0 < 0)
                    row.s0 = x;
                if (col.s0 < 0)
                    col.s0 = y;
                col.s1 = y+1;
                row.s1 = x+1;
            }
        }
        ++y;
        assert(y < MY);
    }

    for (int yy = 0; yy < y; ++yy)
    {
        Row &r = m.rows[yy];
        outln(" ", "row", yy, r.s0, r.s1, r.buffer);
    }

    for (int xx = 0; xx < m.x; ++xx)
    {
        Col &c = m.cols[xx];
        outln(" ", "col", xx, c.s0, c.s1);
    }

    Vec2 at{m.rows[0].s0, 0};
    int dir = 0;
    while (!path.empty())
    {
        outln(" ", "at", at.y, at.x, ESWN[dir]);
        int olddir = dir;
        char f = path.front();
        if (f == 'R')
        {
            path.remove_prefix(1);
            dir = R[dir];
            outln(" ", "R", olddir, dir);
        }
        else if (f == 'L')
        {
            path.remove_prefix(1);
            dir = L[dir];
            outln(" ", "R", olddir, dir);
        }
        else
        {
            int steps = ParseInt(path);
            outln(" ", "steps", steps);

            for (int i = 0; i < steps; ++i)
            {
                Vec2 next = at;
                next += dirvec[dir];
                const Row& r = m.rows[at.y];
                const Col& c = m.cols[at.x];
                int r0 = m.rows[at.y].s0;
                int r1 = m.rows[at.y].s1;
                int c0 = m.cols[at.x].s0;
                int c1 = m.cols[at.x].s1;
                if (next.x == r1) next.x = r0;
                if (next.x < r0) next.x = r1-1;
                if (next.y == c1) next.y = c0;
                if (next.y < c0) next.y = c1-1;
                assert(next.x >= r0 && next.x < r1);
                assert(next.y >= c0 && next.y < c1);
                char ch = m.rows[next.y].buffer[next.x];
                assert(ch == '.' || ch == '#');
                if (ch == '#') break;
                at = next;
            }
        }
    }
    int score = (at.y+1) * 1000 + (at.x+1) * 4 + dir;
    outln(" ", "done", at.y+1, at.x+1, dir, score);
}


int main()
{
    unbuffer(cout);
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    deque<sv> file_lines = ParseLines(file_contents);

    Day22b(file_lines);
    return 0;
}
