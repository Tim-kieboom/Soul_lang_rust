#pragma once
#include "soul_array.hpp"
#include "../soul_stdmem.hpp"
#include "../soul_type_traits.hpp"

using __Soul_STR__ = __Soul_ARRAY__<char>;
using __Soul_LITERAL_STR__ = __Soul_ARRAY__<char>::AsConst;

using str = __Type<__Soul_ARRAY__<char>>;

template<typename K>
constexpr bool __is_Soul_String_StringRef =
    std::is_same<K, __Soul_STR__>::value ||
    std::is_same<K, __Soul_STR__*>::value ||
    std::is_same<K, __Soul_STR__* const>::value ||
    
    std::is_same<K, __Soul_LITERAL_STR__>::value ||
    std::is_same<K, __Soul_LITERAL_STR__*>::value ||
    std::is_same<K, __Soul_LITERAL_STR__* const>::value;

using __Soul_cpp_string = std::string;

inline __Soul_STR__ str(const char* cppStr) 
{
    return __Soul_STR__((char*)cppStr, strlen(cppStr));
}

inline __Soul_STR__ str(__Soul_cpp_string cppStr) 
{
    char* c_str = new char[cppStr.size()];
    memcpy(c_str, cppStr.c_str(), cppStr.size());
    return __Soul_STR__(c_str, cppStr.size());
}

inline __Soul_STR__ str(std::stringstream& ss) 
{
    auto string = ss.str();
    return __Soul_STR__((char*)string.c_str(), string.length());
}

inline __Soul_STR__ str()                  { return __Soul_ARRAY__<char>(); }
template<typename T, typename std::enable_if<__is_Soul_PrimitiveT<T>, int>::type = 0>
inline __Soul_STR__ str(const T value)     { __Soul_cpp_string out; __soul_str_parse__(value, out); return str(out); }


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


template<typename T, typename std::enable_if<__is_Soul_IntT<T>, int>::type = 0>
inline void ___soul_primitive_parse__(const T& msg, __Soul_cpp_string& out) {
    char buf[32];
    std::snprintf(buf, sizeof(buf), "%lld", static_cast<long long>(msg));
    out += buf;
}

template<typename T, typename std::enable_if<__is_Soul_UIntT<T>, int>::type = 0>
inline void ___soul_primitive_parse__(const T& msg, __Soul_cpp_string& out) {
    char buf[32];
    std::snprintf(buf, sizeof(buf), "%llu", static_cast<long long>(msg));
    out += buf;
}

template<typename T, typename std::enable_if<__is_Soul_FloatT<T>, int>::type = 0>
inline void ___soul_primitive_parse__(const T& msg, __Soul_cpp_string& out) {
    char buf[32];
    std::snprintf(buf, sizeof(buf), "%g", msg);
    out += buf;
}

inline void ___soul_primitive_parse__(const bool& msg, __Soul_cpp_string& out) {
    out += msg ? __Soul_cpp_string("true") : __Soul_cpp_string("false");
}

inline void ___soul_primitive_parse__(const char& msg, __Soul_cpp_string& out) {
    out += __Soul_cpp_string(1, msg);
}

inline void __soul_char_array_parse__(const __Soul_ARRAY__<char>* const msg, __Soul_cpp_string& out) 
{ 
    for(auto __it = msg->__cbegin(); __it != msg->__cend(); ++__it) 
    { 
        out.push_back(*__it); 
    } 
}

inline void __soul_char_array_parse__(const __Soul_ARRAY__<char>::AsConst* const msg, __Soul_cpp_string& out) 
{ 
    for(auto __it = msg->__cbegin(); __it != msg->__cend(); ++__it) 
    { 
        out.push_back(*__it); 
    } 
}

template<typename T>
inline typename std::enable_if<!__is_Soul_String_StringRef<T>, void>::type
__soul_array_element_parse__(const T& element, __Soul_cpp_string& out);
template<typename T>
inline typename std::enable_if<__is_Soul_String_StringRef<T>, void>::type
__soul_array_element_parse__(const T& element, __Soul_cpp_string& out);

