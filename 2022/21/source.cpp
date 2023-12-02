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

#define DAY            20
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

struct Math
{
    sv result;
    sv a0;
    sv op;
    sv a1;
    int64_t r;
    int64_t done{2};
    int64_t human{false};
    Math *a0n{nullptr};
    Math *a1n{nullptr};
 };
map<sv, Math*> nodes;

void Print(Math& e)
{
    if (e.op == "")
    {
        if (e.result == "humn")
            out("", e.result);
        else
            out("", e.r);
    }
    else
    {
        if(e.human)
            out("", '[');
        else
            out("", '(');

      Print(*e.a0n);
      out("", ' ', e.op, ' ');
      Print(*e.a1n);
        if(e.human)
            out("", ']');
        else
            out("", ')');
    }
}

void Tag(Math* n)
{
    for (bool go = true; go; )
    {
        go = false;
        for (auto [k,v] : nodes)
        {
            if (v->op != "")
            {
                assert(!(v->a0n->human && v->a1n->human));
                if (!v->human && (v->a0n->human || v->a1n->human))
                {
                    outln(' ', "human", v->result);
                    v->human = true;
                    go = true;
                }
            }
        }
    }
}


int64_t Eval(Math *e)
{
    if (e->op == "") return e->r;
    Eval(e->a0n);
    Eval(e->a1n);
    if (e->op == "+") e->r = e->a0n->r + e->a1n->r;
    else if (e->op == "-") e->r = e->a0n->r - e->a1n->r;
    else if (e->op == "*") e->r = e->a0n->r * e->a1n->r;
    else if (e->op == "/") e->r = e->a0n->r / e->a1n->r;
    else assert(false);
    return e->r;
}

void Day20(deque<sv> &file_lines)
{

    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        // outln(' ', line);
        vector<sv> tokens = Parse(line, ' ');
        if (tokens.size() == 2)
        {
            nodes[tokens[0]] = new Math{tokens[0], "", "", "", atoi(tokens[1])};
        }
        else
        {
            nodes[tokens[0]] = new Math{tokens[0], tokens[1], tokens[2], tokens[3]};
        }
    }

    for (auto [k,v] : nodes)
    {
        if (v->op != "")
        {
            v->a0n = nodes[v->a0];
            v->a1n = nodes[v->a1];
        }
    }

    Math *h = nodes["humn"];
    assert(h);
    h->human = true;
    Tag(h);

    Math *root = nodes["root"];


    Math *r= nodes[root->a0];
    Math *l= nodes[root->a1];
    Print(*l);
    outln("", "");
    outln("", "");
    outln("", "");
    Print(*r);
    outln("", "");
    outln("", "");
    outln("", "");
    swap(l, r);
    assert(l->human);
    assert(!r->human);
    while (l != h)
    {
#if 0

    (a op b) = (c op d)

    *   b = (c op d) / a
    /   a / b = (c op d)
        a = (c op d) * b
        b = a / (c op d)
#endif

        assert(l->op != "");
        assert(r->op != "");
        assert(l->human && !r->human);
        assert(l->a0n->human || l->a1n->human);
        assert(!(l->a0n->human && l->a1n->human));
        sv op = l->op;
        if (l->a0n->human)
        {
            //    +   a = (c op d) - b
            //    -   a = (c op d) + b
            //    *   a = (c op d) / b
            //    /   a = (c op d) * b
            Math* right = new Math;
            right->a0n = r;
            right->a1n = l->a1n;
            if (op == "+") right->op = "-";
            else if (op == "-") right->op = "+";
            else if (op == "*") right->op = "/";
            else if (op == "/") right->op = "*";
            else assert(false);
            l = l->a0n;
            r = right;
        }
        else
            if (l->a1n->human)
            {
                //    +   b = (c op d) - a
                //    *   b = (c op d) / b
                //    -   b = a - (c op d)
                //    /   b = a / (c op d)
                Math* right = new Math;
                right->a0n = r;
                right->a1n = l->a0n;
                l = l->a1n;
                r = right;
                if (op == "+") right->op = "-";
                else if (op == "*") right->op = "/";
                else
                {
                    if (op == "-") right->op = "-";
                    else if (op == "/") right->op = "/";
                    else assert(false);
                    swap(right->a0n, right->a1n);
                }
            }
            else
                assert(false);
        }

    outln("", "done");
    Print(*r);
    outln("", "");
    Eval(r);
    outln("", r->r);
    
#if 0

    int64_t min = -1000000000000;
    int64_t max = -min;
    for (;;)
    {
        Math* a0 = nullptr;
        Math* a1 = nullptr;
        int mid = (min + max) / 2;
        for (auto [k,v] : nodes)
        {
            if (v->result == "humn") v->r = mid;
            if (v->op == "")
                v->done = mid;
            else
                v->done = mid - 1;
        }

        while (root->done != mid)
        {
            for (auto [k,v] : nodes)
            {
                Math&e = *v;
                a0 = nodes[e.a0];
                a1 = nodes[e.a1];
                if (mid != e.done && a0->done == mid && a1->done == mid)
                {
                    if (e.op == "+") e.r = a0->r + a1->r;
                    else if (e.op == "-") e.r = a0->r - a1->r;
                    else if (e.op == "*") e.r = a0->r * a1->r;
                    else if (e.op == "/") e.r = a0->r / a1->r;
                    else assert(false);
                    e.done = mid;
                    if (e.result == "root")
                        break;
                }
            }
        }
        outln(' ', "root loop", min, mid, max, a0, a1);
        if (a0 < a1)
            min = mid+1;
        else
            max = mid-1;
    }
#endif
}

int main()
{
    unbuffer(cout);
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    StripCharacters(file_contents, ":");
    deque<sv> file_lines = ParseLines(file_contents);

    Day20(file_lines);
    return 0;
}

//89002938276389
//32853424641061