#pragma once

#include <assert.h>
#include <charconv>
#include <string_view>
#include <vector>

bool isspace(char ch, char separator);

int atoi(std::string_view input, bool trim=true);

struct SplitResult
{
    std::string_view before;
    std::string_view after;
};
SplitResult Split(std::string_view text, char separator);

std::vector<std::string_view> Parse(std::string_view text, char separator);
