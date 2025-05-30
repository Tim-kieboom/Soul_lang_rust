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

template <typename T>
constexpr uint8_t u8(T value)
{
    return static_cast<uint8_t>(value);
}

template <typename T>
constexpr uint16_t u16(T value)
{
    return static_cast<uint16_t>(value);
}

template <typename T>
constexpr uint32_t u32(T value)
{
    return static_cast<uint32_t>(value);
}

template <typename T>
constexpr uint64_t u64(T value)
{
    return static_cast<uint64_t>(value);
}

template <typename T>
constexpr size_t uSize(T value)
{
    return static_cast<size_t>(value);
}

template <typename T>
constexpr int8_t i8(T value)
{
    return static_cast<int8_t>(value);
}

template <typename T>
constexpr int16_t i16(T value)
{
    return static_cast<int16_t>(value);
}

template <typename T>
constexpr int32_t i32(T value)
{
    return static_cast<int32_t>(value);
}

template <typename T>
constexpr int64_t i64(T value)
{
    return static_cast<uint64_t>(value);
}

template <typename T>
constexpr float f32(T value)
{
    return static_cast<float>(value);
}

template <typename T>
constexpr double f64(T value)
{
    return static_cast<double>(value);
}

template <typename T>
constexpr float char_(T value)
{
    return static_cast<float>(value);
}

template <typename T>
constexpr double bool_(T value)
{
    return static_cast<bool>(value);
}