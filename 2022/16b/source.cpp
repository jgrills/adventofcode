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

constexpr int max_valves = 64;

struct Valve
{
    sv name;
    int index;
    int flow;
    int opened;
    vector<sv> dests;
    int dist[max_valves]{0};
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
vector<Valve> valves;
Valve bad_valve;
Valve& Valves(sv name)
{
    for (Valve& v : valves)
    {
        if (v.name == name)
            return v;
    }
    assert(false);
    return bad_valve;
}
Valve& Valves(int index)
{
    assert(index >= 0 && index < valves.size());
    return valves[index];
}

struct ValveSet
{
    uint64_t flags{0};

    static void validate(int index) { assert(index >= 0 && index < 64); }
    static uint64_t bit(int index) { validate(index); return static_cast<uint64_t>(1) << index;}

    void erase(int index) { validate(index); flags &= ~bit(index); }
    void erase(sv name) { erase(Valves(name).index); }
    void insert(int index)  { validate(index); flags |= bit(index); }
    void insert(sv name)  { insert(Valves(name).index); }
    bool contains(int index) const { validate(index); return (flags & bit(index)) != 0; }
    bool contains(sv name) const { return contains(Valves(name).index); }
    int size() const { int result = 0; for (int i = 0; i < max_valves; ++i) if ((flags & bit(i)) != 0) ++result; return result; }
};

struct Path
{
    int at;
    ValveSet seen;
};
bool operator<(const Path& lhs, const Path& rhs)
{
    return lhs.seen.size() > rhs.seen.size();
}
typedef priority_queue<Path> PathQueue;

int PathFind(Valve& from, Valve& to)
{
    priority_queue<Path> paths;
    Path path;
    path.at = from.index;
    path.seen.insert(from.index);
    paths.push(path);
    ValveSet shortest;

    for (;;)
    {
        assert(!paths.empty());
        Path p = paths.top();
        paths.pop();

        // When we pull the destination from the priority queue we are done
        if (p.at == to.index)
            return p.seen.size() - 1;

        // Don't expand this further if we found a better way here
        if (shortest.contains(p.at))
            continue;

        // Mark this node as having its shortes path found
        shortest.insert(p.at);
        Valve &v = Valves(p.at);

        // Search the local edges
        for (sv d : v.dests)
        {
            Valve &vd = Valves(d);
            int vi = vd.index;
            // Don't loop back on ourselves, don't go somewhere we already have a good path fo
            if (!p.seen.contains(vi) && !shortest.contains(vi))
            {
                Path pd = p;
                pd.at = vi;
                pd.seen.insert(vi);
                paths.push(pd);
            }
        }
    }
    return 0;
}

struct Sit
{
    struct Agent
    {
        int at{0};
        int minutes{0};
    };
    Agent agent[2];
 
    int volume{0};
    int estimate{0};
    int remainingCount{0};
    int8_t remaining[16]{0};
};
bool operator<(const Sit& lhs, const Sit& rhs)
{
    return lhs.estimate < rhs.estimate;
}
ostream& operator<<(ostream& os, const Sit& s)
{
    os << s.agent[0].minutes << ' ' << s.agent[0].at << ' ' << s.agent[1].minutes << ' ' << s.agent[1].at << ' ' << s.volume << " [" << s.estimate << "] :";
    for (int i = 0; i < s.remainingCount; ++i)
    {
        int r = s.remaining[i];
        os << ' ' << Valves(r).name;
    }
    return os;
}

int minutes;
mutex mut;
priority_queue<Sit> sits;
Sit best;

void ProcessElementB()
{
    for (;;)
    {
        Sit s;
        {
            //lock_guard<mutex> lock(mut);
            if (sits.empty()) break;

            s = sits.top();
            sits.pop();

            bool win = s.volume > best.volume;
            if (win)
            {
                best = s;
            }

            if (s.estimate < best.volume)
            {
                //outln(' ', "bad? done?");
                //outln(' ', "s", s);
                //outln(' ', "best", best);
                continue;
            }
        }

        if (best.volume == s.volume)
            outln(' ', sits.size(), "=size", s.volume, s.remainingCount, '[', s.estimate, ']', "WINNER");

        int agent_index = (s.agent[0].minutes <= s.agent[1].minutes) ? 0 : 1;
        Sit::Agent &agent = s.agent[agent_index];
        Valve &v = valves[agent.at];
        for (int ri = 0; ri < s.remainingCount; ++ri)
        {
            int dest = s.remaining[ri];
            int p = v.dist[dest];
            assert(p != 0);
            Sit sd = s;
            sd.agent[agent_index].at = dest;
            sd.agent[agent_index].minutes += p + 1;
            sd.remaining[ri] = sd.remaining[--sd.remainingCount];
            int left = minutes - sd.agent[agent_index].minutes;
            sd.volume += valves[dest].flow * left;
            sd.estimate = sd.volume;
            for (int sdri = 0; sdri < sd.remainingCount; ++sdri)
            {
                int sddest = sd.remaining[sdri];
                sd.estimate += left * Valves(sddest).flow;
            }

            // VVV outln(' ', "  -> ", d, ":", sd);
            {
                //lock_guard<mutex> lock(mut);
                sits.push(sd);
            }
        }
    }
}

void Day16b(deque<sv> &file_lines)
{
    minutes = 26;

    // ValveSet initial_valve_set;
    vector<sv> targets;
    targets.reserve(file_lines.size());
    int max_flow = 0;
    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();;
        vector<sv> tokens = Parse(line, ' ');
        Valve v;
        v.name = tokens[0];
        v.index = valves.size();
        v.flow = atoi(tokens[1]);
        for (int i = 2; i < tokens.size(); ++i)
            v.dests.push_back(tokens[i]);
        valves.push_back(v);
        if (v.flow)
        {
            max_flow += v.flow;
            targets.push_back(v.name);
            // initial_valve_set.insert(v.index);
        }
        outln(' ', v, v.flow);
    }

    ValveSet flowValves;

    targets.push_back("AA");
    for (int from = 0; from < targets.size()-1; ++from)
        for (int to = from+1; to < targets.size(); ++to)
        {
            sv fs = targets[from];
            sv ts = targets[to];
            Valve& fv = Valves(fs);
            Valve& tv = Valves(ts);
            int p = PathFind(fv, tv);
            outln(' ', "path", fs, ts, p);
            fv.dist[tv.index] = p;
            tv.dist[fv.index] = p;
        }
    targets.pop_back();

    Sit s;
    s.agent[0].minutes = 0;
    s.agent[0].at = Valves("AA").index;
    s.agent[1].minutes = 0;
    s.agent[1].at = s.agent[0].at;
    for (sv t : targets)
    {
        s.remaining[s.remainingCount] = Valves(t).index;
        ++s.remainingCount;
    }
    sits.push(s);

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
    // unbuffer(cout);

    outln(' ', "Starting");

    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    // SpaceCharacters(file_contents, "=");
    // StripCharacters(file_contents, ":,");
    deque<sv> file_lines = ParseLines(file_contents);

    Day16b(file_lines);
    return 0;
}
