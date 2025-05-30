#pragma once
#include "soul_cppLibs.h"
#include "Collections/soul_str.hpp"

static inline void Throw(__Soul_STR__ msg)
{
    throw std::runtime_error(msg.__Soul_getC_Str__());
}

static inline void Fail(__Soul_STR__ msg)
{
    std::cout << msg.__Soul_getC_Str__() << std::endl;
    assert(("assertFail failed (read last console message)", false));
}

static inline void checkThrow(bool condition, __Soul_STR__ msg)
{
    if (condition)
        Throw(msg);
}

static inline void checkFail(bool condition, __Soul_STR__ msg)
{
    if (condition)
        Fail(msg);
}

static inline void checkFail_debug(bool condition, __Soul_STR__ msg)
{
#ifndef NDEBUG
    checkFail(condition, msg);
#endif
}

static inline void checkThrow_debug(bool condition, __Soul_STR__ msg)
{
#ifndef NDEBUG
    checkThrow(condition, msg);
#endif
}