#pragma once
#include "soul_CompileConst_exp.hpp"
#include "soul_CompileConst_log.hpp"
#include <limits>

namespace __Soul_CompileConst_math__ {

constexpr double pow2(const double base) noexcept {
    return base * base;
}

template <typename T>
constexpr double abs(const T number) noexcept {
    return (number < 0) ? -number : number;
}

constexpr int64_t round(const double number) noexcept {
    return (int64_t)(number + 0.5f);
}

constexpr double _pow_int(const double base, const int64_t exponent, const double result) noexcept {
    return (exponent <= 0) ? result 
            : _pow_int(base, exponent - 1, result * base);
}

constexpr double pow_int(const double base, const int64_t exponent) noexcept {
    return (exponent < 0) ? 1.0f / _pow_int(base, (exponent * -1) -1, base) 
            : _pow_int(base, exponent - 1, base);
}

constexpr double pow_float(const double base, const double exponent) noexcept {
    return exp(exponent * log(base));
}

constexpr bool _isInteger(const double number) noexcept {
    //'< epsilon' to account for floating point errors
    return (abs(number - round(number)) < std::numeric_limits<double>::epsilon());
}

constexpr double pow(const double base, const double exponent) noexcept {
    return (exponent == 0.0) ? 1.0 
            : (base == 0.0) ? 0.0 
            : (base == 1.0) ? 1.0 
            : (_isInteger(exponent)) ? pow_int(base, (int64_t)exponent) //exponent DOESN'T have a decimal
            : pow_float(base, exponent); //exponent DOES have a decimal
                        
}

constexpr double sqrt(const double base) noexcept {
    return pow(base, 0.5f);
}

constexpr double root(const double base, const double exponent) noexcept {
    return pow(base, 1.0f / exponent);
}
}