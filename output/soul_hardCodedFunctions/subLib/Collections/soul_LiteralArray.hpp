#pragma once
#include "soul_array.hpp"

template<typename T, size_t SIZE>
struct __Soul_LITERAL_ARRAY__ 
{
    std::array<T, SIZE> __Soul_raw_buffer__;

    using const_iterator = typename std::array<T, SIZE>::const_iterator;

    __Soul_ARRAY__<T> __to_runtime_array__(size_t size) 
    {
        auto arr = __Soul_ARRAY__<T>(size);

        for(size_t i = 0; i < std::min(size, sizeof(SIZE)); i++)
            arr.__soul_UNSAFE_at__(i) = __Soul_raw_buffer__[i];

        return arr;
    }

    constexpr const_iterator begin() const noexcept 
    {
        return __Soul_raw_buffer__.cbegin();
    }

    constexpr const_iterator end() const noexcept 
    {
        return __Soul_raw_buffer__.cend();
    }
};

template<typename T, T... Values>
constexpr __Soul_LITERAL_ARRAY__<T, sizeof...(Values)> __NEW_Soul_LITERAL_ARRAY__() 
{
    return __Soul_LITERAL_ARRAY__<T, sizeof...(Values)>{{Values...}};
}  