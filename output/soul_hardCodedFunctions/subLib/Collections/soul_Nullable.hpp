#pragma once
#include "../soul_cppLibs.h"

template <typename T>
class __Soul_Nullable__
{
private:
    bool has_value_;
    alignas(T) unsigned char storage_[sizeof(T)];

    constexpr void clearStorage()
    {
        if (has_value_)
        {
            reinterpret_cast<T*>(storage_)->~T();
            has_value_ = false;
        }
    }

public:
    ~__Soul_Nullable__()
    {
        clearStorage();
    }

    // Constructor empty
    constexpr __Soul_Nullable__() noexcept : has_value_(false) {}

    // Constructor from value
    constexpr __Soul_Nullable__(const T& value) : has_value_(true)
    {
        new (storage_) T(value);
    }

    // Move constructor
    constexpr __Soul_Nullable__(T&& value) : has_value_(true)
    {
        new (storage_) T(std::move(value));
    }

    // Copy constructor
    constexpr __Soul_Nullable__(const __Soul_Nullable__& other) : has_value_(other.has_value_)
    {
        if (has_value_)
        {
            new (storage_) T(*other);
        }
    }

    // Assignment operators
    constexpr __Soul_Nullable__& operator=(const __Soul_Nullable__& other)
    {
        if (this != &other)
        {
            clearStorage();
            has_value_ = other.has_value_;
            if (has_value_)
            {
                new (storage_) T(*other);
            }
        }
        return *this;
    }

    constexpr __Soul_Nullable__& operator=(__Soul_Nullable__&& other) noexcept
    {
        if (this != &other)
        {
            clearStorage();
            has_value_ = other.has_value_;
            if (has_value_) {
                new (storage_) T(std::move(*other));
                other.clearStorage();
            }
        }
        return *this;
    }

    constexpr bool has_value() const noexcept
    {
        return has_value_;
    }

    // Access the contained value
    constexpr T& value()&
    {
        if (!has_value_)
            throw std::runtime_error("Optional object has no value");

        return *reinterpret_cast<T*>(storage_);
    }

    constexpr const T& value() const&
    {
        if (!has_value_)
            throw std::runtime_error("Optional object has no value");

        return *reinterpret_cast<const T*>(storage_);
    }

    constexpr T&& value()&&
    {
        if (!has_value_)
            throw std::runtime_error("Optional object has no value");

        return std::move(*reinterpret_cast<T*>(storage_));
    }

    // Access the contained value or return a default
    constexpr T value_or(T&& default_value) const&
    {
        return has_value_ ? **this : static_cast<T>(std::forward<T>(default_value));
    }

    constexpr T value_or(T&& default_value)&&
    {
        return has_value_ ? std::move(**this) : static_cast<T>(std::forward<T>(default_value));
    }

    constexpr T* operator->()
    {
        return &value();
    }

    constexpr const T* operator->() const
    {
        return &value();
    }

    constexpr T& operator*()&
    {
        return value();
    }

    constexpr const T& operator*() const&
    {
        return value();
    }

    constexpr T&& operator*()&&
    {
        return std::move(value());
    }

    // Boolean conversion
    constexpr explicit operator bool() const noexcept
    {
        return has_value_;
    }
};