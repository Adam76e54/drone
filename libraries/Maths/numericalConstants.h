#pragma once

/**
 * A lot of this is pulled from ArduPilot's AP_Math library but it's mostly just natural constants.  
 */

inline constexpr float PI = 3.14159265358979323846f;
inline constexpr float DEG_TO_RAD = PI / 180.0f;
inline constexpr float RAD_TO_DEG = 180.0f / PI;
inline constexpr float TAU = 2 * PI;
inline constexpr float HALF_PI = PI / 2;
inline constexpr float SQRT_2 = 1.41421356237309504880f;
inline constexpr float GRAVITY = 9.80665f;
