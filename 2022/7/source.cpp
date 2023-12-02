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
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>

#define USE_PART_A     1
#define USE_PART_B     0
#define USE_LONG_INPUT 1
#define VERBOSE        2

#if USE_LONG_INPUT
const char* input_filename = "long";
#else
const char* input_filename = "short";
#endif

#define VV   if constexpr (VERBOSE > 0)
#define VVV  if constexpr (VERBOSE > 1)
#define VVVV if constexpr (VERBOSE > 2)

struct Directory
{
    std::string name;
    Directory* parent{nullptr};
    std::map<std::string, int> files;
    std::map<std::string, Directory*> dirs;

    int size(std::string path) const;
    int result(std::string path) const;
    void find(std::string path, int need, std::string& spath, int &ssize);
};

int Directory::size(std::string path) const {
    int result = 0;
    std::string full = path + "/" + name;
    for (auto [k,v] : files)
    {
        result += v;
    }
    for (auto [k,v] : dirs)
    {
        result += v->size(full);
    }
    VVV std::cout << full << " " << result << "\n";
    return result;
}

int Directory::result(std::string path) const {
    std::string full = path + "/" + name;
    int result = 0;
    int s = size(path);
    if (s >= 100000)
    {
        VVV std::cout << "large " << path <<  " " << s << "\n";
    }
    else
    {
        VVV std::cout << "small " << path <<  " " << s << "\n";
        result += s;
    }

    for (auto [k,v] : dirs)
    {
        result += v->result(full);
    }

    return result;
}

void Directory::find(std::string path, int need, std::string& spath, int &ssize)
{
    std::string full = path + "/" + name;

    int s = size(path);
    if (ssize == 0 || (s >= need && s < ssize))
    {
        spath = full;
        ssize = s;
    }

    for (auto [k,v] : dirs)
    {
        v->find(full, need, spath, ssize);
    }
}

void PartA()
{
    std::ifstream input_file(input_filename);
    int total = 0;

    std::string line_string;
    if (!std::getline(input_file, line_string)) assert(false);
    Directory root;
    Directory* cwd = &root;
    for (;;)
    {
        if (!std::getline(input_file, line_string)) break;
        std::string_view line = line_string;
        std::cout << line << "\n";
        std::vector<std::string_view> tokens = Parse(line, ' ');

        if (tokens[0] == "$")
        {
            if (tokens[1] == "cd")
            {
                if (tokens[2] == "..")
                {
                    VV std::cout << "CD .."  << "\n";
                    cwd = cwd->parent;
                }
                else
                {
                    VV std::cout << "CD " << tokens[2] << "\n";
                    cwd = cwd->dirs[std::string(tokens[2])];
                }
            }
        }
        else if (tokens[0] == "dir")
        {
            std::string dir(tokens[1]);
            Directory* n = new Directory;
            n->name = dir;
            n->parent = cwd;
            cwd->dirs[dir] = n;
            VV std::cout << "DIR " << dir << "\n";
        }
        else
        {
            int s = atoi(tokens[0]);
            std::string f(tokens[1]);
            cwd->files[f] = s;
            std::cout << "FILE " << f << " " << s << "\n";
        }
    }

    int result = root.size("");
    int need = 30000000 - (70000000 - result);

    std::string spath;
    int ssize = 0;
    root.find("", need, spath, ssize);

    std::cout << "result " << result << " " << spath << " " << ssize << "\n";
}

int main()
{
#if USE_PART_A
    PartA();
#endif
#if USE_PART_B
    PartB();
#endif
    return 0;
}
