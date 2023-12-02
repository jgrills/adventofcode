#include "helper.h"

#include <iterator>
#include <fstream>

int atoi(std::string_view text)
{
    int out;
    const std::from_chars_result result = std::from_chars(text.data(), text.data() + text.size(), out);
    assert(result.ec == std::errc());
    assert(result.ptr == text.data() + text.length());
    return out;
}

std::string_view trim(std::string_view text)
{
    while (!text.empty() && std::isspace(text.front()))
        text.remove_prefix(1);
    while (!text.empty() && std::isspace(text.back()))
        text.remove_suffix(1);
    return text;
}

SplitResult Split(std::string_view text, char separator)
{
    const int index = text.find(separator);
    if (index == std::string_view::npos)
        return SplitResult{ text, std::string_view{} };

    std::string_view before = text.substr(0, index);
    std::string_view after = text.substr(index+1);
    return SplitResult{ before, after };
}

std::vector<std::string_view> Parse(std::string_view text, char separator)
{
    std::vector<std::string_view> result;
    while (!text.empty())
    {
        text = trim(text);
        if (!text.empty())
        {
            auto [first, rest] = Split(text, separator);
            trim(first);
            result.push_back(first);
            text = rest;
        }
    }
    return result;
}

std::string LoadFile(const char* filename)
{
    std::ifstream ifs(filename);
    std::string str(std::istreambuf_iterator<char>{ifs}, {});
    return str;
}

std::deque<std::string_view> ParseLines(std::string_view text)
{
    std::deque<std::string_view> result;
    while (!text.empty())
    {
        auto [first, rest] = Split(text, '\n');
        result.push_back(first);
        text = rest;
    }

    return result;
}

void SpaceCharacters(std::string& buffer, std::string_view removals)
{
    for (char& c : buffer)
    {
        for (char r : removals)
            if (c == r)
            {
                c =  ' ';
                break;
            }
    }
}

void StripCharacters(std::string& buffer, std::string_view removals)
{
    int len = 0;
    for (int i = 0; i < buffer.size(); ++i)
    {
        bool remove = false;
        for (char r : removals)
            if (buffer[i] == r)
            {
                remove = true;
                break;
            }

        if (!remove)
            buffer[len++] = buffer[i];
    }
    buffer.resize(len);
}

void unbuffer(std::ostream &os)
{
    os.setf(std::ios::unitbuf);
}

void clear(std::ostream &os)
{
    os << "\033[2J\033[1;1H";
}
