#include "helper.h"

#include <assert.h>
#include <algorithm>
#include <charconv>
#include <cstring>
#include <fstream>
#include <map>
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

typedef int64_t Worry;

struct Monkey
{
    vector<Worry> items;
    string arg1;
    string op;
    string arg2;
    Worry div;
    int dt;
    int df;
    int inspect;
};

Monkey monkeys[10];

Worry arg(sv text, Worry old)
{
    if (text == "old") return old;
    return atoi(text);
}

#if 0
void Day11()
{
    int max = 0;
    {
        ifstream input_file(input_filename);
        bool stop = false;
        while(!stop)
        {
            string str[7];
            vector<sv> line[7];
            for (int i = 0; i < 7; ++i)
            {
                if (!getline(input_file, str[i]))
                {
                    if (i == 0 || i == 6) { stop = true; break; }
                    assert(false);
                }
                line[i] = Parse(str[i], ' ');
            }
            int mi = atoi(line[0][1]);
            if (mi > max) max = mi;
            Monkey&m = monkeys[mi];

            out(' ', mi, "=monkey");
            for (int i = 2; i < line[1].size(); ++i)
            {
                m.items.push_back(atoi(line[1][i]));
                out(' ', ' ', m.items.back());
            }
            sv arg1 = line[2][3];
            sv op = line[2][4];
            sv arg2 = line[2][5];
            assert(line[3][1] == "divisible");
            Worry div = atoi(line[3][3]);
            int dt  = atoi(line[4][5]);
            int df  = atoi(line[5][5]);
            outln(' ', ' ', arg1, op, arg2, ' ', /* div, */dt, df);

            m.arg1 = arg1;
            m.op = op;
            m.arg2 = arg2;
            m.div = div;
            m.dt = dt;
            m.df = df;
            assert(dt != mi && df != mi);
        }
    }
    
    for (int round = 1; round < 21; ++round)
    {
        for (int i = 0; i <= max; ++i)
        {
            Monkey& m = monkeys[i];
            for (auto j : m.items)
            {
                Worry lhs = arg(m.arg1, j);
                Worry rhs = arg(m.arg2, j);
                Worry r;
                if (m.op == "+") r = (lhs + rhs) / 3;
                else if (m.op == "*") r = (lhs * rhs) / 3;
                else assert(false);
                bool d = (r % m.div) == 0;
                int dest = d ? m.dt : m.df;
                // outln(' ', "move", i, j, r, m.div, d, dest);
                assert(dest != i);
                assert(dest >= 0 && dest <= max);
                monkeys[dest].items.push_back(r);
                ++m.inspect;
            }
            m.items.clear();
        }

        vector<Worry> values;
        for (int i = 0; i <= max; ++i)
        {
            Monkey& m = monkeys[i];
            out(' ', i, m.inspect, ":");
            for (auto j : m.items)
            {
                // out(' ', "", j);
            }
            outln("", "");
            values.push_back(m.inspect);
        }
        sort(values.begin(), values.end());
        Worry m0 = values[values.size()-2];
        Worry m1 = values[values.size()-1];
        //outln(' ', "final", m0, m1, m0 * m1);
    }
}

#endif

void Day11b()
{
    int max = 0;
    int mod = 1;
    {
        ifstream input_file(input_filename);
        bool stop = false;
        while(!stop)
        {
            string str[7];
            vector<sv> line[7];
            for (int i = 0; i < 7; ++i)
            {
                if (!getline(input_file, str[i]))
                {
                    if (i == 0 || i == 6) { stop = true; break; }
                    assert(false);
                }
                line[i] = Parse(str[i], ' ');
            }
            int mi = atoi(line[0][1]);
            if (mi > max) max = mi;
            Monkey&m = monkeys[mi];

            // out(' ', mi, "=monkey");
            for (int i = 2; i < line[1].size(); ++i)
            {
                m.items.push_back(atoi(line[1][i]));
                //out(' ', ' ', m.items.back());
            }
            sv arg1 = line[2][3];
            sv op = line[2][4];
            sv arg2 = line[2][5];
            assert(line[3][1] == "divisible");
            Worry div = atoi(line[3][3]);
            int dt  = atoi(line[4][5]);
            int df  = atoi(line[5][5]);
            //outln(' ', ' ', arg1, op, arg2, ' ', dt, df);

            m.arg1 = arg1;
            m.op = op;
            m.arg2 = arg2;
            m.div = div;
            m.dt = dt;
            m.df = df;
            assert(dt != mi && df != mi);
            mod *= div;
        }
    }
    outln(' ', "mod", mod);

    for (int round = 1; round < 10001; ++round)
    {
        for (int i = 0; i <= max; ++i)
        {
            Monkey& m = monkeys[i];
            for (auto j : m.items)
            {
                Worry lhs = arg(m.arg1, j);
                Worry rhs = arg(m.arg2, j);
                Worry r;
                if (m.op == "+") r = (lhs + rhs);
                else if (m.op == "*") r = (lhs * rhs);
                else assert(false);
                r %= mod;
                bool d = (r % m.div) == 0;
                int dest = d ? m.dt : m.df;
                // outln(' ', "move", i, j, r, m.div, d, dest);
                assert(dest != i);
                assert(dest >= 0 && dest <= max);
                monkeys[dest].items.push_back(r);
                ++m.inspect;
            }
            m.items.clear();
        }

        if (round == 1 || round == 20 || round % 1000 == 0)
        {
            outln(" ", "round", round);
            vector<Worry> values;
            for (int i = 0; i <= max; ++i)
            {
                Monkey& m = monkeys[i];
                outln(' ', i, m.inspect);
                values.push_back(m.inspect);
            }
            sort(values.begin(), values.end());
            Worry m0 = values[values.size()-2];
            Worry m1 = values[values.size()-1];
            outln(' ', "final", m0, m1, m0 * m1, std::hex, m0 * m1);
        }
    }
}


int main()
{
    Day11b();
    return 0;
}
