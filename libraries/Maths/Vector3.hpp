#pragma once

/**
 * This is adapted from ArduPilot's Vector3 which is itself closely adpated from Bill Perone's 
 */

template <typename T>
class Vector3 {
  public:

  T x, y, z;  

  constexpr Vector3()
    : x(0), y(0), z(0) {}

  constexpr Vector3(const T x0, const T y0, const T z0)
    : x(x0), y(y0), z(z0) {}

  constexpr Vector3(const Vector3<T> &v)
    : x(v.x), y(v.y), z(v.z) {}

    // test for equality
    bool operator ==(const Vector3<T> &v) const;

    // test for inequality
    bool operator !=(const Vector3<T> &v) const;

    // negation
    Vector3<T> operator -(void) const;

    // addition
    Vector3<T> operator +(const Vector3<T> &v) const;

    // subtraction
    Vector3<T> operator -(const Vector3<T> &v) const;

    // uniform scaling
    Vector3<T> operator *(const T num) const;

    // uniform scaling
    Vector3<T> operator  /(const T num) const;

    // addition
    Vector3<T> &operator +=(const Vector3<T> &v);

    // subtraction
    Vector3<T> &operator -=(const Vector3<T> &v);

    // uniform scaling
    Vector3<T> &operator *=(const T num);

    // uniform scaling
    Vector3<T> &operator /=(const T num);

    // non-uniform scaling
    Vector3<T> &operator *=(const Vector3<T> &v);
};