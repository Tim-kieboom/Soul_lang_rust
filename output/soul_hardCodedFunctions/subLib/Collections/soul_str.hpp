#pragma once
#include "soul_array.hpp"

using __Soul_STR__ = __Soul_ARRAY__<char>;
using __Soul_LITERAL_STR__ = __Soul_ARRAY__<const char>;

inline __Soul_STR__ str(const char* cppStr) 
{
    return __Soul_STR__((char*)cppStr, strlen(cppStr));
}

inline __Soul_STR__ str(std::string cppStr) 
{
    char* c_str = new char[cppStr.size()];
    memcpy(c_str, cppStr.c_str(), cppStr.size());
    return __Soul_STR__(c_str, cppStr.size());
}

inline __Soul_STR__ str(const bool value)
{
    const char* strValue = value ? "true" : "false"; 
    return str(strValue);
}

inline __Soul_STR__ str(std::stringstream& ss) 
{
    auto string = ss.str();
    return __Soul_STR__((char*)string.c_str(), string.length());
}

inline __Soul_STR__ str()                     { return __Soul_ARRAY__<char>(); }
inline __Soul_STR__ str(const char value)     { return str(std::to_string(value)); }
inline __Soul_STR__ str(const float value)    { return str(std::to_string(value)); }
inline __Soul_STR__ str(const double value)   { return str(std::to_string(value)); } 
inline __Soul_STR__ str(const int16_t value)  { return str(std::to_string(value)); }
inline __Soul_STR__ str(const int32_t value)  { return str(std::to_string(value)); }
inline __Soul_STR__ str(const int64_t value)  { return str(std::to_string(value)); }
inline __Soul_STR__ str(const uint32_t value) { return str(std::to_string(value)); }
inline __Soul_STR__ str(const uint16_t value) { return str(std::to_string(value)); }
inline __Soul_STR__ str(const uint64_t value) { return str(std::to_string(value)); }
inline __Soul_STR__ str(const int8_t value)   { return str(std::to_string((int16_t)value)); }
inline __Soul_STR__ str(const uint8_t value)  { return str(std::to_string((uint16_t)value)); }

std::ostream& operator<<(std::ostream& os, const __Soul_STR__& string) 
{
    for(auto __it = string.__cbegin(); __it != string.__cend(); ++__it) {
        char ch = *__it;
        os << ch;
    }
    return os;
}

std::ostream& operator<<(std::ostream& os, const __Soul_LITERAL_STR__& string) 
{
    for(auto __it = string.__cbegin(); __it != string.__cend(); ++__it) {
        char ch = *__it;
        os << ch;
    }
    return os;
}

template <typename T>
void __soul__append_to_stream__(std::stringstream& ss, T&& arg)
{
    ss << std::forward<T>(arg);
}

template<>
void __soul__append_to_stream__(std::stringstream& ss, const __Soul_STR__&& arg)
{
    ss << arg;
}

template <typename T, typename... Args>
void __soul__append_to_stream__(std::stringstream& ss, T&& arg, Args&&... args)
{
    ss << std::forward<T>(arg);
    __soul__append_to_stream__(ss, std::forward<Args>(args)...);
}

template <typename ...Args>
inline __Soul_STR__ __soul_format_string__(Args&&... args)
{
    std::stringstream ss;
    __soul__append_to_stream__(ss, std::forward<Args>(args)...);
    return str(ss);
}

inline const char* __Copy_To_C_Str__(__Soul_STR__ const* string) {
    char* buffer = new char[string->__size()+1];
    
    int i = 0;
    for(auto __it = string->__cbegin(); __it != string->__cend(); ++__it) {
        auto el = *__it;
        buffer[i++] = el;
    }
    buffer[i] = '\0';
    return buffer;
}

inline double __Parse_f64__(__Soul_STR__ const* string) { return std::stod(__Copy_To_C_Str__(string)); }
inline int64_t __Parse_i64__(__Soul_STR__ const* string) { return std::stoll(__Copy_To_C_Str__(string)); }
inline uint64_t __Parse_u64__(__Soul_STR__ const* string) { return std::stoull(__Copy_To_C_Str__(string)); }


