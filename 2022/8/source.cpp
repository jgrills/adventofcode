#include "helper.h"

#include <assert.h>
#include <charconv>
#include <cstring>
#include <fstream>
#include <iostream>
#include <map>
#include <string>
#include <string_view>
#include <vector>

#define USE_LONG_INPUT 1
#define VERBOSE        2

#if USE_LONG_INPUT
const char* input_filename = "long";
constexpr int width = 99;
constexpr int height = 99;
#else
const char* input_filename = "short";
constexpr int width = 5;
constexpr int height = 5;
#endif

#define VV   if constexpr (VERBOSE > 0)
#define VVV  if constexpr (VERBOSE > 1)
#define VVVV if constexpr (VERBOSE > 2)

const char* YN(bool b)
{
    return b ? "v" : "i";
}

const char* blocked(bool b)
{
    return b ? "b" : "v";
}

int tree_height[height][width];
bool tree_visible[height][width];

void scan(int x, int y, int dirx, int diry)
{
    int h = tree_height[y][x];
    for (;;) {
        int x1 = x += dirx;
        int y1 = y += diry;
        if (x1 < 0) return;
        if (x1 >= width) return;
        if (y1 < 0) return;
        if (y1 >= height) return;
        int h1 = tree_height[y1][x1];
        if (h1 > h)
        {
            tree_visible[y][x] = true;
            // std::cout << "made visible " << x << " " << y << " @ " << h << " : " << x1 << " " << y1 << " @ " << h1 << "\n";
            h = h1;
        }
        x = x1;
        y = y1;
    }
}

int ScanB(int x, int y, int dirx, int diry)
{
    int ox = x;
    int oy = y;
    int result = 0;
    int h = tree_height[y][x];
    for (;;)
    {
        int x1 = x + dirx;
        int y1 = y + diry;
        if (x1 < 0 || x1 >= width || y1 < 0 || y1 >= height)
        {
            if (ox == 2 && oy == 1) std::cout << "oob " << x1 << " " << y1 << "\n";
            break;
        }
        int h1 = tree_height[y1][x1];
        bool b = (h1 >= h);
        if (ox == 2 && oy == 1) std::cout << x << y << h << " " << dirx << " " << diry << " " << x1 << y1 << h1 << " " << blocked(b)  << "\n";
        result += 1;
        if (b) break;
        x = x1;
        y = y1;
    }
    if (ox == 2 && oy == 1) std::cout << x << y << h << " result: " << result << "\n";
    return result;
}

int ScanB(int x, int y) {
    int result = 1;
    result *= ScanB(x, y, 1, 0);
    result *= ScanB(x, y, -1, 0);
    result *= ScanB(x, y, 0, 1);
    result *= ScanB(x, y, 0, -1);
    return result;
}

void PartA()
{
    std::ifstream input_file(input_filename);
    int total = 0;
    for (int y = 0; y < height; ++y)
    {
        std::string line_string;
        if (!std::getline(input_file, line_string)) break;
        std::string_view line = line_string;
        assert(line.length() == width);
        for (int x = 0; x < width; ++x)
        {
            tree_height[y][x] = line[x] - '0';
        }
    }

    for (int y = 0; y < height; ++y)
    {
        tree_visible[y][0] = true;
        tree_visible[y][width-1] = true;
    }
    for (int x = 0; x < width; ++x)
    {
        tree_visible[0][x] = true;
        tree_visible[height-1][x] = true;
    }

    for (int y = 1; y < height; ++y)
    {
        scan(0, y, 1, 0);
        scan(width-1, y, -1, 0);
    }
    for (int x = 1; x < width-1; ++x)
    {
        scan(x, 0, 0, 1);
        scan(x, height-1, 0, -1);
    }

    int visible = 0;
    for (int y = 0; y < height; ++y)
    {
        std::cout << y << ':';
        for (int x = 0; x < width; ++x)
        {
            std::cout << ' ' << tree_height[y][x] << YN(tree_visible[y][x]);
            if (tree_visible[y][x]) visible += 1;
             
        }
        std::cout << "\n";
    }
    std::cout << "vis " << visible << '\n';

    int best = -1;
    int bestx = -1;
    int besty = -1;
    for (int y = 0; y < height; ++y)
    {
        for (int x = 0; x < width; ++x)
        {
            int r = ScanB(x, y);
            if (r >= best)
            {
                best = r;
                bestx = x;
                besty = y;
            }
        }
    }
    std::cout << "best" << bestx << " " << besty << " " << best <<'\n';
}

int main()
{
    PartA();
    return 0;
}
