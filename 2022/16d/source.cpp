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
vector<Valve*> targetptrs;
Valve bad_valve;
Valve& Targets(sv name)
{
    for (Valve* tp : targetptrs)
        if (tp->name == name)
            return *tp;
    assert(false);
    return bad_valve;
}
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
    int count{0};
    uint64_t flags{0};

    static void validate(int index) { assert(index >= 0 && index < 64); }
    static uint64_t bit(int index) { validate(index); return static_cast<uint64_t>(1) << index;}

    void erase(int index)
    {
        validate(index);
        uint64_t r = flags & ~bit(index);
        if (r != flags)
        {
            --count;
            flags = r;
        }
    }
    void erase(sv name) { erase(Valves(name).index); }
    void insert(int index)
    {
        validate(index);
        uint64_t r = flags | bit(index);
        if (r != flags)
        {
            ++count;
            flags = r;
        }
    }
    void insert(sv name)  { insert(Valves(name).index); }
    bool contains(int index) const { validate(index); return (flags & bit(index)) != 0; }
    bool contains(sv name) const { return contains(Valves(name).index); }
    int size() const { return count; }
};

struct Path
{
    sv at;
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
    path.at = from.name;
    path.seen.insert(from.index);
    paths.push(path);
    ValveSet shortest;

    for (;;)
    {
        assert(!paths.empty());
        Path p = paths.top(); paths.pop();

        // When we pull the destination from the priority queue we are done
        if (p.at == to.name)
            return p.seen.size() - 1;

        Valve &pv = Valves(p.at);

        // Don't expand this further if we found a better way here
        if (shortest.contains(pv.index))
            continue;

        // Mark this node as having its shortes path found
        shortest.insert(pv.index);

        // Search the local edges
        for (sv d : pv.dests)
        {
            Valve &vd = Valves(d);
            int vdi = vd.index;
            // Don't loop back on ourselves, don't go somewhere we already have a good path fo
            if (!p.seen.contains(vdi) && !shortest.contains(vdi))
            {
                Path pd = p;
                pd.at = d;
                pd.seen.insert(vdi);
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
        sv at;
        int minutes{0};
    };
    Agent agent[2];
 
    int volume{0};
    int estimate{0};
    int remainingCount{0};
    sv remaining[16];
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
        os << ' ' << s.remaining[i];
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
        if (sits.empty()) break;

        Sit s = sits.top(); sits.pop();
        bool win = s.volume > best.volume;
        if (win)
        {
            best = s;
            outln(' ', sits.size(), s.volume, '[', s.estimate, ']', "WINNER");
        }
        if (s.estimate < best.volume)
            continue;

        int agent_index = (s.agent[0].minutes <= s.agent[1].minutes) ? 0 : 1;
        Sit::Agent &agent = s.agent[agent_index];
        Valve &v = Valves(agent.at);
        for (int ri = 0; ri < s.remainingCount; ++ri)
        {
            sv dest_name = s.remaining[ri];
            int dest = Valves(dest_name).index;
            int p = v.dist[dest];
            assert(p != 0);
            Sit sd = s;
            sd.agent[agent_index].at = dest_name;
            sd.agent[agent_index].minutes += p + 1;
            sd.remaining[ri] = sd.remaining[--sd.remainingCount];
            int left = minutes - sd.agent[agent_index].minutes;
            sd.volume += valves[dest].flow * left;
            sd.estimate = sd.volume;
            for (int sdri = 0; sdri < sd.remainingCount; ++sdri)
                sd.estimate += left * Valves(sd.remaining[sdri]).flow;

            // VVV outln(' ', "  -> ", d, ":", sd);
            sits.push(sd);
        }
    }
}

void Day16b(deque<sv> &file_lines)
{
    minutes = 26;

    vector<sv> targets;
    targets.reserve(file_lines.size());
    targetptrs.reserve(file_lines.size());
    int max_flow = 0;
    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();;
        vector<sv> tokens = Parse(line, ' ');
        valves.emplace_back();

        Valve &v = valves.back();
        v.name = tokens[0];
        v.index = valves.size();
        v.flow = atoi(tokens[1]);
        for (int i = 2; i < tokens.size(); ++i)
            v.dests.push_back(tokens[i]);
        if (v.flow)
        {
            max_flow += v.flow;
            targets.push_back(v.name);
        }
        outln(' ', v, v.flow);
    }

    Sit s;
    s.agent[0].minutes = 0;
    s.agent[0].at = "AA";
    s.agent[1].minutes = 0;
    s.agent[1].at = s.agent[0].at;
    for (sv t : targets)
    {
        s.remaining[s.remainingCount] = t;
        ++s.remainingCount;
    }
    sits.push(s);

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

    ProcessElementB();

    outln(' ', "best", best);
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
