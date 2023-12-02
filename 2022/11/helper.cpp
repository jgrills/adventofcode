#include "helper.h"

bool isspace(char ch, char separator)
{
    return (std::isspace(ch) || ch == ':' || ch == ',');
}

int atoi(std::string_view input, bool trim)
{
    if (trim)
    {
        while (!input.empty() && std::isspace(input.front()))
            input.remove_prefix(1);
        while (!input.empty() && std::isspace(input.back()))
            input.remove_suffix(1);
        assert(!input.empty());
    }

    int out;
    const std::from_chars_result result = std::from_chars(input.data(), input.data() + input.size(), out);
    assert(result.ec == std::errc());
    assert(result.ptr == input.data() + input.length());
    return out;
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
        while (!text.empty() && isspace(text.front(), separator))
            text.remove_prefix(1);
        if (!text.empty())
        {
            auto [first, rest] = Split(text, separator);
            while (!first.empty() && isspace(first.back(), separator))
                first.remove_suffix(1);
            result.push_back(first);
            text = rest;
        }
    }
    return result;
}
