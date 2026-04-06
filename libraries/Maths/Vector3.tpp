#include "Vector3.hpp"

/**
 * This is adapted from ArduPilot's Vector3 which is itself closely adpated from Bill Perone's 
 */

#pragma GCC optimize ("O2");

template <typename T>
bool Vector3<T>::operator ==(const Vector3<T> &v) const {
    return (x == v.x) && (y == v.y) && (z == v.z);
}

template <typename T>
bool Vector3<T>::operator !=(const Vector3<T> &v) const {
    return (x != v.x) || (y != v.y) || (z != v.z);
}

template <typename T>
Vector3<T> Vector3<T>::operator -(void) const {
    return Vector3<T>(-x, -y, -z);
}

template <typename T>
Vector3<T> Vector3<T>::operator +(const Vector3<T> &v) const {
    return Vector3<T>(x + v.x, y + v.y, z + v.z);
}

template <typename T>
Vector3<T> Vector3<T>::operator -(const Vector3<T> &v) const {
    return Vector3<T>(x - v.x, y - v.y, z - v.z);
}

template <typename T>
Vector3<T> Vector3<T>::operator *(const T num) const {
    return Vector3<T>(x * num, y * num, z * num);
}

template <typename T>
Vector3<T> Vector3<T>::operator /(const T num) const {
    return Vector3<T>(x / num, y / num, z / num);
}

template <typename T>
Vector3<T> &Vector3<T>::operator +=(const Vector3<T> &v) {
    x += v.x; y += v.y; z += v.z;
    return *this;
}

template <typename T>
Vector3<T> &Vector3<T>::operator -=(const Vector3<T> &v) {
    x -= v.x; y -= v.y; z -= v.z;
    return *this;
}

template <typename T>
Vector3<T> &Vector3<T>::operator *=(const T num) {
    x *= num; y *= num; z *= num;
    return *this;
}

template <typename T>
Vector3<T> &Vector3<T>::operator /=(const T num) {
    x /= num; y /= num; z /= num;
    return *this;
}

template <typename T>
Vector3<T> &Vector3<T>::operator *=(const Vector3<T> &v) {
    x *= v.x; y *= v.y; z *= v.z;
    return *this;
}

