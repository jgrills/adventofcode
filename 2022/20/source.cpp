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

struct Node
{
    int64_t value;
};

void Day19(deque<sv> &file_lines)
{
    vector<Node*> orig;
    vector<Node*> nodes;
    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        Node *n = new Node{atoi(line)};
        nodes.push_back(n);
        orig.push_back(n);
        outln(' ', "val", nodes.back()->value);
    }

    int sz = nodes.size();
    for (Node* mover : orig)
    {
        int i = -1;
        for (int q = 0; q < sz; ++q)
        {
            if (nodes[q] == mover)
            {
                i = q;
                break;
            }
        }
        assert(i >= 0 && i <= sz);
        outln(' ', "mover", mover, mover->value, '[', i, ']');
        if (mover->value > 0)
        {
            for (int b = 0; b < mover->value; ++b)
            {
                assert(i >= 0 && i <= sz);
                int j = (i + 1) % sz;
                assert(j >= 0 && j <= sz);
                swap(nodes[i], nodes[j]);
                i = j;
            }
        }
        else if (mover->value < 0)
        {
            for (int b = 0; b < -mover->value; ++b)
            {
                assert(i >= 0 && i <= sz);
                int j = i - 1;
                if (j < 0)
                    j = sz - 1;
                assert(j >= 0 && j <= sz);
                swap(nodes[i], nodes[j]);
                i = j;
            }
        }
    }

    int i = -1;
    for (int q = 0; q < sz; ++q)
    {
        if (nodes[q]->value == 0)
        {
            i = q;
            break;
        }
    }

    int result = 0;
    for (int j = 1; j  <= 3000; ++j)
    {
        i = (i + 1) % sz;
        if ((j % 1000) == 0)
        {
            result += nodes[i]->value;
            outln(" ", j, i, nodes[i]->value);
        }
    }
    outln(" ", "done", result);
}

int Find(const vector<Node*>& nodes, Node* n)
{
    int sz = nodes.size();
    for (int i = 0; i < sz; ++i)
        if (nodes[i] == n)
            return i;
    assert(false);
    return -1;
}

void Print(sv text, const vector<Node*>& nodes, Node* start=nullptr)
{
    int base = start ? Find(nodes, start) : 0;
    out("", text, ":");
    int sz = nodes.size();
    for (int i = 0; i < sz; ++i)
        out("", ' ', nodes[(base+i)%sz]->value);
    outln("", "");
}

void Day19b(deque<sv> &file_lines)
{
    int64_t key = 811589153;
    // int64_t key = 1;
    vector<Node*> orig;
    vector<Node*> nodes;
    Node* zero = nullptr;
    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        Node *n = new Node{atoi(line) * key};
        nodes.push_back(n);
        orig.push_back(n);
        if (n->value == 0)
        {
            assert(!zero);
            zero = n;
        }
        outln(' ', "val", nodes.back()->value);
    }
    assert(zero);

    // Print("initial", nodes);

    const int64_t sz = nodes.size();
    for (int mixes = 1; mixes <= 10; ++mixes)
    {
        for (Node* mover : orig)
        {
            int i = Find(nodes, mover);
            assert(i >= 0 && i < sz);

            if (mover->value > 0)
            {
                const int64_t mov = mover->value % (sz - 1);
                for (int b = 0; b < mov; ++b)
                {
                    assert(i >= 0 && i <= sz);
                    int j = (i + 1) % sz;
                    assert(j >= 0 && j <= sz);
                    swap(nodes[i], nodes[j]);
                    i = j;
                }
            }
            else if (mover->value < 0)
            {
                const int64_t mov = (-mover->value) % (sz - 1);
                for (int b = 0; b < mov; ++b)
                {
                    assert(i >= 0 && i <= sz);
                    int j = i - 1;
                    if (j < 0)
                        j = sz - 1;
                    assert(j >= 0 && j <= sz);
                    swap(nodes[i], nodes[j]);
                    i = j;
                }
            }
        }

        // Print("mix", nodes, zero);
    }

    int i = -1;
    for (int q = 0; q < sz; ++q)
    {
        if (nodes[q]->value == 0)
        {
            i = q;
            break;
        }
    }

    int64_t result = 0;
    int i0 = (i + 1000) % sz;
    int i1 = (i + 2000) % sz;
    int i2 = (i + 3000) % sz;
    outln(" ", i, nodes[i]->value, result);
    result += nodes[i0]->value;
    outln(" ", i0, nodes[i0]->value, result);
    result += nodes[i1]->value;
    outln(" ", i1, nodes[i1]->value, result);
    result += nodes[i2]->value;
    outln(" ", i2, nodes[i2]->value, result, "final");
}

int main()
{
    unbuffer(cout);
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    deque<sv> file_lines = ParseLines(file_contents);

    Day19b(file_lines);
    return 0;
}

// -1949981831
