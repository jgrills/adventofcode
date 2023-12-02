#pragma once

#include <assert.h>
#include <charconv>
#include <iostream>
#include <queue>
#include <string_view>
#include <vector>

std::string LoadFile(const char * filename);
void SpaceCharacters(std::string& buffer, std::string_view what_to_replace);
void StripCharacters(std::string& buffer, std::string_view what_to_remove);

struct SplitResult
{
    std::string_view before;
    std::string_view after;
};
SplitResult Split(std::string_view text, char separator);
std::vector<std::string_view> Parse(std::string_view text, char separator);
std::deque<std::string_view> ParseLines(std::string_view text);

int atoi(std::string_view input);
std::string_view trim(std::string_view text);

template <class S, class T, class ... Ts>
void out(std::ostream& os, const S& sep, const T& first, const Ts & ... rest)
{
    os << first;
    (
        [&sep, &rest]{
            std::cout << sep;
            std::cout << rest;
        } (), ...
    );
}

template <class S, class T, class ... Ts>
void outln(std::ostream& os, const S& sep, const T& first, const Ts & ... rest)
{
    out(os, sep, first, rest ...);
    os << '\n';
}

template <class S, class T, class ... Ts>
void cout(const S& sep, const T& first, const Ts & ... rest)
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
void coutln(const S& sep, const T& first, const Ts & ... rest)
{
    cout(sep, first, rest ...);
    std::cout << '\n';
}

#if 0

template<typename T?
bool operator<(const T& lhs, const T& rhs)
{
    return lhs.estimate < rhs.estimate;
}

template<typename T?
ostream& operator<<(ostream& os, const T& s) 
{
    os << s.field;
    return os;
}

#endif
