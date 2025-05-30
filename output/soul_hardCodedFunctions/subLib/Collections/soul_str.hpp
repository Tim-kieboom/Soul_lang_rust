#pragma once 
#include "soul_LiteralArray.hpp"

class __Soul_STR__
{
private:
    std::shared_ptr<char> rawStr;
    uint32_t offset_ = 0;
    uint32_t size_ = 0;

public:

    __Soul_STR__() = default;
    ~__Soul_STR__() = default;

    __Soul_STR__(const char* rawStr)
        : rawStr(), offset_(0), size_(strlen(rawStr))
    {
        this->rawStr = std::shared_ptr<char>(new char[size_], __Soul_ARRAY_deleter__<char>);

        for(uint32_t i = 0; i < size_; i++)
            this->rawStr.get()[i] = rawStr[i];
    }

    __Soul_STR__(std::string&& rawStr)
        : rawStr(), offset_(0), size_(rawStr.size())
    {
        this->rawStr = std::shared_ptr<char>(new char[size_], __Soul_ARRAY_deleter__<char>);

        for(uint32_t i = 0; i < size_; i++)
            this->rawStr.get()[i] = rawStr[i];
    }

    __Soul_STR__(uint32_t size_)
        : rawStr(std::shared_ptr<char>(new char[size_], __Soul_ARRAY_deleter__<char>)), offset_(0), size_(size_)
    {
    }

    __Soul_STR__(__Soul_STR__* other, uint32_t start, uint32_t end)
        : rawStr(other->rawStr), offset_(start), size_(end)
    {
    }

    char& operator[](int64_t index) noexcept
    {
        if (index < 0)
            index = index + size_;

        index += offset_;

        if (index >= size_)
        {
            printf("!!ERROR!! index out of range\n");
            exit(1);
        }

        return (rawStr.get())[index];
    }

    __Soul_STR__ operator+(__Soul_STR__& other) noexcept
    { 
        int64_t thisSize = (this->size_ - this->offset_);
        int64_t otherSize = (other.size_ - other.offset_);
        int64_t newSize = thisSize + otherSize;
        if(newSize <= 0)
            return __Soul_STR__();
        
        __Soul_STR__ newStr(newSize);
        for(int64_t i = 0; i < thisSize; i++)
            newStr.__soul_UNSAFE_at__(i) = this->__soul_UNSAFE_at__(i);

        for(int64_t i = 0; i < otherSize; i++)
            newStr.__soul_UNSAFE_at__(i + thisSize) = other.__soul_UNSAFE_at__(i);

        return newStr;
    }

    bool operator==(__Soul_STR__& other) noexcept
    {
        if((int64_t)(this->size_) - this->offset_ != (int64_t)(other.size_) - other.offset_)
            return false;

        for(uint32_t i = 0; i < this->size_; i++)
        {    
            if(this->__soul_UNSAFE_at__(i) != other.__soul_UNSAFE_at__(i))
                return false;
        }

        return true;
    }

    char& __soul_UNSAFE_at__(uint32_t index) noexcept
    {
        return (rawStr.get())[index + offset_];
    }

    __Soul_STR__ __soul_makeSpan_fail__(__Soul_Range__&& range) noexcept
    {
        switch(range.type)
        {
            case __Soul_Range__::RangeType::START:
                return __soul_makeSpan_fail_start__(range.start);

            case __Soul_Range__::RangeType::END:
                return __soul_makeSpan_fail_end__(range.end);

            default:
            case __Soul_Range__::RangeType::START_END:
                return __soul_makeSpan_fail_start_end__(range);
        };
    }

    uint32_t size() const
    {
        return size_;
    }

    uint32_t offset() const
    {
        return offset_;
    }

    char* __Soul_getC_Str__() noexcept
    {
        return rawStr.get(); 
    }

    const char* __Copy_To_C_Str__() const noexcept
    {
        const size_t size = (size_ - offset_)+1;
        char* buffer = new char[size];

        for(uint32_t i = 0; i < size_; i++)
            buffer[i] = rawStr.get()[i];
        
        buffer[size-1] = '\0';
        return buffer;
    }

    __Soul_ARRAY_Iterator__<char> begin()
    {
        return __Soul_ARRAY_Iterator__<char>((char*)rawStr.get() + offset_);
    }

    __Soul_ARRAY_Iterator__<char> end()
    {
        return __Soul_ARRAY_Iterator__<char>((char*)rawStr.get() + size_);
    }

    __Soul_ARRAY_ConstIterator__<char> begin() const
    {
        return __Soul_ARRAY_ConstIterator__<char>(rawStr.get() + offset_);
    }

    __Soul_ARRAY_ConstIterator__<char> end() const
    {
        return __Soul_ARRAY_ConstIterator__<char>(rawStr.get() + size_);
    }

private:
        __Soul_STR__ __soul_makeSpan_fail_start_end__(__Soul_Range__& range) noexcept
    {
        auto& start = range.start;
        auto& end = range.end;

        if (start < 0)
            start = start + size_;

        if (end < 0)
            end = end + size_;

        start += offset_;
        end += offset_ + 1;

        if (end < start)
        {
            printf("!!ERROR!! (start < end) while making stringSpan\n");
            exit(1);
        }

        return __Soul_STR__(this, start, end);
    }

    __Soul_STR__ __soul_makeSpan_fail_start__(int64_t start) noexcept
    {
        if (start < 0)
            start = start + size_;

        start += offset_;

        if (size_ < start)
        {
            printf("!!ERROR!! (start < end) while making stringSpan\n");
            exit(1);
        }

        return __Soul_STR__(this, start, size_);
    }

    __Soul_STR__ __soul_makeSpan_fail_end__(int64_t end) noexcept
    {

        if (end < 0)
            end = end + size_;

        end += offset_ + 1;

        if (end < offset_)
        {
            printf("!!ERROR!! (start < end) while making stringSpan\n");
            exit(1);
        }

        return __Soul_STR__(this, offset_, end);
    }

};

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
    return __Soul_STR__(cppStr);
}

inline __Soul_STR__ toStr(std::string cppStr) 
{
    return __Soul_STR__(cppStr.c_str());
}

inline __Soul_STR__ toStr(const bool value)
{
    const char* strValue = value ? "true" : "false"; 
    return toStr(strValue);
}

inline __Soul_STR__ toStr(std::stringstream& ss) 
{
    return __Soul_STR__(ss.str());
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
    return toStr(ss);
}

inline double __Parse_f64__(__Soul_STR__& string) { return std::stod(string.__Copy_To_C_Str__()); }
inline int64_t __Parse_i64__(__Soul_STR__& string) { return std::stoll(string.__Copy_To_C_Str__()); }
inline uint64_t __Parse_u64__(__Soul_STR__& string) { return std::stoull(string.__Copy_To_C_Str__()); }

inline double __Parse_f64__(const char* string) { return std::stod(string); }
inline int64_t __Parse_i64__(const char* string) { return std::stoll(string); }
inline uint64_t __Parse_u64__(const char* string) { return std::stoull(string); }