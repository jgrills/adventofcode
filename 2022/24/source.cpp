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

#define DAY            24
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

struct Blizzard
{
    Vec2 at;
    Vec2 step;
    char ch;
};
vector<Blizzard> bliz;

int width, height;

const int max_turns = 850;
char turn_maps[max_turns][32][128];
int8_t expanded[max_turns][32][128];

const Vec2 steps[5] { {0,0}, {1,0}, {0,1}, {-1,0}, {0,-1} };
Vec2 goal[2] {{1,0}, {0,0}};

void Print(int turn, const Vec2& at=Vec2{0,0}, char at_ch='\0')
{
    char &ch = turn_maps[turn][at.y][at.x];
    if (at_ch)
    {
        assert(ch == '.');
        ch = at_ch;
    }

    for (int i = 0; i < height; ++i)
        outln("", turn_maps[turn][i]);

    if (at_ch)
        ch = '.';
}

int map_turn = -1;
void BuildMap(int desired_turn)
{
    // Build all the maps sequentailly
    while (map_turn < desired_turn)
    {
        ++map_turn;
        VVV outln(' ', "BuildMap", map_turn, desired_turn);

        // Fill space with dots
        memset(turn_maps[map_turn], '.', sizeof(turn_maps[map_turn]));

        // horizontal walls.
        // Add an extra at `height` that's fully filled to avoid having to range change y==height
        for (int x = 0; x < width; x++)
        {
            turn_maps[map_turn][0][x] ='#';
            turn_maps[map_turn][height-1][x] ='#';
            turn_maps[map_turn][height][x] ='#';
        }

        // vertical walls
        for (int y = 1; y < height-1; y++)
        {
            turn_maps[map_turn][y][0] = '#';
            turn_maps[map_turn][y][width-1] = '#';
            turn_maps[map_turn][y][width] ='\0';
        }

        // Add start and goal back in
        turn_maps[map_turn][0][width] ='\0';
        turn_maps[map_turn][0][1] ='.';
        turn_maps[map_turn][height-1][width-2] ='.';
        turn_maps[map_turn][height-1][width] ='\0';

        // Add all the blizzards
        for (Blizzard &b : bliz)
        {
            Vec2 &at = b.at;
            char &ch = turn_maps[map_turn][at.y][at.x];
            if (ch == '.')
                ch = b.ch;
            else if (isdigit(ch))
                ch++;
            else
                ch = '2';

            // Animation the blizzards
            at += b.step;
            if (at.x == 0) at.x = width-2;
            else if (at.x == width-1) at.x = 1;
            else if (at.y == 0) at.y = height-2;
            else if (at.y == height-1) at.y = 1;
        }
    }
}

void ReadMap(deque<sv> &file_lines)
{
    Vec2 at{0, 0};
    while (!file_lines.empty())
    {
        sv line = file_lines.front(); file_lines.pop_front();
        width = line.size();
        at.x = 0;
        for (char ch : line)
        {
            if (ch == '.')
            {
            }
            else if (ch == '#')
            {
            }
            else if (ch == '<' || ch == '^' || ch == '>' || ch == 'v')
            {
                Vec2 step;
                if (ch == '<') step.x = -1;
                else if (ch == '>') step.x = 1;
                else if (ch == '^') step.y = -1;
                else if (ch == 'v') step.y = 1;
                else assert(false);
                bliz.push_back(Blizzard{at, step, ch});
            }
            else
                assert(false);
            ++at.x;
        }
        ++at.y;
    }
    height = at.y;

    goal[0].x = 1;
    goal[0].y = 0;
    goal[1].x = width - 2;
    goal[1].y = height - 1;
}

struct Route
{
    Vec2 at;
    int goal;
    int turn;
    int estimate;
    char moves[max_turns] = { (char)255 };
};
int Estimate(const Route &turn)
{
    return abs(goal[turn.goal].x - turn.at.x) + abs(goal[turn.goal].y - turn.at.y);
}
bool operator<(const Route &lhs, const Route &rhs)
{
    return lhs.turn + lhs.estimate > rhs.turn + rhs.estimate;
}

int Pathfind(Route &turn)
{
    priority_queue<Route> routes;
    turn.estimate = Estimate(turn);
    routes.push(turn);
    while (routes.size())
    {
        turn = routes.top(); routes.pop();
        if (turn.at == goal[turn.goal])
        {
            outln(' ', "done", turn.turn, turn.at.x, turn.at.y, turn.estimate);
            return turn.turn;
        }

        assert(turn.turn+1 <= max_turns);
        int8_t &exp = expanded[turn.turn][turn.at.y][turn.at.x];
        if (exp == 0)
        {
            exp = 'Y';
            turn.turn += 1;
            assert(turn.turn < max_turns);

            // Build the map if it hasn't been seen
            BuildMap(turn.turn);

            const Vec2 prev = turn.at;
            for (int i = 0; i < 5; ++i)
            {
                turn.at = prev + steps[i];
                if (turn.at.x >= 0 && turn.at.y >= 0 && turn_maps[turn.turn][turn.at.y][turn.at.x] == '.')
                {
                    turn.moves[turn.turn] = i;
                    turn.estimate = Estimate(turn);
                    routes.push(turn);
                }
            }
        }
    }
    outln("", "Search exhaused space");
    return -1;
}

void PrintSteps(Route& turn, Vec2 at, int t0, int t1)
{
#if 0
    for (int i = t0; i <= t1; ++i)
    {
        if (turn.moves[i] == (char)255)
            outln(" ", "bad move", i, (int)turn.moves[i]);
        else
        {
            outln(" ", "good move", i, steps[(int)turn.moves[i]]);
            at += steps[(int)turn.moves[i]];
        }
        outln(' ', i, "of", turn.turn, '{', at.x, at.y, '}');
        Print(i, at, 'E');
        cout << '\n';
    }
#endif
}

void Day24(deque<sv> &file_lines)
{
    ReadMap(file_lines);

    Route turn{goal[0], 1, 0, 0 };
    turn.estimate = Estimate(turn);
    int r0 = Pathfind(turn);
    outln(' ', "Found in", r0, "steps");
    PrintSteps(turn, goal[0], 0, r0);

    turn = Route{goal[1], 0, r0+1, 0};
    turn.estimate = Estimate(turn);
    int r1 = Pathfind(turn);
    outln(' ', "Found in", r1, r1-r0, "steps");
    PrintSteps(turn, goal[1], r0+1, r1);

    turn = Route{goal[0], 1, r1+1, 0};
    turn.estimate = Estimate(turn);
    int r2 = Pathfind(turn);
    outln(' ', "Found in", r2, r2-r1, "steps");
    PrintSteps(turn, goal[0], r1+1, r2);
}

int main()
{
    unbuffer(cout);
    const char* input_filename = USE_LONG_INPUT ? "long" : "short";
    string file_contents = LoadFile(input_filename);
    deque<sv> file_lines = ParseLines(file_contents);

    Day24(file_lines);
    return 0;
}
