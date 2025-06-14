#pragma once
#include "../soul_cppLibs.hpp"

// class __Soul_Range_Iterator__
// {
// private:
//     uint32_t i;

// public:
//     typedef std::forward_iterator_tag iterator_category;
//     typedef uint32_t value_type;
//     typedef std::ptrdiff_t difference_type;
//     typedef const uint32_t* pointer;
//     typedef const uint32_t& reference;

//     __Soul_Range_Iterator__(uint32_t i) 
//         : i(i)
//     {
//     }

//     const uint32_t& operator*() { return i; }
//     __Soul_Range_Iterator__& operator++() { ++i; return *this; }
//     __Soul_Range_Iterator__ operator++(int) { __Soul_Range_Iterator__ tmp = *this; ++i; return tmp; }
//     bool operator!=(const __Soul_Range_Iterator__& other) const { return i != other.i; }
//     bool operator==(const __Soul_Range_Iterator__& other) const { return i == other.i; }
// };

template <typename T>
void __Soul_ARRAY_deleter__(T* ptr) 
{
    delete[] ptr;
}

struct __Soul_Range__
{
    enum class RangeType { START, END, START_END };
    RangeType type = RangeType::START_END;

    int64_t start = 0;  
    int64_t end = 0;

    __Soul_Range__() = default;
    __Soul_Range__(int64_t start, int64_t end) 
        : start(start), end(end), type(RangeType::START_END) 
    {
    }

    __Soul_Range__(int64_t index, RangeType type) 
        : type(type)
    {
        switch(type)
        {
            case RangeType::START:
                start = index;
                break;
            case RangeType::END:
                end = index;
                break;
            case RangeType::START_END:
                end = start = index;
                break;            
        }
    }

    // __Soul_Range_Iterator__ begin() const
    // {
    //     return __Soul_Range_Iterator__((type == RangeType::START_END || type == RangeType::START) ? start : 0);
    // }

    // __Soul_Range_Iterator__ end() const
    // {
    //     return __Soul_Range_Iterator__((type == RangeType::START_END || type == RangeType::END) ? end : UINT32_MAX);
    // }
};