template<typename K>
void __soul_array_parse__(const __Soul_ARRAY__<K>& array, __Soul_cpp_string& out)
{

    if(array.__size() == 0)
    {
        out += "[empty]";
        return;
    }

    out.push_back('[');

    int64_t lastIndex = array.__size() - 1;
    for (int64_t i = 0; i < lastIndex; i++)
    {
        __soul_array_element_parse__(array[i], out);
        out += ", ";
    }

    if(lastIndex >= 0)
        __soul_array_element_parse__(array[lastIndex], out);

    out.push_back(']');
}

template<typename T, typename std::enable_if<__is_Soul_PrimitiveT<T>, int>::type = 0>
inline void __soul_str_parse__(const T& msg, __Soul_cpp_string& out)                                 { ___soul_primitive_parse__(msg, out); }
template<typename T, typename std::enable_if<__is_Soul_PrimitiveT<T>, int>::type = 0>
inline void __soul_str_parse__(T* msg, __Soul_cpp_string& out)                                       { ___soul_primitive_parse__(*msg, out); }
template<typename T, typename std::enable_if<__is_Soul_PrimitiveT<T>, int>::type = 0>
inline void __soul_str_parse__(const T* const msg, __Soul_cpp_string& out)                           { ___soul_primitive_parse__(*msg, out); }

template<typename T>
inline void __soul_str_parse__(const __Soul_ARRAY__<T>& msg, __Soul_cpp_string& out)                 { __soul_array_parse__(msg, out); }
template<typename T>
inline void __soul_str_parse__(const __Soul_ARRAY__<T>&& msg, __Soul_cpp_string& out)                { __soul_array_parse__(msg, out); }

template<typename T>
inline void __soul_str_parse__(const __Soul_ARRAY__<T>* const msg, __Soul_cpp_string& out)           { __soul_array_parse__(*msg, out); }

inline void __soul_str_parse__(const __Soul_ARRAY__<char>& msg, __Soul_cpp_string& out)                 { __soul_char_array_parse__((__Soul_ARRAY__<char>::AsConst*)&msg, out); }
inline void __soul_str_parse__(const __Soul_ARRAY__<char>&& msg, __Soul_cpp_string& out)                { __soul_char_array_parse__((__Soul_ARRAY__<char>::AsConst*)&msg, out); }
inline void __soul_str_parse__(const __Soul_ARRAY__<char>::AsConst& msg,  __Soul_cpp_string& out)       { __soul_char_array_parse__(&msg, out); }
inline void __soul_str_parse__(const __Soul_ARRAY__<char>::AsConst&& msg,  __Soul_cpp_string& out)      { __soul_char_array_parse__(&msg, out); }
inline void __soul_str_parse__(const __Soul_ARRAY__<char>* const msg, __Soul_cpp_string& out)           { __soul_char_array_parse__(msg, out); }
inline void __soul_str_parse__(const __Soul_ARRAY__<char>::AsConst* const msg, __Soul_cpp_string& out)  { __soul_char_array_parse__(msg, out); }

inline void __soul_str_parse__(char msg, __Soul_cpp_string& out)                                     { out.push_back(msg); }
inline void __soul_str_parse__(int8_t msg, __Soul_cpp_string& out)                                   { out += std::to_string(static_cast<int16_t>(msg)); }
inline void __soul_str_parse__(uint8_t msg, __Soul_cpp_string& out)                                  { out += std::to_string(static_cast<uint16_t>(msg)); }

template<typename T>
inline typename std::enable_if<!__is_Soul_String_StringRef<T>, void>::type
__soul_array_element_parse__(const T& element, __Soul_cpp_string& out)
{
    __soul_str_parse__(element, out);
}

template<typename T>
inline typename std::enable_if<__is_Soul_String_StringRef<T>, void>::type
__soul_array_element_parse__(const T& element, __Soul_cpp_string& out)
{
    out.push_back('\"');
    __soul_str_parse__(element, out);
    out.push_back('\"');
}

template <typename ...Args>
inline __Soul_STR__ __soul_format_string__(Args&&... args)
{
    __Soul_cpp_string ss = "";

    (__soul_str_parse__(std::forward<Args>(args), ss), ...);

    return str(ss);
}



















































