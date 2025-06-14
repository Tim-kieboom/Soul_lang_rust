#pragma once
#include "../soul_cppLibs.hpp"

/*
-------------- NON RECURSIVE EXP --------------

    double result = 1.0;
    double term = 1.0;

    for (int i = 1; i <= terms; ++i) {
        term *= x / i;
        result += term;
    }

    return result;
----------------------------------------------
*/
namespace __Soul_CompileConst_math__
{

constexpr double _CompileConst_exp(const double x, const uint32_t terms, const double result, const double term, const uint64_t i) noexcept
{
    return  (i <= terms) ?
                _CompileConst_exp
                (
                    x,
                    terms,
                    (result + term),
                    (term * (x / i)),
                    (i + 1)
                ) :
                result;
}

constexpr double _exp(const double x, const uint32_t terms) noexcept
{
    return _CompileConst_exp(x, terms, 1.0f, 1.0f, 1) - 1;
}

//calculates e^x
constexpr double exp(const double x) noexcept
{
    return _exp(x, 20);
}
}