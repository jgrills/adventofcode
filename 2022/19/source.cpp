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
#include <vector>

#define DAY            19
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

struct Cost
{
    int costs[4]{0};
};
bool operator>=(const Cost &lhs, const Cost &rhs)
{
    return lhs.costs[0] >= rhs.costs[0] && lhs.costs[1] >= rhs.costs[1] && lhs.costs[2] >= rhs.costs[2] && lhs.costs[3] >= rhs.costs[3];
}
Cost operator+(const Cost &lhs, const Cost &rhs)
{
    return Cost{lhs.costs[0] + rhs.costs[0], lhs.costs[1] + rhs.costs[1], lhs.costs[2] + rhs.costs[2], lhs.costs[3] + rhs.costs[3]};
}
Cost operator-(const Cost &lhs, const Cost &rhs)
{
    return Cost{lhs.costs[0] - rhs.costs[0], lhs.costs[1] - rhs.costs[1], lhs.costs[2] - rhs.costs[2], lhs.costs[3] - rhs.costs[3]};
}
ostream& operator<<(ostream& os, const Cost& c) 
{
    out(' ', '{' , c.costs[0] , c.costs[1], c.costs[2], c.costs[3], '}');
    return os;
}

struct Bp
{
    int index;
    Cost robots[4];
};
ostream& operator<<(ostream& os, const Bp& bp) 
{
    out(' ', '[', bp.index, bp.robots[0], bp.robots[1], bp.robots[2], bp.robots[3], ']');
    return os;
}

int minutes = 24;

struct Sit
{
    const Bp* blueprint;
    int minute{0};
    int estimate{0};
    Cost bank{0,0,0,0};
    Cost income{1,0,0,0};
};
ostream& operator<<(ostream& os, const Sit& s) 
{
    out(' ', s.blueprint->index, s.minute, s.estimate, s.bank.costs[3], s.bank, s.income);
    return os;
}
bool operator<(const Sit &lhs, const Sit &rhs)
{
    return lhs.estimate + lhs.bank.costs[3] < rhs.estimate + rhs.bank.costs[3];
}
void Estimate(Sit &s)
{
    s.estimate = 0;
    Cost income = s.income;
    Cost bank = s.bank;
    for (int i = s.minute; i < minutes; ++i)
    {
        s.estimate += income.costs[3];
        Cost delta{1,1,0,0};
        if (bank.costs[2] >= s.blueprint->robots[3].costs[2])
        {
            bank.costs[2] -= s.blueprint->robots[3].costs[2];
            delta.costs[3] += 1;
        }
        if (bank.costs[1] >= s.blueprint->robots[2].costs[1])
        {
            bank.costs[1] -= s.blueprint->robots[2].costs[1];
            delta.costs[2] += 1;
        }
        bank = bank + income;
        income = income + delta;
    }
}

const char * robot_names[4]{ "ore", "clay", "obsidian", "geode"};
sv Robot(int t)
{
    assert(t >= 0 && t < 4);
    return robot_names[t];
}

int Search(const Bp *b)
{
    priority_queue<Sit> sits;
    Sit s{b};
    Estimate(s);
    sits.push(s);
    while (!sits.empty())
    {
        s = sits.top(); sits.pop();
        if (s.minute == minutes)
        {
            outln(' ', "new best", s);
            return s.bank.costs[3];
        }

        // bump up the minute and mine the ore
        s.minute += 1;
        Cost bank = s.bank;
        s.bank = s.bank + s.income;

        // push do nothing
        Estimate(s);
        sits.push(s);

        // try buying each of the robots
        for (int i = 0; i < 4; ++i)
        {
            // See if we can afford the robot
            bool can_buy = true;
            for (int c = 0; can_buy && c < 4; ++c)
                if (bank.costs[c] < s.blueprint->robots[i].costs[c])
                {
                    can_buy = false;
                }

            // buy it and put that state on the queue
            if (can_buy)
            {
                Sit n{s};
                for (int c = 0; c < 4; ++c)
                    n.bank.costs[c] -= n.blueprint->robots[i].costs[c];
                n.income.costs[i] += 1;
                Estimate(n);
                sits.push(n);
            }
        }
    }
    assert(false);
    return -1;
}

void Day19(deque<sv> &file_lines)
{
    int total = 0;
    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        vector<sv> tokens = Parse(line, ' ');

        Bp b;
        b.index = atoi(tokens[0]);
        b.robots[0] = Cost{atoi(tokens[1]), 0, 0};
        b.robots[1] = Cost{atoi(tokens[2]), 0, 0};
        b.robots[2] = Cost{atoi(tokens[3]), atoi(tokens[4]), 0};
        b.robots[3] = Cost{atoi(tokens[5]), 0, atoi(tokens[6])};

        outln(' ', "blueprint", b);
        int geodes = Search(&b);
        int value = geodes * b.index;
        total += value;
        outln(' ', "result", b.index, geodes, value, total);
    }
    outln(' ', "total", total);
}

void Day19b(deque<sv> &file_lines)
{
    minutes = 32;
    int total = 1;
    int count = 0;
    while (!file_lines.empty())
    {
        ++count;
        if (count > 3)
            break;

        sv line = file_lines.front(); file_lines.pop_front();
        vector<sv> tokens = Parse(line, ' ');

        Bp b;
        b.index = atoi(tokens[0]);
        b.robots[0] = Cost{atoi(tokens[1]), 0, 0};
        b.robots[1] = Cost{atoi(tokens[2]), 0, 0};
        b.robots[2] = Cost{atoi(tokens[3]), atoi(tokens[4]), 0};
        b.robots[3] = Cost{atoi(tokens[5]), 0, atoi(tokens[6])};

        outln(' ', "blueprint", b);
        int geodes = Search(&b);
        total *= geodes;
        outln(' ', "result", b.index, geodes,  total);
    }
    outln(' ', "total", total);

}
int main()
{
    unbuffer(cout);

    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    StripCharacters(file_contents, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ:,.");
    deque<sv> file_lines = ParseLines(file_contents);

    Day19(file_lines);
    return 0;
}
