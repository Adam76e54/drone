#pragma once

#include <limits>
#include <cmath>

// Check if two floats are equal within a certain tolerance.
template <typename T>
requires std::is_same_v<T, float>
inline bool equal(T a, T b, T epsilon = std::numeric_limits<T>::epsilon()) {
    return std::fabs(a - b) < epsilon;
}

