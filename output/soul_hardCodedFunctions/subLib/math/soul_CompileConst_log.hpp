#pragma once
#include "../soul_cppLibs.hpp"
/*
-------------- NON RECURSIVE NATURAL_LOG --------------

    if (x <= 0)
        return 0;

    // Confine x to a sensible range
    int power_adjust = 0;
    while (x > 1.0)
    {
        x /= EULER_CONST;
        power_adjust++;
    }

    while (x < 0.25)
    {
        x *= EULER_CONST;
        power_adjust--;
    }

    // Now use the Taylor series to calculate the logarithm
    x -= 1.0;
    double t = 0.0, s = 1.0, z = x;
    for (int k = 1; k <= TAYLOR_ITERATIONS; k++)
    {
        t += z * s / k;
        z *= x;
        s = -s;
    }

    // Combine the result with the power_adjust value and return
    return t + power_adjust;

----------------------------------------------
*/
namespace __Soul_CompileConst_math__
{
    
struct __Soul_CompileConst_log_Result__
{
    double base;
    int64_t powerAdjust;

    constexpr __Soul_CompileConst_log_Result__(double base, int64_t powerAdjust) noexcept
        : base(base), powerAdjust(powerAdjust)
    {
    }
};

constexpr double _CompileConst_log_Taylor(const double base, const double t, const double z, const double s, const uint8_t i) noexcept
{
    constexpr uint8_t TAYLOR_ITERATIONS = 20;

    return  (i > TAYLOR_ITERATIONS) ? 
                t : 
                _CompileConst_log_Taylor
                (
                    base, 
                    t + (z * s / i), 
                    z * base, 
                    -s, 
                    i + 1
                );
}


constexpr double _CompileConst_log_Taylor(const __Soul_CompileConst_log_Result__ result) noexcept
{
    return _CompileConst_log_Taylor(result.base - 1, 0.0f, 1.0f, result.base - 1, 1) + result.powerAdjust;
}

constexpr __Soul_CompileConst_log_Result__ _CompileConst_log_lower(const double base, const int64_t powerAdjust) noexcept
{
    constexpr double EULER_CONST = 2.718281828459045235f;

    return (base < 0.25f) ? _CompileConst_log_lower(base * EULER_CONST, powerAdjust-1) : __Soul_CompileConst_log_Result__(base, powerAdjust);
}

constexpr __Soul_CompileConst_log_Result__ _CompileConst_log_upper(const double base, const int64_t powerAdjust) noexcept
{
    constexpr double EULER_CONST = 2.718281828459045235f;

    return (base > 1.0f) ? _CompileConst_log_upper(base / EULER_CONST, powerAdjust+1) : __Soul_CompileConst_log_Result__(base, powerAdjust);
}

constexpr __Soul_CompileConst_log_Result__ _CompileConst_log_upper(const __Soul_CompileConst_log_Result__ result) noexcept
{
    return _CompileConst_log_upper(result.base, result.powerAdjust);
}

constexpr double log(const double base) noexcept
{
    return  (base <= 0) ? 
                0 : 
                _CompileConst_log_Taylor
                (
                    _CompileConst_log_upper(_CompileConst_log_lower(base, 0))
                );
}

constexpr double log(const double base, const double exponent) noexcept
{
    return log(base) / log(exponent);
}
}