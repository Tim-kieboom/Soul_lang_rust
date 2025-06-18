#pragma once
#include "soul_copy.hpp"
#include "../soul_panic.hpp"


template<typename T>
struct __Soul_ARRAY__;
namespace __lib_soul_array_priv {
    // Primary template: for non-template types
    template<typename T>
    struct add_const_recursive {
        using type = std::add_const_t<T>;
    };

    // Specialization for __Soul_ARRAY__ types
    template<typename T>
    struct add_const_recursive<__Soul_ARRAY__<T>> {
        using type = __Soul_ARRAY__<typename add_const_recursive<T>::type> const;
    };

    template<typename T>
    using add_const_recursive_t = typename add_const_recursive<T>::type;
}

template<typename T>
class __Soul_ARRAY_Iterator__
{
private:
    T* ptr;

public:
    typedef std::forward_iterator_tag iterator_category;
    typedef T value_type;
    typedef std::ptrdiff_t difference_type;
    typedef T* pointer;
    typedef T& reference;

    constexpr __Soul_ARRAY_Iterator__(T* ptr) noexcept
        : ptr(ptr)
    {
    }

    constexpr T& operator*() noexcept { return *ptr; }
    constexpr T* operator->() noexcept { return ptr; }
    __Soul_ARRAY_Iterator__& operator++() { ++ptr; return *this; }
    __Soul_ARRAY_Iterator__ operator++(int) { __Soul_ARRAY_Iterator__ tmp = *this; ++ptr; return tmp; }
    constexpr bool operator!=(const __Soul_ARRAY_Iterator__& other) const noexcept { return ptr != other.ptr; }
    constexpr bool operator==(const __Soul_ARRAY_Iterator__& other) const noexcept { return ptr == other.ptr; }
};

template<typename T>
class __Soul_ARRAY_ConstIterator__
{
private:
    const T* ptr;

public:
    typedef std::forward_iterator_tag iterator_category;
    typedef T value_type;
    typedef std::ptrdiff_t difference_type;
    typedef const T* pointer;
    typedef const T& reference;

    constexpr __Soul_ARRAY_ConstIterator__(const T* ptr) noexcept
        : ptr(ptr)
    {
    }

    constexpr const T& operator*() const noexcept { return *ptr; }
    constexpr const T* operator->() const noexcept { return ptr; }
    __Soul_ARRAY_ConstIterator__& operator++() { ++ptr; return *this; }
    __Soul_ARRAY_ConstIterator__ operator++(int) { __Soul_ARRAY_ConstIterator__ tmp = *this; ++ptr; return tmp; }
    constexpr bool operator!=(const __Soul_ARRAY_ConstIterator__& other) const noexcept { return ptr != other.ptr; }
    constexpr bool operator==(const __Soul_ARRAY_ConstIterator__& other) const noexcept { return ptr == other.ptr; }
}; 

template <typename T>
struct __Soul_ARRAY__
{
    /// if array is span then __f_spanPtr = __f_spanPtr + offset so can not be used for delete so use this ptr
    T* __f_OGPtr = nullptr;
    /// is arrayPtr + offset
    T* __f_spanPtr = nullptr;
    size_t __f_size = 0;

    __Soul_ARRAY__(size_t size): __f_size(size) 
    {
        __f_OGPtr = __f_spanPtr = new T[__f_size];
    }

    constexpr __Soul_ARRAY__() = default;
    constexpr __Soul_ARRAY__(T* ptr, size_t size)
        : __f_OGPtr(ptr), __f_spanPtr(ptr), __f_size(size) {}

    constexpr __Soul_ARRAY__(T* OGptr, T* spanPtr, size_t size)
        : __f_OGPtr(OGptr), __f_spanPtr(spanPtr), __f_size(size) {}

    constexpr size_t __size() const noexcept
    {
        return __f_size;
    }

    constexpr size_t __offset() const noexcept
    {
        return (size_t)(__f_spanPtr - __f_OGPtr);
    }

    __Soul_ARRAY__<T> __clone() 
    {
        T* arr = new T[__f_size];
        std::copy(__f_spanPtr, __f_spanPtr + __f_size, arr);
        return __Soul_ARRAY__(arr, __f_size);
    }

    constexpr T __get(size_t index) const noexcept 
    {   
        return __f_spanPtr[index];
    }

    constexpr T const* __get_constRef(size_t index) const noexcept
    {   
        return &__f_spanPtr[index];
    }

    constexpr T* __get_mutRef(size_t index) const noexcept
    {
        return &__f_spanPtr[index];
    }

    constexpr __Soul_ARRAY__<T> __new_span(size_t start, size_t end) const noexcept 
    {
        return __Soul_ARRAY__<T>{__f_OGPtr, __f_spanPtr+start, end - start};
    }
    
    using AsConst = __lib_soul_array_priv::add_const_recursive_t<__Soul_ARRAY__<T>>;

    void __free() {
        if(__f_OGPtr != nullptr) {
            delete[] __f_OGPtr;
        }
    }

    template <
        typename U = T,
        typename NonConstT = typename std::remove_const<U>::type,
        typename std::enable_if<
            std::is_const<U>::value && !std::is_same<U, NonConstT>::value,
            int
        >::type = 0
    >
    operator __Soul_ARRAY__<NonConstT>() const {
        NonConstT* arr = new NonConstT[__f_size];
        for (size_t i = 0; i < __f_size; ++i)
            arr[i] = __f_spanPtr[i];
        return __Soul_ARRAY__<NonConstT>(arr, __f_size);
    }

    using iterator = __Soul_ARRAY_Iterator__<T>;
    using const_iterator = __Soul_ARRAY_ConstIterator__<T>;

    constexpr iterator __begin() { return iterator(__f_spanPtr); }
    constexpr iterator __end() { return iterator(__f_spanPtr + __f_size); }

    constexpr const_iterator __cbegin() const { return const_iterator(__f_spanPtr); }
    constexpr const_iterator __cend() const { return const_iterator(__f_spanPtr + __f_size); }
};

constexpr size_t __stack_array_size(void const*) { return 0; }

template<typename T, size_t N>
constexpr size_t __stack_array_size(const T (&)[N]) { return N; }


#define __Soul_ARRAY_LiteralCtor__(elType, progmem) __Soul_ARRAY__<elType>::AsConst{progmem, (progmem == nullptr) ? 0 : __stack_array_size(progmem)}





