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
void sout(std::ostream& os, const S& sep, const T& first, const Ts & ... rest)
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
void soutln(std::ostream& os, const S& sep, const T& first, const Ts & ... rest)
{
    sout(os, sep, first, rest ...);
    os << '\n';
}

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

struct Vec2
{
    int x{0};
    int y{0};
};
inline bool operator<(const Vec2 &lhs, const Vec2 &rhs)
{
    if (lhs.y < rhs.y) return true;
    if (lhs.y > rhs.y) return false;
    if (lhs.x < rhs.x) return true;
    return false;
}
inline Vec2 min(const Vec2 &lhs, const Vec2 &rhs)
{
    return Vec2{std::min(lhs.x, rhs.x), std::min(lhs.y, rhs.y)};
}
inline Vec2 max(const Vec2 &lhs, const Vec2 &rhs)
{
    return Vec2{std::min(lhs.x, rhs.x), std::min(lhs.y, rhs.y)};
}
inline Vec2 operator+(const Vec2 &lhs, const Vec2 &rhs)
{
    return Vec2{lhs.x + rhs.x, lhs.y + rhs.y};
}
inline Vec2& operator+=(Vec2 &lhs, const Vec2 &rhs)
{
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    return lhs;
}

void clear();

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
