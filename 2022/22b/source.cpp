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

const int dimension = 50;
const int block_dimension = 50;
const Vec2 block_dim{block_dimension,block_dimension};
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

const int ind_right = 0;
const int ind_down = 1;
const int ind_left = 2;
const int ind_up = 3;
const Vec2 dir_right{ 1, 0 };
const Vec2 dir_down{ 0, 1 };
const Vec2 dir_left{ -1, 0 };
const Vec2 dir_up { 0, -1 };
const Vec2 dirvec[4] = { dir_right, dir_down, dir_left, dir_up };
const int R[4]{1, 2, 3, 0};
const int L[4]{3, 0, 1, 2};
const sv ESWN[4] = { "E", "S", "W", "N" };
const Vec2 block_indices[] = { { 1, 0 }, {2, 0}, {1, 1}, {0, 2}, {1 , 2}, {0, 3} };

struct Edge
{
    int from_block;
    int from_step_ind;
    int to_block;
    int to_step_ind;
    bool flip;
};

const Edge edges[] =
{
    { 1, ind_up,     6, ind_right, false },
    { 1, ind_left,   4, ind_right, true },

    { 2, ind_up,     6, ind_up,    false },
    { 2, ind_right,  5, ind_left,  true },
    { 2, ind_down,   3, ind_left,  false },

    { 3, ind_right,  2, ind_up,    false },
    { 3, ind_left,   4, ind_down,  false },

    { 4, ind_up,     3, ind_right, false },
    { 4, ind_left,   1, ind_right, true },

    { 5, ind_right,  2, ind_left,  true },
    { 5, ind_down,   6, ind_left,  false },

    { 6, ind_right,  5, ind_up,    false },
    { 6, ind_down,   2, ind_down,  false },
    { 6, ind_left,   1, ind_down,  false }
};

int InBlock(const Vec2 &p0)
{
    for (int i = 0; i < 6; ++i)
    {
        Vec2 topleft = block_indices[i] * block_dim;
        Vec2 bottomright = topleft + block_dim;
        if (p0.x >= topleft.x && p0.x < bottomright.x && p0.y >= topleft.y && p0.y < bottomright.y)
            return i + 1;
    }
    return -1;
}

void GetEdge(int at_block, int step_ind, bool flip, Vec2 &p0, Vec2 &p1, Vec2 &edge_step)
{
    Vec2 topleft = block_indices[at_block-1] * block_dim;
    if (step_ind == ind_right)
    {
        p0 = topleft + Vec2{block_dimension-1, 0};
        p1 = topleft + Vec2{block_dimension-1, block_dimension-1};
    }
    else if (step_ind == ind_down)
    {
        p0 = topleft + Vec2{0, block_dimension-1};
        p1 = topleft + Vec2{block_dimension-1, block_dimension-1};
    }
    else if (step_ind == ind_left)
    {
        p0 = topleft;
        p1 = topleft + Vec2{0, block_dimension-1};
    }
    else if (step_ind == ind_up)
    {
        p0 = topleft;
        p1 = topleft + Vec2{block_dimension-1, 0};
    }
    else
        assert(false);

    if (flip)
        std::swap(p0, p1);

    edge_step = (p1 - p0).sign();
}

void RemapStep(int at_block, Vec2 at_xy, int at_ind, int &next_block, Vec2 &next, int &next_ind)
{
    const Edge* edge{nullptr};
    for (const Edge& e : edges)
    {
            if (e.from_block == at_block && e.from_step_ind == at_ind)
            {
                edge = &e;
                break;
            }
    
    }
    assert(edge);

    next_block = edge->to_block;
    next_ind = edge->to_step_ind;
    Vec2 f0, f1, fstep;
    GetEdge(at_block, at_ind, false, f0, f1, fstep);
    Vec2 t0, t1, tstep;
    GetEdge(edge->to_block, (edge->to_step_ind + 2) % 4, edge->flip, t0, t1, tstep);
    assert(at_ind == edge->from_step_ind);

    outln(' ', "remap", at_block, at_xy, ESWN[at_ind], "[", f0, f1, fstep, "]=edge ->", next_block, ESWN[next_ind], "[", t0, t1, tstep, "]");

    Vec2 f2 = f0;
    Vec2 t2 = t0;
    for (int i = 0; i < dimension; ++i)
    {
        if (f2 == at_xy)
        {
            next = t2;
            outln(" ", "edge progress", i, f2, t2);
            return;
        }
        f2 += fstep;
        t2 += tstep;
    }
    assert(false);
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

    Vec2 at_xy{m.rows[0].s0, 0};
    int at_ind = 0;
    int at_block = InBlock(at_xy);
    while (!path.empty())
    {
        outln(" ", "at_xy", at_xy, ESWN[at_ind]);
        int olddir = at_ind;
        char f = path.front();
        if (f == 'R')
        {
            path.remove_prefix(1);
            at_ind = R[at_ind];
        }
        else if (f == 'L')
        {
            path.remove_prefix(1);
            at_ind = L[at_ind];
        }
        else
        {
            int steps = ParseInt(path);

            for (int i = 0; i < steps; ++i)
            {
                const Vec2 &step = dirvec[at_ind];
                Vec2 next_xy = at_xy + step;
                int next_block = InBlock(next_xy);
                int next_ind = at_ind;

                // outln(' ', "testing step", i, ':', at_block, at_xy, ESWN[at_ind], next_xy, next_block);
                if (next_block < 0)
                {
                    RemapStep(at_block, at_xy, at_ind, next_block, next_xy, next_ind);
                    outln(" ", "next", at_xy, at_block, at_ind, next_xy, next_block);

                    int back_block, back_ind;
                    Vec2 back_xy;
                    RemapStep(next_block, next_xy, (next_ind+2)%4, back_block, back_xy, back_ind);
                    // outln(" ", "back", at_block, at_xy, at_ind, back_block, back_xy, (back_ind+2)%4);
                    assert(at_block == back_block && at_xy == back_xy && at_ind == (back_ind+2)%4);
                }

                char ch = m.rows[next_xy.y].buffer[next_xy.x];
                assert(ch == '.' || ch == '#');
                if (ch == '#')
                    break;
                at_block = next_block;
                at_xy = next_xy;
                at_ind = next_ind;
            }
        }
    }
    int score = (at_xy.y+1) * 1000 + (at_xy.x+1) * 4 + at_ind;
    outln(" ", "done", at_xy.y+1, at_xy.x+1, at_ind, score);
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
