#include "helper.h"

#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <map>
#include <unistd.h>
#include <string>
#include <string_view>
#include <vector>

#define USE_LONG_INPUT 1
#define VERBOSE        1

using namespace std;
typedef string_view sv;

#define VV1 if constexpr (VERBOSE >= 1)
#define VV2 if constexpr (VERBOSE >= 2)
#define VV3 if constexpr (VERBOSE >= 3)

string GetInputText()
{
#if USE_LONG_INPUT
    return string(">>><<>>><<<>>>><<>>><<<<>>>><<<>>>><<<>>><<<<>><<><>><<<>>>><<<<>><<<>><<<<>>><<<<>>><>><<>><<<>>>><<<<>>><<<<>>>><<<><>>>><><<>>>><<><>>>><<<>><<<>>><<<<>><<><<<<>><<><>><<<>><>>>><<<>>>><<>>>><<><<<>><<>>>><>><<<<>>>><>>><<<>><<<<>>>><<>><<<<><<><<<>>><<<<><<>>>><<><<<>><<<<><<<><<>>>><>><<<<>><>>>><<><><>>><>>>><<<><<>>>><<<><<<<>><<<>>><>>><<>>>><>>><<>><<<<>>>><<<>><<>>><<<<><<<>><>>>><<<<>><<<<><>><<<<>>><>>><<<<>>><<<<>>><<<<>>><<<>><<<>>><<<>><>>><<<<><>><<<<>>>><<<<>><<<>>>><<<<>><<<<>><<<<>>>><<<<>><<<>><<<>><<<>><<>>><<>>>><<<>>>><>>>><>><<<>>>><>><<<><<<<><<<>>>><<><<>>><>><>>>><<<>>><<<<>>><>>>><>>><<<><<>>><<<<><<>>>><<<><>><>>>><<<>><<<>><<<><<<<>><>><<<<>>><>>>><<>>><<>><>>>><<<<><>>><>><><>>>><<<><<<>>>><><<<<><<<><<<<>>><<<>><<<<>>><>>><<<<>>><<>><<<>>><<>><<<<><<<<><<<<>><<<>>><<>>>><>><<<<>>><><<>>><<<>>>><>>>><>>><<<<>>><<<<>><<<<><><>><<<<>><<>>>><>>>><<><<>>>><><<<>>><<>>>><<<>>><<<<>><<<<>>>><<<<>><<<><<>>>><<<<>>>><<<>>><<><<<>>><><<<>>>><<<<>>><>>><>>>><<<<>><<<>><<><<<>><>><>><<<><<<>>>><>><><>>>><<>>><<>><>><<<>><><<<<>>>><<>>><<>>><<>><<<>>><<<<>><<<<>>>><>>><<<<>><><<<>>>><<>><>>>><>>>><>>>><>>>><>>><<<<>><><<<<>>>><<<<>><><<<<><>><<>>><>>><<<>><><<>>>><<<>>><<<<>>>><<<>><<<<>>>><<<>>>><<<<>>>><<><<<<>>>><<<><<<>><<<<><<<>>><><<<>><<>><<<>>><<<><<<>>>><<<<>><<<<>><<>><<<<>><>><>>><<<<><>>>><<>><<<><<>><<<<><<><<<<>><>>><>><<<>>><<<>><<<<>>>><<<<>>>><<<>>><<<<>><>>><<>><<<><<<<>>>><<>>>><<<<>><<<<>><<<>>>><<>>>><><><<>><><<<><<<>>><>>><<<<>>>><<<>><<<<>>><<<<>><<<>>><<<<>>>><<><>><<<<><<<<>><<<><<><><><<<<>><<<<>>><<<>><>>>><>>><<<<>>><<<<><<>>><<<<><<>><<<><<<<>><<>>>><>><>><<<>>>><<<<>>><<<><<<>>>><<<><<<>>>><<<>><>>><<>>><<<><<<<>>>><>>>><<<<><<>><<>>><<><>>>><>>><><<>>>><>>><><>><<<>>>><<<><>><<>>>><<<<>>><<><<>>>><>><<>>><<<<><<>>>><<><>>><<<><<><<<<>><<<<>>>><<>><>><<>><>>>><<<>>>><<<<>><><<>>>><<<><<>>><<<>>>><<<>>><<>>>><<<<>>><>>><<<><<>>><><>>><<<>>>><<<<>>><<>>>><<>>><<<<>>>><<<>><><<>><<<>>><<>><<<>>>><<>><<>>><<>>>><<<>>>><<<>>><>>>><<><<<<><<<>>><<<><<<>>><<>>>><>><<<>><<>>>><><>>>><>>><<<>><<<>>>><<>>><<>><><<<>><<<<>><>>><>>>><<<<>><<<>>><<>>><<>>><<<<>>>><<>>>><>>><<<<>>><<<>><<<><<<>><<<<><<>>>><<>><>>><<<>>>><<><<<<>>>><<<>>>><<<><<<>><<<<><<<<><<>>>><<<<>>>><<<><<<<>>><<<<>><<><<<<>>>><>>><<<<>><<>><<<<>>>><<<>><<>>><<<><<<<>>><<>>>><<>>>><<<<>><>>>><<<<><>><>><<>>>><>>>><<<<>>><<<<>>><<<>>>><<<>>><<>>><<<<><><<>>><<><<<<>>><<>><<<>><<>><>><<<><<><<>>><<>>><<>>>><<>><<<><<<<>>><<<>>><<><>>><<<<>><<<<>>><<<>><>><<<<>><<<<>><<<<><>><>>><><>><<<>><>><>><<>><>>>><<<<>>><>><<>><<<<><<<>>>><<<<>>>><<<<><>>>><<<<><<<<>>>><>>>><><>>>><>>><<<<><<<<>>>><<<<><<<<>><<<<><<<<><<<>><<<>>><<>>>><<<<>>><><<>>>><<<><>>>><<<>><<<>>><<<>>>><>>><<<<><>>><>>>><<>>><>><<<<>><>><>><>>>><<<<><<>><<<>>><<<<>><<<<><<<<>><<<>><<>>>><<<>>><<><<><<<<>>>><<<>>>><<<>>><>><<<<>>>><<<<><<<<>>><<><<<<>>>><<<>><>>><>>>><<<>>>><<>>><>>><<<>><<<<>>><<><<<><<<<><<<<>><<<<>>><<>>>><<>><<>><<<<>>>><<<>>><<<<>>>><><<><<<>>>><><<>>><><><<<><<<<><><>>>><<<>>>><<><><<>><<>>><<>>><<<<>>>><<>><<<<>>><<<>>>><<<<>>><<>>>><<<>>>><>>><>>><<<><>><<<<>><<<>>>><<<<>><><<>>>><<<><<>><>><<<<><<><<<>>>><<><<>><<>><<<<>>>><<<>><<>>>><<>>>><<>>><>>><<<<>><<<<>>>><<<<>>>><><<<<>>>><<<>>>><><>>>><<<>>><>>><>>>><>>><<>><<<>><>>><>>><<<<>>><<>>>><<<>><<>>><<><<<<>>><<<><<<<>>>><>><<<>><<<>><>>><<<>>><<<<>>><<>><<<<>>><>>>><><<<>><<<<>>><<>>><<<<>>>><<<>><<<<>>><>><<<<><<<>>><<><<<<>>><<<>>><>>><>>><<>>>><<<<><<<>>>><<<>>>><>>>><<>>>><<<<>>>><<<>>><<<<>><<<<>><>>><<<<>>>><>>><<<>>>><><<<<>>>><<<>><>><<<<><>><>><<<<>>>><>>><<<<>><>><<><>>>><<<>>>><<<><>><<<<>><<><><<<>><<>>><<<>>>><>>><<>>><<<<><>>><<><>>>><<><<>><<<><<>><>>>><>>>><<><<<<>><<>>><<<<>><<<>>><<<<>>><>>>><>>><>>><<<<>>>><<<<>>>><<><>><<<<>>>><<>>><<<<><<>><<<<>>>><>><>><<<>>><>>><<>>>><<<<>><<><<<>>><><<<<><<<>><<<>>>><<<>>><>><<<<><<<<><<<<><>>>><<<<>><<>>><<>>>><<>>><><<<<>>><<>><<<>><<><<>>>><<<<>>>><<<<>>><<><<><<>>>><<>>><>>>><<<<>>>><<<>><<><<<>>><<<<>>><><<<>>>><<<><>>><<<>>>><<<<>>>><<>>><>>><<<<>>><<>><<<>>><>>>><><<>><>>>><<<<>>><>>>><<<><>>><<<<>>><><<<>>>><<<<>><<<<><><<<<>><<<>>>><>>><<>><<><<<<>><>>>><>>><<>>>><<<<>><<<<><<>>>><<>>>><>>>><<>><<<<>>><<<<>>><<><<>>>><>>><<<<>>>><<<>>>><><<<>><<<><>>>><<>><<>>>><<>><<<<>><><<>>>><<>>>><<>>>><<>>>><<<<>>><>><<<>>><<>><<>>>><<>>>><><<<><<<>>><<>><<><>><><<>>><><<<>>>><<><<<<>>>><<<<>><><<<>><>>><<<>>><<><>>>><><>><<<>>>><<>><<>><<>>><>>><<<>>><<<>>>><<<<><<><<<<>>>><<<>>>><<<><<<<>>><<<>>>><<>><<<>><><<<>>>><<<><><<>>><<<<>><<>>><>>><<<<>>><>>><<<>><>>>><<>><<<>><<<<>>><><<><<>>><<>><<<<><<<>>>><<<>>><<>><<<<>>><<<<>><<>>>><><><<<<>>><<<<>>><>>>><<>>><>>>><<<<>><<<<><<<>><>><<<>><>>>><><<<>>>><<<>>><<<>>>><>>><<<<>><>>><<<<>>><<<>><>><<<>>><<><<>>>><<>>><<<><<<<>>>><<>><>><<>><>>>><<<<>>>><<<>>><>><<<>><>><<<>><><<<<>>><<<<><<<<><<>>>><>>>><<>>><>><>><<<>>>><>><<>>>><<<<>><<<<><<<>>>><>>>><<<<>><<<>>>><<>>><<<><<<><<<<>><<<<><><<><<<>><<<<>><<<<>>><<<><>><<<>>><<>>>><><<<<>><<<>>>><<<<>>><<<<>>><>>><<<<>>>><<>><<<<><<<<>><>><<<>>>><>>><>><<<<>>>><<<<>><<>><<<<>>><<>>>><<>><<<>>>><<<<>>><<>>>><>>><>>><<<>><<<>>><>><<<<>><<<>>><>>>><<>>><<<<><<<><>><>><<<<>><<<<>>><><<<><<<>>><>><<<<>><<<<><<<>>><<>>><<<<>><<<>>>><<>>>><<<>>>><<<>><<<>>>><<>><<><<<>>>><<>>>><<><<<<>>>><<<<><<<>><<>>><><<<<>>>><<><<<<>>><<<>>><<<<>><<>>><<<>><<<>>><<<<>>>><<<>>><>>>><<<<><<<>>>><<<<>><>><<<<>>>><<<>><<<><<>>><><><<>>><<<<>>><<<><>>>><<<>>><<<<><<><><<>>>><<<<>><<<>>>><<<>><<><<<<>><<>>>><<<>>><>><>>><<<<>>>><><>><<><<>>>><<<>>><><<<>>>><<<<>><<<>>>><<<<>>>><<<<>><><>>><<<>>>><<<>>><<>><<<>><>><<>>><<<>>>><<>>>><<<<><>><<><<<<><<<><<<><>>>><<<><<<><<<>>>><<<>>>><<<>>><><<>>>><<<>>><<<>>><<<<><<<<>><<<<>><<<><<>>>><<<<><>>>><><<<>>>><<<>>><<<>><>>><<>>><<<>><><>>><<>><<<<>><<<<>><<>>>><><<<<>>><>>><<<>>><<<<>>>><<<<><<<><>><<<<><<<<><<<<><<<<>>>><<><<>>>><<>>><>><<>>><<<>>>><><>>><>><<<>>><<>>><>>>><<>><<>>>><<>><<<>>>><<<>>><<<<>>><<>>>><<<<><<<<>>><<<<>>>><<>><<<>>><<<<>>><>><<<<>><<<>>><<<>>><<<>>>><<<>>><<>><>>>><<<<><<>><<>>>><>>><<<>>>><<<<>>><<<>>><<>>>><<<<>>><<>>><<<<>>>><>>><<<<><<><>>>><<<<>>>><>><<><<<>>>><<>>>><>>>><<<><<<<>>>><<<>>>><<>>><<<<>><<>>><>><>>>><<<<><<<<>>>><<>>><<<>><<>><>><<>><<>>><<>>>><<>><<<>>>><><<><><<<>><<<<><<<>>>><>>><<>>><<<>><>>><<>><<<>>><>>>><<<>>><<>>><<>><>>><<><<<>>>><<<<>>>><<<>>><<>><<><<<>>>><<<<>><>>><<<<><<<<>>>><<<>>><<<>><<<<>>><<<<>>>><<<><<>><<>>>><<><<>><<>>><<<>><<><>>><>>><>><<<>>><<>><<>>>><><<<>>>><<>>>><<>>><>>>><<>>><<<><<>><<<>>>><<<<><<<<><<>>>><<<><>>><<<><<>><<<>>><<<><><<>>><<>>><<<<><>><<>><<<><<>>><<><>>><<<<>>>><>>>><<<<><<><<<>>>><<<>><<>>>><<<><<>>>><>>><<<>>>><<<<>>>><<><<<<>>>><<>>><>>>><<<><<<<><<<<>>><<<>><<<<>><<<>><<><<<>>><<<>>><<<>><<<<><<<<>><<<<>>><<<>><>>><<<<><<>>><>><<>>><<<<>>>><>>><>><<<>>><<>>><<>>>><<<><<>><>><>><<<>>>><>>>><><>><>>><<<>><<<<><<<<>><><<<>><<<>>><<<<><<>><<<<>>><><<><<<>>><<<>><>>>><>>>><<<>>><<<<>><<>>><<<<>><<>><><<<>><<>>><<>>><<<>>><<<<><<<<>>>><>>>><<<<>>><<<<>>><<<<>>><>><<><<>>><<<>><>>><<>>>><<<>>>><<>>><>>>><>><<>>><><<<<>><<<<><<<<><<<>>><<<<>>><<>><<<<>>><<<>>>><><<>><>><>>>><<<<>>><<<<>>>><<<<><<>><<<<>>>><<<>><<<<>>><<<<>>>><>>>><<<>><>>><<<<>>>><<<<><<<<>>>><<>><<<<>>><<<<><<><<<<>><>>><<<>>><<<>>><<<>>><<<>><<<<>>>><<<>>><>>>><<<<>>>><<>>><><<>><>><>><>><>>>><<<<>>>><<<>>>><<<<>><<<<>><<<<>>><<><>>>><>>><<<<>><<<>><<<<>><<>><><><<<>>><>><><<>><>>><<<><>>><>>>><<><<<<>>>><<>><<>><<<><<>>><<>>>><<<<>>><<<>><<>>><<<<><<<>>>><<<<>>>><><>>>><>><<>>><<<<>>><<>><<<>>><<<<>><>>><<>><<><<>><<>>>><<<<><<<<>><<<<>>>><<<<><<<<><><>>>><<>>><><<>>>><<<><>>>><<<<>>><<<<>>>><<<<>><<<>>>><<<<>><<<><<<<>>><<<<>>>><>><>>><<<><><<<>>>><<<<>>><>><>>><<>>><<>><<>>>><>>><<><<><><<>><<<>><>>><>>>><>>><>>><<>>><<<<><>><<><<<>><<<<>>><<<><>>><<>><>><<<<>>><>><<<<>>>><>>>><>>><>><<<><<>>><<>><<<<>>><<<<>><<><<<<><<>><<<><<<><><<<<>><<>><<<<><<<<>><>><<>><>><<<<>>><><><<<<>><<<>><<<>>><<<<>>>><<<><><<<><<><>><<<<>>>><><>><>>>><<><>>>><<>>><>><<>>>><<<>>><>>>><<>>>><<>>><<<><<<<>>>><<<<><<<>>>><<>><<<<>>>><<<><>>>><<<><<<><<<>><<<>><<<>><<<>><<>>>><<<>>><<>><<<<>><<><<>>><<<<><><<<<><>>>><<>><<>>><<<>><<<>>><<<>>>><<<>>>><<<<><<<><<<<>>><<>><<<><<<<>><<<>>>><<<><>>>><>>><<<><>><<<>>>><<<<>><<<<>>><<<<>>><>>><<<>><<<>>>><>>><<><<><<<>><<<>>>><<><<><>>>><<><<<<>>><<<<>>><<<>>>><<>>>><<<><<<<>>><<><<<>>>><<>><<<<>>><><<<<><<>><<<>>><<<>>>><<<<>>><><><<<>>><<<>>>><>>><>>><>>>><<<<>>>><>>><<<<><<><<<>>>><<<<><<<>><>><<<><<<><<>>><<<<>>><<>>><<<><<<<>>><<<<><<<<>><<><<<<>>><<>>>><>><<>>>><<<>>><><>>>><<<<>>><>><<<>>>><<><<>><<>>>><<<<>>>><<<<>>><>>><<<<>>>><<>>>><<<>>><<<<>><<>>>><<>>><>>><<<<>>><<<<>><>>><>>>><><><<>><<<<>>><<<>><<<>>><<>>><<><>>><>>>><<>><<<<>>><<<>>>><<>><>>>><<<<><>>>><<>><<><<<>>>><<<<><<<>>>><<<<>>><<<<>><><>>><><<<<>>><<<><<>>>><<<>>><<>>><<<<>>>><>>><>><>>>><>><<>>>><<<><<>>>><<<<><>><<>>><<>>>><<>><<>>>><>>>><<>>>><<<>>><<<<>>><<<<>>>><<<<>>><<>>>><<>>>><<<>><>>>><<<<>>>><<<<><<><<<<>>>><<<>><>><<>>>><<><><<<<>><<<<>><<<>>>><<<<>><<>>>><<<><<<<>>><>>><<<<><<<>>><<<>>>><<<><>><>><>>>><<<<>>>><<<>><<<<>><<<><<<<>>>><<<>><>>>><<<<>>><<<>>><<>><<<<>>><<<>><<<><>>>><>>><<<<><><><<<<>><<>><<<>>><<<<>><<>>><<>>>><<<>><<<><<>>><<<>>>><<>>><<<<>><<>>>><>><><<<<>><<<<>>>><>>>><<<>>>><<<<><<>>>><>>>><<<><<<>>>><<<>>>><<>><<>><<<>>>><>>><<<>>>><<<<>>><>>><<><<<><<<<>>><<>><<<<>>>><<<<>>>><<>><>><<>>><<<<><>>>><<<>>><<><<<<>>><>>><<>><<<<><<<>>><<<>><<<><<<>><<><>>>><<<><<<><<<>><<>>>><>>>><>><<<<>>>><<>><<<><><<>>>><<>>>><<<>><<<><<<>>>><<<>>><<>><<<>>>><<<<><<<<>><<>>><><<>>>><<<<>><<<<><<<><<<>><<>>>><<<<>>><<<><<<>>>><<<>>>><<>><<<><<<<>>><>>>><<<<><<<<>>><<<<>><<<><<<>>><<>>>><<>>>><<<<>>><<<>>>><<<<><><<<<>>><<<<>><<>><<><<>><<>><<<>><>>>><<<>><>><>><<<<>><><><>><><<<<>>><<<>>>><<>>>><<<>><<><<<>><<<>>>><>>><<<><>>>><>>>><<><>>>><<<>><<<>>><<<<>><<>><<<<>>>><<<>>><<>>>><<>>><<<><<<<><<>>><<<<>>>><<>>>><>><<>>><<<<>>>><<<>>><<>><<>>>><<<>><<>>>><<>>>><<<<>>><><<<>>>><<<<>>>><><<>>><<<>><>>><>>><<<<>>><><<>><<<<><<>>>><>>>><<<>>><<>><>><<<<>>><><<<<><>>><<<><<<>>><<<>>>><>><<<<>>><<<>>><<<>><>><<<<><<>>>><<<<>><>>>");
#else
    return string(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
#endif
}

struct Rock
{
  char shape[4][6];
};

const Rock r0{ "****", "    ", "    ", "    "};
const Rock r1{ " *  ", "*** ", " *  ", "    "};
const Rock r2{ "*** ", "  * ", "  * ", "    "};
const Rock r3{ "*   ", "*   ", "*   ", "*   "};
const Rock r4{ "**  ", "**  ", "    ", "    "};
const Rock rocks[5]{ r0, r1, r2, r3, r4 };

ostream& operator<<(ostream& os, const Rock& s) 
{
    os << "------------\n";
    for (int i = 0; i < 4; ++i)
    {
        os << s.shape[i] << '\n';
    }
    return os;
}

struct Row
{
    Row() { strcpy(data, "......."); }
    char data[8];
};
bool operator==(const Row &lhs, const Row &rhs)
{
    return memcmp(lhs.data, rhs.data, sizeof(lhs.data)) == 0;
}
ostream& operator<<(ostream& os, const Row& r)
{
    os << r.data;
    return os;
}

struct Map
{
    deque<Row> rows;
    int height = 0;
    int64_t front = 0;
};

bool Test(Map& map, const Rock & rock, int y, int x)
{
    if (y < 0)
    {
        assert(map.front == 0);
        return false;
    }
    while (map.rows.size() < y + 4)
    {
        Row empty;
        map.rows.push_back(empty);
    }

    for (int ry = 0; ry < 4; ++ ry)
    {
        for (int rx = 0; rx < 4; ++rx)
        {
            if (rock.shape[ry][rx] == '*')
            {
                int col = x+rx;
                if (col < 0 || col >= 7)
                    return false;
                if (map.rows[y+ry].data[col] != '.')
                    return false;
            }
        }
    }
    return true;
}

void Combine(Map& map, const Rock & rock, int y, int x, char ch, bool updateHeight = false)
{
    while (map.rows.size() < y + 4)
    {
        Row empty;
        map.rows.push_back(empty);
    }

    for (int ry = 0; ry < 4; ++ ry)
    {
        for (int rx = 0; rx < 4; ++rx)
        {
            if (rock.shape[ry][rx] == '*')
            {
                int col = x+rx;
                int row = y+ry;
                assert(col >= 0);
                assert(col <= 7);
                map.rows[row].data[col] = ch;
                if (updateHeight && row > map.height)
                {
                    map.height = row;
                }
            }
        }
    }
}

int window = 20;
void Resize(Map& map, int &y)
{
    while (map.rows.size() > window)
    {
        map.rows.pop_front();
        --y;
        --map.height;
        ++map.front;
    }
}

const char* YN(bool b) { return b ? "true" : "false"; }

ostream& operator<<(ostream& os, const Map& m)
{
    for (int i = m.rows.size()-1; i >= 0; --i)
    {
        os << '|' << m.rows[i].data << "|\n";
    }
    os << "+-------+ height " << m.front+m.height+1 << '\n';
    return os;
}

void clear()
{
    std::cout << "\033[2J\033[1;1H";
}

void sleep()
{
#if 0
    string line_string;
    getline(std::cin, line_string); 
#endif

#if 1
    usleep(100000);
#endif
}

int main()
{
    std::cout << std::setprecision(4);

    //const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    //string file_contents = LoadFile(input_filename);
    // SpaceCharacters(file_contents, "=");
    // StripCharacters(file_contents, ":,");
    //deque<sv> file_lines = ParseLines(file_contents);
    Map m;

    string input_text = GetInputText();
    sv input(input_text);

#if 0
    coutln(' ', rocks[0]);
    coutln(' ', rocks[1]);
    coutln(' ', rocks[2]);
    coutln(' ', rocks[3]);
    coutln(' ', rocks[4]);
#endif

    int oh = 0;
    int64_t oc = 0;
    int ri = 0;
    int rx = 2;
    int ry = 3;
    // int64_t goal = 2022;
    int64_t goal = 1000000000000;
    int64_t count = 0;
    int wraps = 0;
    while (count < goal)
    {
        Resize(m, ry);

        if (input.empty())
        {
            int nh = m.front+m.height+1;
            int dh = nh - oh;
            int64_t dc = count - oc;
            if (++wraps == 2)
            {
                coutln(" ", "wrap", dc, dh, input_text.length());
                while (count+dc+dc < goal)
                {
                    count += dc;
                    m.front += dh;
                }
            }
            oc = count;
            oh = nh;
            input = input_text;
        }

        char ch = input.front(); input.remove_prefix(1);
        int dx = 0;
        if (ch == '<')
            dx = -1;
        else if (ch == '>')
            dx = 1;
        else   
            assert(false);

        bool blow = Test(m, rocks[ri], ry, rx+dx);
        if (blow)
            rx += dx;

#if 1
        clear();
        Combine(m, rocks[ri], ry, rx, '*');
        coutln(' ', m);
        Combine(m, rocks[ri], ry, rx, '.');
        coutln(' ', rx, ry, ch, dx, YN(blow), "=blow");
        sleep();
#endif
        bool fall = Test(m, rocks[ri], ry-1, rx);
        if (fall)
        {
            ry -= 1;
        }
        else
        {
            Combine(m, rocks[ri], ry, rx, '*', true);
            ++count;
            ri = (ri + 1) % (sizeof(rocks)/sizeof(rocks[0]));
            rx = 2;
            ry = m.height + 4;

            double progress = (count * 100.0) / (double)goal;
            if (count % 10000 == 0) coutln(' ', "done", progress, "=progress", count, "=count", m.front+m.height+1, "=height");
        }

#if 1
        clear();
        Combine(m, rocks[ri], ry, rx, '*');
        coutln(' ', m);
        Combine(m, rocks[ri], ry, rx, '.');
        coutln(' ', rx, ry, ch, dx, YN(blow), "=blow", YN(fall), "=fall", count, "=count");
        sleep();
#endif
    }

    //clear();
    //coutln(' ', m);
    coutln(' ', "done", count, "=count", m.front+m.height+1, "=height");
 
    return 0;
}
