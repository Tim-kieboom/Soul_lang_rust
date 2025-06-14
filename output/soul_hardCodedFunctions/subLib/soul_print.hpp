#pragma once
#include "Collections/soul_Collections.hpp"

inline void print(){ }

template<typename T, typename = void>
inline void Print(T msg)
{
    std::cout << msg;
}

template<typename K>
inline void Print(__Soul_ARRAY__<K> array)
{
    if(array.size() == 0)
    {
        std::cout << "[empty]";
        return;
    }

    std::cout << '[';

    int64_t lastIndex = array.size() - array.offset() - 1;
    for (int64_t i = 0; i < lastIndex; i++)
    {
        print_element(array.__soul_UNSAFE_at__(i), std::is_same<K, __Soul_ARRAY__<char>>());
        std::cout << ", ";
    }

    if(lastIndex >= 0)
        print_element(array.__soul_UNSAFE_at__(lastIndex), std::is_same<K, __Soul_ARRAY__<char>>());

    std::cout << ']';
}

template<>
inline void Print(__Soul_ARRAY__<char> msg)
{
    for(auto ch : msg) {
        std::cout << ch;
    }
}


template<size_t SIZE>
inline void Print(__Soul_LITERAL_ARRAY__<char, SIZE> array)
{
    std::cout << array.__to_c_str__();
}

template<typename K, size_t SIZE>
inline void Print(__Soul_LITERAL_ARRAY__<K, SIZE> array)
{
    if(array.__literal_array_len__() == 0)
    {
        std::cout << "[empty]";
        return;
    }

    std::cout << '[';

    int64_t lastIndex = array.__literal_array_len__() - 1;
    for (int64_t i = 0; i < lastIndex; i++)
    {
        print_element(array.__soul_UNSAFE_at__(i), std::is_same<K, __Soul_ARRAY__<char>>());
        std::cout << ", ";
    }

    if(lastIndex >= 0)
        print_element(array.__soul_UNSAFE_at__(lastIndex), std::is_same<K, __Soul_ARRAY__<char>>());

    std::cout << ']';
}

template<typename T>
inline void print_element(const T& element, std::true_type)
{
    Print(element);
}

template<typename T>
inline void print_element(const T& element, std::false_type)
{
    std::cout << '"';
    Print(element);
    std::cout << '"';
}

template<> inline void Print<int8_t>(int8_t msg) { std::cout << static_cast<int16_t>(msg); }
template<> inline void Print<uint8_t>(uint8_t msg) { std::cout << static_cast<uint16_t>(msg); }

inline void Println()
{
    std::cout << std::endl;
}

template<typename T, typename = void>
inline void Println(T msg)
{
    std::cout << msg << std::endl;
}

template<typename T, size_t SIZE>
inline void Println(__Soul_LITERAL_ARRAY__<T, SIZE> msg)
{
    Print(msg);
    std::cout << std::endl;
}

template<>
inline void Println(const char* msg)
{
    Print(msg);
    std::cout << std::endl;
}

template<>
inline void Println(__Soul_ARRAY__<char> msg)
{
    for(auto ch : msg) {
        std::cout << ch;
    }
    std::cout << std::endl;
}

template<typename K>
inline void Println(__Soul_ARRAY__<K> array)
{
    Print(array);
    std::cout << std::endl;
}

template<> inline void Println<int8_t>(int8_t msg) { std::cout << static_cast<int16_t>(msg) << std::endl; }
template<> inline void Println<uint8_t>(uint8_t msg) { std::cout << static_cast<uint16_t>(msg) << std::endl; }

