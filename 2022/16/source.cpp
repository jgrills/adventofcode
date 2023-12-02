#include "helper.h"

#include <assert.h>
#include <algorithm>
#include <charconv>
#include <cstring>
#include <deque>
#include <fstream>
#include <map>
#include <mutex>
#include <queue>
#include <set>
#include <string>
#include <string_view>
#include <thread>
#include <vector>

#define DAY            16
#define USE_LONG_INPUT 1
#define VERBOSE        2

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

struct Valve
{
    string name;
    int flow;
    int opened;
    vector<string> dests;
    map<string,int> dist;
};

ostream& operator<<(ostream& os, const Valve& v) 
{
    os << v.name << ' ' << v.flow << " :";
    for (auto& d : v.dests)
    {
        os << ' ' << d;
    }
    return os;
}

map<string,Valve> valves;
struct Path
{
    string at;
    set<string> seen;
};
bool operator<(const Path& lhs, const Path& rhs)
{
    return lhs.seen.size() > rhs.seen.size();
}
typedef priority_queue<Path> PathQueue;

int PathFind(const string&from, const string &to)
{
    priority_queue<Path> paths;
    Path path;
    path.at = from;
    path.seen.insert(from);
    paths.push(path);
    set<string> shortest;

    for (;;)
    {
        assert(!paths.empty());
        Path p = paths.top();
        paths.pop();
        if (p.at == to) return p.seen.size() - 1;
        if (shortest.count(p.at) != 0) continue;
        shortest.insert(p.at);
        Valve &v = valves[string(p.at)];

        for (const string& d : v.dests)
        {
            if (p.seen.count(d) == 0 && shortest.count(d) == 0)
            {
                Path pd = p;
                pd.at = d;
                pd.seen.insert(d);
                paths.push(pd);
            }
        }
    }
    return 0;
}

struct Sit
{
    string at[2];
    int m[2];
 
    int volume;
    int estimate;
    set<string> remaining;
};
bool operator<(const Sit& lhs, const Sit& rhs)
{
    return lhs.estimate < rhs.estimate;
}
ostream& operator<<(ostream& os, const Sit& s) 
{
    os << s.m[0] << ' ' << s.at[0] << ' ' << s.m[1] << ' ' << s.at[1] << ' ' << s.volume << " [" << s.estimate << "] :";
    for (const auto& r : s.remaining)
    {
        os << ' ' << r;
    }
    return os;
}

int minutes;
mutex mut;
priority_queue<Sit> sits;
Sit best;


void ProcessElement()
{
    for (;;)
    {
        Sit s;
        {
            lock_guard<mutex> lock(mut);
            if (sits.empty()) break;

            s = sits.top();
            sits.pop();

            bool win = s.volume > best.volume;
            if (win)
            {
                best = s;
                outln(' ', sits.size(), "=size", s.m, "=m", s.at, s.volume, s.remaining.size(), '[', s.estimate, ']', win ? "WINNER" : "");
            }

            if (s.estimate < best.volume)
            {
                //outln(' ', "bad? done?");
                //outln(' ', "s", s);
                //outln(' ', "best", best);
                continue;
            }
        }

        Valve&v = valves[s.at[0]];
        for (const string& d : s.remaining)
        {
            int p = v.dist[d];
            assert(p != 0);
            Sit sd = s;
            sd.at[0] = d;
            sd.remaining.erase(d);
            sd.m[0] += p + 1;
            int left = (minutes - sd.m[0]);
            sd.volume += valves[d].flow * left;
            sd.estimate = sd.volume;
            for (const string& sdd : sd.remaining)
            {
                sd.estimate += left * valves[sdd].flow;
            }

            // VVV outln(' ', "  -> ", d, ":", sd);
            {
                lock_guard<mutex> lock(mut);
                sits.push(sd);
            }
        }
    }
}

