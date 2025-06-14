#pragma once
#include "soul_LiteralArray.hpp"

using __Soul_STR__ = __Soul_ARRAY__<char>;

inline uint64_t len(__Soul_STR__ str) 
{ 
    return str.size(); 
}

__Soul_STR__ __Soul_copy__(__Soul_STR__ other)
{
    int64_t lastIndex = other.size() - other.offset();

    __Soul_STR__ copyStr(lastIndex);
    
    uint32_t i = 0;
    for(uint32_t i = 0; i < lastIndex; i++)
        copyStr.__soul_UNSAFE_at__(i) = other.__soul_UNSAFE_at__(i);

    return copyStr;
}

inline __Soul_STR__ toStr(const char* cppStr) 
{
    return __Soul_STR__((char*)cppStr, strlen(cppStr));
}

inline __Soul_STR__ toStr(std::string cppStr) 
{
    return __Soul_STR__((char*)cppStr.c_str(), cppStr.length());
}

inline __Soul_STR__ toStr(const bool value)
{
    const char* strValue = value ? "true" : "false"; 
    return toStr(strValue);
}

inline __Soul_STR__ toStr(std::stringstream& ss) 
{
    auto string = ss.str();
    return __Soul_STR__((char*)string.c_str(), string.length());
}

inline __Soul_STR__ toStr(const char value)     { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const float value)    { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const double value)   { return toStr(std::to_string(value)); } 
inline __Soul_STR__ toStr(const int16_t value)  { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const int32_t value)  { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const int64_t value)  { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const uint32_t value) { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const uint16_t value) { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const uint64_t value) { return toStr(std::to_string(value)); }
inline __Soul_STR__ toStr(const int8_t value)   { return toStr(std::to_string((int16_t)value)); }
inline __Soul_STR__ toStr(const uint8_t value)  { return toStr(std::to_string((uint16_t)value)); }

std::ostream& operator<<(std::ostream& os, const __Soul_STR__& str) 
{
    for(char ch : str) {
        os << ch;
    }
    return os;
}

template<size_t SIZE>
std::ostream& operator<<(std::ostream& os, const __Soul_LITERAL_ARRAY__<char, SIZE>& str) 
{
    for(char ch : str) {
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

template<size_t SIZE>
void __soul__append_to_stream__(std::stringstream& ss, const __Soul_LITERAL_ARRAY__<char, SIZE>&& arg)
{
    ss << arg.__to_c_str__();
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
    return toStr(ss);
}

inline const char* __Copy_To_C_Str__(__Soul_STR__& string) {
    char* buffer = new char[string.size()+1];
    
    int i = 0;
    for(auto el : string) {
        buffer[i++] = el;
    }
    buffer[i] = '\0';
    return buffer;
}

inline double __Parse_f64__(__Soul_STR__& string) { return std::stod(__Copy_To_C_Str__(string)); }
inline int64_t __Parse_i64__(__Soul_STR__& string) { return std::stoll(__Copy_To_C_Str__(string)); }
inline uint64_t __Parse_u64__(__Soul_STR__& string) { return std::stoull(__Copy_To_C_Str__(string)); }

inline double __Parse_f64__(const char* string) { return std::stod(string); }
inline int64_t __Parse_i64__(const char* string) { return std::stoll(string); }
inline uint64_t __Parse_u64__(const char* string) { return std::stoull(string); }


