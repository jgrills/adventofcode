#include "helper.h"

#include <assert.h>
#include <algorithm>
#include <charconv>
#include <cstring>
#include <fstream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <string_view>
#include <vector>

using namespace std;

#define DAY            11
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

typedef string_view sv;

const string cright = "\033[38;2;255;0;0m";
const string cleft = "\033[38;2;0;255;255m";
const string cup = "\033[38;2;0;255;0m";
const string cdown = "\033[38;2;255;0;255m";
const string cnormal = "\033[38;2;127;127;127m";

const string c_up = cup;
const string c_down = cright;
const string c_level = "\033[38;2;255;255;255m";

vector<string> m;
int seen[50][200];
string color[50][200];
struct Loc { int x; int y; char ch; };
bool operator==(const Loc &lhs, const Loc &rhs)
{
    return (lhs.x == rhs.x && lhs.y == rhs.y);
}

typedef vector<Loc> Path;
bool operator<(const Path& lhs, const Path& rhs)
{
    return lhs.size() > rhs.size();
}
typedef priority_queue<Path> PathQueue;
PathQueue paths;

void AddPath(Path p, Loc step)
{
    Loc where = p.back();

    if (seen[where.y][where.x] == 0)
    {
        char f = m[where.y][where.x];
        if (f == 'S') f = 'a';

        where.x += step.x;
        where.y += step.y;
        where.ch = step.ch;
        if (where.x < 0 || where.y < 0 || where.x >= m[0].size() || where.y >= m.size()) return;
        char t = m[where.y][where.x];
        if (t == 'E') t = 'z';
        if (t <= f+1)
        {
            p.push_back(where);
            paths.push(p);
            // outln(" ", "Add", where.x, where.y);
        }
    }
}

void Day12()
{
    Loc start, end;

    ifstream input_file(input_filename);
    bool stop = false;
    while(!stop)
    {
        string str;
        if (!getline(input_file, str)) break;
        outln(' ', str);
        auto s = str.find('S');
        if (s!= string::npos) {
            start.x = s;
            start.y = m.size();
            start.ch = 'S';
        }
        auto e = str.find('E');
        if (e != string::npos) {
            end.x = e;
            end.y = m.size();
            end.ch = 'E';
        }
        m.push_back(str);
    }
    outln(' ', "map", m[0].size(), m.size(), "=size", start.x, start.y, "=start", end.x, end.y, "=end");

    Path p;
    p.push_back(start);
    paths.push(p);
    Path solve;
    for(;;)
    {
        Path p = paths.top();
        paths.pop();

        Loc where = p.back();
        if (where == end)
        {
            solve = p;
            break;
        }

        AddPath(p, Loc{0, -1, '^'});
        AddPath(p, Loc{1, 0, '>'});
        AddPath(p, Loc{-1, 0, '<'});
        AddPath(p, Loc{0, 1, 'v'});
        seen[where.y][where.x] = p.size();
    }

    for (int i = 1; i < solve.size()-1; ++i)
    {
        Loc& prev = solve[i-1];
        Loc& l = solve[i];
        Loc step{ l.x - prev.x, l.y - prev.y };

        char prev_char = m[prev.y][prev.x];
        char cur_char = m[l.y][l.x];

        auto&c = color[l.y][l.x];
        assert(c.empty());
        if (prev_char == cur_char) c = c_level;
        else if (prev_char < cur_char) c = c_up;
        else if (prev_char > cur_char) c = c_down;
        else assert(false);

#if 0
        auto&c = color[prev.y][prev.x];
        if (step == Loc{1,0}) c = cright;
        else if (step == Loc{-1,0}) c = cleft;
        else if (step == Loc{0,1}) c = cup;
        else if (step == Loc{0,-1}) c = cdown;
        else assert(false);
#endif
    }

    for (int j = 0; j < m.size(); ++j)
    {
        const auto& line = m[j];
        for (int i = 0; i < line.size(); ++i)
        {
            out("", color[j][i], line[i], cnormal);
        }
        outln("", "");
    }
    outln(' ', "done", solve.size()-1);
}

void AddPathB(Path p, Loc step)
{
    Loc where = p.back();

    if (seen[where.y][where.x] == 0)
    {
        char t = m[where.y][where.x];
        if (t == 'S') t = 'a';

        where.x += step.x;
        where.y += step.y;
        where.ch = step.ch;
        if (where.x < 0 || where.y < 0 || where.x >= m[0].size() || where.y >= m.size()) return;
        char f = m[where.y][where.x];
        if (f == 'E') f = 'z';
        if (t <= f+1)
        {
            p.push_back(where);
            paths.push(p);
            // outln(" ", "Add", where.x, where.y);
        }
    }
}
void Day12b()
{
    Loc start, end;

    ifstream input_file(input_filename);
    bool stop = false;
    while(!stop)
    {
        string str;
        if (!getline(input_file, str)) break;
        outln(' ', str);
        auto s = str.find('S');
        if (s!= string::npos) {
            start.x = s;
            start.y = m.size();
            start.ch = 'S';
        }
        auto e = str.find('E');
        if (e != string::npos) {
            end.x = e;
            end.y = m.size();
            end.ch = 'E';
        }
        m.push_back(str);
    }
    outln(' ', "map", m[0].size(), m.size(), "=size", start.x, start.y, "=start", end.x, end.y, "=end");

    Path p;
    p.push_back(end);
    paths.push(p);
    Path solve;
    for(;;)
    {
        assert(!paths.empty());
        Path p = paths.top();
        paths.pop();

        Loc where = p.back();
        if (m[where.y][where.x] == 'a')
        {
            solve = p;
            break;
        }

        AddPathB(p, Loc{0, -1, '^'});
        AddPathB(p, Loc{1, 0, '>'});
        AddPathB(p, Loc{-1, 0, '<'});
        AddPathB(p, Loc{0, 1, 'v'});
        seen[where.y][where.x] = p.size();
    }

    for (int i = 1; i < solve.size()-1; ++i)
    {
        Loc& prev = solve[i-1];
        Loc& l = solve[i];
        Loc step{ l.x - prev.x, l.y - prev.y };

        char prev_char = m[prev.y][prev.x];
        char cur_char = m[l.y][l.x];

        auto&c = color[l.y][l.x];
        assert(c.empty());
        if (prev_char == cur_char) c = c_level;
        else if (prev_char < cur_char) c = c_up;
        else if (prev_char > cur_char) c = c_down;
        else assert(false);

#if 0
        auto&c = color[prev.y][prev.x];
        if (step == Loc{1,0}) c = cright;
        else if (step == Loc{-1,0}) c = cleft;
        else if (step == Loc{0,1}) c = cup;
        else if (step == Loc{0,-1}) c = cdown;
        else assert(false);
#endif
    }

    for (int j = 0; j < m.size(); ++j)
    {
        const auto& line = m[j];
        for (int i = 0; i < line.size(); ++i)
        {
            out("", color[j][i], line[i], cnormal);
        }
        outln("", "");
    }
    outln(' ', "done", solve.size()-1);
}

int main()
{
    Day12b();
    return 0;
}
