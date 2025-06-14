#pragma once
#include "soul_CompileConst_math.hpp"	

#define constSqrt __Soul_CompileConst_math__::sqrt

inline double root(double base, double x) noexcept
{
    return pow(base, (1 / x));
}

inline double log(double base, double x)
{
    return std::log(base) / std::log(x);
}