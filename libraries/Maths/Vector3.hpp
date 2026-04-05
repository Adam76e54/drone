#pragma once

/**
 * This is adapted from ArduPilot's Vector3 which is itself closely adpated from Bill Perone's 
 */
template <typename T>
class Vector3 {
  public:

  T x, y, z;  

  constexpr Vector3()
    : x(0)
    , y(0)
    , z(0) {}

  constexpr Vector3(const T x0, const T y0, const T z0)
    : x(x0)
    , y(y0)
    , z(z0) {}


};