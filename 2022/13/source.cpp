#include "helper.h"

#include <algorithm>
#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <map>
#include <set>
#include <string>
#include <string_view>
#include <vector>

#define DAY            13
#define USE_LONG_INPUT 1
#define VERBOSE        0

constexpr int day = DAY;

using namespace std;
typedef string_view sv;

#if USE_LONG_INPUT
const char* input_filename = "long";
#else
const char* input_filename = "short";
#endif

int verbose = VERBOSE;
#define VV   if (verbose > 0)
#define VVV  if (verbose > 1)
#define VVVV if (verbose > 2)

struct Element;
typedef std::vector<Element*> List;
struct Element
{
    bool is_list;
    int value;
    List list;
};

List Parse(sv& line)
{
    List result;

    assert(!line.empty());
    assert(line[0] == '[');
    line.remove_prefix(1);

    for (;;)
    {
        if (line[0] == ']') break;

        Element *e = new Element;
        if (line[0] == '[')
        {
            e->is_list = true;
            e->list = Parse(line);
        }
        else if (isdigit(line[0]))
        {
            e->is_list = false;
            e->value = atoi2(line);
        }
        result.push_back(e);

        if (line[0] == ',') line.remove_prefix(1);
    }

    line.remove_prefix(1);
    return result;
}

ostream& operator<<(ostream& os, const List& list) 
{
    os << '[';
    int count = 0;
    for (Element* e : list)
    {
        if (e->is_list)
            os << e->list;
        else
            os << e->value;
        if (++count < list.size()) os << ',';
    }
    os << ']';
    return os;
}

int compare(const List& lhs, const List& rhs, string indent)
{
    for (int index = 0;; ++index)
    {
        if (index >= lhs.size() && index >= rhs.size())
        {
            VV outln(' ', indent, "equal, out on both");
            return 0;
        }
        if (index >= lhs.size())
        {
            VV outln(' ', indent, "less, out on lhs");
            return -1;
        }
        if (index >= rhs.size())
        {
            VV outln(' ', indent, "greater, out on rhs");
            return 1;
        }

        Element* l = lhs[index];
        Element* r = rhs[index];
        if (!l->is_list && !r->is_list)
        {
            int result = 0;
            if (l->value < r->value) result = -1;
            if (l->value > r->value) result = 1;
            VV outln(' ', indent, "compare values", l->value, r->value, result);
            if (result != 0) return result;
        }
        else
        {
            if (!l->is_list && l->list.empty())
            {
                Element *f = new Element;
                f->is_list = false;
                f->value = l->value;
                l->list.push_back(f);
            }

            if (!r->is_list && r->list.empty())
            {
                Element *f = new Element;
                f->is_list = false;
                f->value = r->value;
                r->list.push_back(f);
            }

            VV outln(' ', indent, "compare lists", left, "<=>", right, ":", l->list.size(), r->list.size());
            int result = compare(l->list, r->list, indent + "  ");
            VV outln(' ', indent, "  result", result);
            if (result != 0) return result;
        }
    }
    return 0;
}

void Day13()
{
    ifstream input_file(input_filename);
    int total = 0;
    for (int pair = 1;; ++pair)
    {
        string s1;
        if (!getline(input_file, s1)) break;
        string s2;
        if (!getline(input_file, s2)) assert(false);
        string s3;
        getline(input_file, s3);

        sv l1 = s1;
        sv l2 = s2;

        List list1 = Parse(l1);
        List list2 = Parse(l2);
        
        VV outln(' ', "l1", list1);
        VV outln(' ', "l2", list2);
        int cmp = compare(list1, list2, "  ");
        assert(cmp != 0);
        bool c = (cmp <= 0);
        if (c) outln("", pair);
        if (c) total += pair;
        outln(' ', "  final", pair, "=pair", total, "=total", cmp, c ? "right" : "wrong");
    }
    outln(' ', "Day", day);
}

void Day13b()
{
    ifstream input_file(input_filename);
    vector<List> packets;

    for (int pair = 1;; ++pair)
    {
        string s1;
        if (!getline(input_file, s1)) break;
        string s2;
        if (!getline(input_file, s2)) assert(false);
        string s3;
        getline(input_file, s3);

        sv l1 = s1;
        sv l2 = s2;

        List list1 = Parse(l1);
        List list2 = Parse(l2);
        
        VV outln(' ', "l1", list1);
        VV outln(' ', "l2", list2);

        packets.push_back(list1);
        packets.push_back(list2);
    }

    sv sv0("[[2]]");
    sv sv1("[[6]]");
    List a0 = Parse(sv0);
    List a1 = Parse(sv1);
    packets.push_back(a0);
    packets.push_back(a1);
    sort(packets.begin(), packets.end(), [](const List& lhs, const List& rhs) { return compare(lhs, rhs, "  ") < 0; });

    int prod = 1;
    int index = 0;
    for (const List& l : packets)
    {
        ++index;
        outln(' ', l);
        if (compare(l, a0, "  ") == 0)
        {
            prod *= index;
            outln (' ', "a0", index, prod, a0);
         }
        if (compare(l, a1, "  ") == 0)
        {
            prod *= index;
            outln (' ', "a1", index, prod, a1);
         }
    }
    outln(' ', "13b done", prod);
}

int main()
{
    Day13b();
    return 0;
}