void Day16()
{
    minutes = 30;
    std::ifstream input_file(input_filename);
    int total = 0;

    set<string> targets;
    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        sv line = line_string;

        vector<sv> tokens = Parse(line, ' ');
        Valve v;
        v.name = string(tokens[0]);
        v.flow = atoi(tokens[1]);
        for (int i = 2; i < tokens.size(); ++i)
            v.dests.push_back(string(tokens[i]));
        valves[v.name] = v;
        if (v.flow) targets.insert(v.name);
        outln(' ', v, v.flow);
    }

    Sit s;
    s.m[0] = 0;
    s.volume = 0;
    s.estimate = 0;
    s.at[0] = "AA";
    s.remaining = targets;

    vector<string> tlist;
    for (auto t : targets)
    {
        auto& v = valves[t];
        tlist.push_back(v.name);
        outln(' ', "target", v.name, v.flow);
        s.estimate += minutes * v.flow;
    }

    sits.push(s);

    tlist.push_back("AA");
    for (int from = 0; from < tlist.size()-1; ++from)
        for (int to = from+1; to < tlist.size(); ++to)
        {
            string& fs = tlist[from];
            string& ts = tlist[to];
            int p = PathFind(fs, ts);
            outln(' ', "path", fs, ts, p);
            valves[fs].dist[ts] = p;
            valves[ts].dist[fs] = p;
        }

    thread* thr[30];
    for (int i = 0; i < 30; ++i) thr[i] = new thread(ProcessElement);
    for (int i = 0; i < 30; ++i) thr[i]->join();

    outln(' ', "best", best);
    outln(' ', "Day", day);
}

void ProcessElementB()
{
    for (;;)
    {
        Sit s;
        {
            lock_guard<mutex> lock(mut);
            if (sits.empty()) break;

            s = sits.top();
            sits.pop();

            bool win = s.volume > best.volume;
            if (win)
            {
                best = s;
                outln(' ', sits.size(), "=size", s.volume, s.remaining.size(), '[', s.estimate, ']', win ? "WINNER" : "");
            }

            if (s.estimate < best.volume)
            {
                //outln(' ', "bad? done?");
                //outln(' ', "s", s);
                //outln(' ', "best", best);
                continue;
            }
        }

        int i = (s.m[0] <= s.m[1]) ? 0 : 1;
        Valve&v = valves[s.at[i]];
        for (const string& d : s.remaining)
        {
            int p = v.dist[d];
            if (p == 0)
            {
                outln(' ', p, s.at[i], d);
                assert(false);
            }
            assert(p != 0);
            Sit sd = s;
            sd.at[i] = d;
            sd.remaining.erase(d);
            sd.m[i] += p + 1;
            int left = (minutes - sd.m[i]);
            sd.volume += valves[d].flow * left;
            sd.estimate = sd.volume;
            for (const string& sdd : sd.remaining)
            {
                sd.estimate += left * valves[sdd].flow;
            }

            // VVV outln(' ', "  -> ", d, ":", sd);
            {
                lock_guard<mutex> lock(mut);
                sits.push(sd);
            }
        }
    }
}

void Day16b()
{
    minutes = 26;

    std::ifstream input_file(input_filename);
    int total = 0;

    set<string> targets;
    for (;;)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        sv line = line_string;

        vector<sv> tokens = Parse(line, ' ');
        Valve v;
        v.name = string(tokens[0]);
        v.flow = atoi(tokens[1]);
        for (int i = 2; i < tokens.size(); ++i)
            v.dests.push_back(string(tokens[i]));
        valves[v.name] = v;
        if (v.flow) targets.insert(v.name);
        outln(' ', v, v.flow);
    }

    Sit s;
    s.m[0] = 0;
    s.m[1] = 0;
    s.volume = 0;
    s.estimate = 0;
    s.at[0] = "AA";
    s.at[1] = "AA";
    s.remaining = targets;

    vector<string> tlist;
    for (auto t : targets)
    {
        auto& v = valves[t];
        tlist.push_back(v.name);
        outln(' ', "target", v.name, v.flow);
        s.estimate += minutes * v.flow;
    }

    sits.push(s);

    tlist.push_back("AA");
    for (int from = 0; from < tlist.size()-1; ++from)
        for (int to = from+1; to < tlist.size(); ++to)
        {
            string& fs = tlist[from];
            string& ts = tlist[to];
            int p = PathFind(fs, ts);
            outln(' ', "path", fs, ts, p);
            valves[fs].dist[ts] = p;
            valves[ts].dist[fs] = p;
        }

#if 0
    thread* thr[30];
    for (int i = 0; i < 30; ++i) thr[i] = new thread(ProcessElementB);
    for (int i = 0; i < 30; ++i) thr[i]->join();
#else
    ProcessElementB();
#endif


    outln(' ', "best", best);
    outln(' ', "Day", day);
}

int main()
{
    Day16b();
    return 0;
}
