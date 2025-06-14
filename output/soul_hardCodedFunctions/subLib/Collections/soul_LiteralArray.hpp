#pragma once
#include "soul_array.hpp"

template<typename T, size_t SIZE>
struct __Soul_LITERAL_ARRAY__ 
{
    std::array<T, SIZE> __Soul_raw_buffer__;

    using const_iterator = typename std::array<T, SIZE>::const_iterator;

    constexpr operator __Soul_ARRAY__<T>() const 
    {
        return this->__to_runtime_array__(SIZE);
    }

    constexpr __Soul_ARRAY__<T> __to_runtime_array__(size_t size) const
    {
        auto arr = __Soul_ARRAY__<T>(size);

        for(size_t i = 0; i < std::min(size, SIZE); i++)
            arr.__soul_UNSAFE_at__(i) = __Soul_raw_buffer__[i];

        return arr;
    }

    constexpr T __soul_UNSAFE_at__(uint32_t index) const {
        return __Soul_raw_buffer__[index];
    }

    constexpr size_t __literal_array_len__() const {
        return SIZE;
    }

    constexpr const char* __to_c_str__() const {
        return &__Soul_raw_buffer__[0];
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

template <size_t N>
constexpr __Soul_LITERAL_ARRAY__<char, N+1> __NEW_Soul_LITERAL_ARRAY_C_STR__(const char (&str)[N]) {
    __Soul_LITERAL_ARRAY__<char, N+1> arr = {};
    size_t i = 0;
    for (; i < N; ++i) {
        arr.__Soul_raw_buffer__[i] = str[i];
    }
    arr.__Soul_raw_buffer__[i] = '\0';

    return arr;
}