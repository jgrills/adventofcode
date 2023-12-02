#pragma once

#include <assert.h>
#include <charconv>
#include <iostream>
#include <string_view>
#include <queue>
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

template <class S, class T, class ... Ts>
void out(const S& sep, const T& first, const Ts & ... rest)
{
    std::cout << first;
    (
        [&sep, &rest]{
            std::cout << sep;
            std::cout << rest;
        } (), ...
    );
}

template <class S, class T, class ... Ts>
void outln(const S& sep, const T& first, const Ts & ... rest)
{
    out(sep, first, rest ...);
    std::cout << '\n';
}

