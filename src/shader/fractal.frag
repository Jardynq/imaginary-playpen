#version 300 es
precision highp float;
precision highp int;


uniform float time;
uniform int max_iterations;
uniform vec2 offset;
uniform vec2 center;
uniform vec2 zoom;
uniform float aspect_ratio;
uniform float escape_radius;
uniform float fractal_count;

uniform sampler2D texture_sampler;

const int flags_mandelbrot   = 0x01;
const int flags_inverse      = 0x02;
const int flags_multi        = 0x04;
const int flags_animated     = 0x08;
const int flags_arbitrary    = 0x0c;
const int flags_color_mix    = 0x10;
uniform int flags;

// TODO
const int coloring_v1 = 0x1;
const int coloring_v2 = 0x2;
const int coloring_v3 = 0x4; //etc
uniform int coloring;




in vec2 f_uv;
out vec4 color;


const float pi = 3.1415926535897;
const float pi_05 = 1.57079632679;




vec2 cmul(vec2 a, vec2 b) {
    vec2 result;
    result.x = a.x * b.x - a.y * b.y;
    result.y = a.x * b.y + a.y * b.x;
    return result;
}
vec2 cmul(vec2 a) {
    vec2 result;
    result.x = a.x * a.x - a.y * a.y;
    result.y = a.x * a.y + a.y * a.x;
    return result;
}
vec2 cdiv(vec2 a, vec2 b) {
    vec2 result;
    result.x = (a.x * b.x + a.y * b.y) / (b.x * b.x + b.y * b.y);
    result.y = (a.y * b.x - a.x * b.y) / (b.x * b.x + b.y * b.y);
    return result;
}
vec2 csqrt(vec2 a) {
    vec2 result;
    float c = a.x * a.x + a.y * a.y;
    result.x = sqrt((a.x + sqrt(c)) / 2.0);
    result.y = (a.y / abs(a.y)) * sqrt((-a.x + sqrt(c)) / 2.0);

    return result;
}




// Increment U
uvec4 inc128(uvec4 u) {
    // Compute all carries to add
    uint hy = uint(u.y == 0xffffffffu);
    uint hz = uint(u.z == 0xffffffffu);
    uint hw = uint(u.w == 0xffffffffu);
    uvec4 c = uvec4(hy & hz & hw & 1u, hz & hw & 1u, hw & 1u, 1u);
    return u + c;
}
// Return -U
uvec4 neg128(uvec4 u) {
    // (1 + ~U) is two's complement
    return inc128(u ^ uvec4(0xffffffffu, 0xffffffffu, 0xffffffffu, 0xffffffffu)); 
}
// Return U+V
uvec4 add128(uvec4 u, uvec4 v) {
    uvec4 s = u + v;
    uvec4 h = uvec4(s.x < u.x, s.y < u.y, s.z < u.z, s.w < u.w);
    uvec4 c1 = h.yzwx & uvec4(1, 1, 1, 0); // Carry from U+V
    uint hy = uint(s.y == 0xffffffffu);
    uint hz = uint(s.z == 0xffffffffu);
    uvec4 c2 = uvec4((c1.y | (c1.z & hz)) & hy, c1.z & hz, 0, 0); // Propagated carry
    return s + c1 + c2;
}
// Return U<<1
uvec4 shl128(uvec4 u) {
    // TODO: shorten
    uvec4 h = uvec4(u.x >> 31, (u.y >> 31) & 1u, (u.z >> 31) & 1u, (u.w >> 31) & 1u); // Bits to move up
    return uvec4(u.x << 1, u.y << 1, u.z << 1, u.w << 1) | h.yzwx;
}

// Return U>>1
uvec4 shr128(uvec4 u) {
    uvec4 h = uvec4(u.x << 31, u.y << 31, u.z << 31, u.w << 31) & uvec4(0x80000000, 0x80000000, 0x80000000, 0); // Bits to move down
    return uvec4(u.x >> 1, u.y >> 1, u.z >> 1, u.w >> 1)  | h.wxyz;
}

uint mul_hi(uint a, uint b) {
    // TODO
    return (a * b) >> 16;
}
// Return U*K.
// U MUST be positive.
uvec4 mul128u(uvec4 u, uint k) {
  uvec4 s1 = u * k;
  uvec4 s2 = uvec4(mul_hi(u.y, k), mul_hi(u.z, k), mul_hi(u.w, k), 0);
  return add128(s1, s2);
}

// Return U*V truncated to keep the position of the decimal point.
// U and V MUST be positive.
uvec4 mulfpu(uvec4 u, uvec4 v) {
  // Diagonal coefficients
  uvec4 s = uvec4(u.x * v.x, mul_hi(u.y, v.y), u.y * v.y, mul_hi(u.z, v.z));
  // Off-diagonal
  uvec4 t1 = uvec4(mul_hi(u.x, v.y), u.x * v.y, mul_hi(u.x, v.w),u.x * v.w);
  uvec4 t2 = uvec4(mul_hi(v.x, u.y), v.x* u.y, mul_hi(v.x, u.w), v.x * u.w);
  s = add128(s, add128(t1, t2));
  t1 = uvec4(0, mul_hi(u.x, v.z), u.x * v.z, mul_hi(u.y, v.w));
  t2 = uvec4(0, mul_hi(v.x, u.z), v.x * u.z, mul_hi(v.y, u.w));
  s = add128(s, add128(t1, t2));
  t1 = uvec4(0, 0, mul_hi(u.y, v.z), u.y * v.z);
  t2 = uvec4(0, 0, mul_hi(v.y, u.z), v.y * u.z);
  s = add128(s, add128(t1, t2));
  // Add 3 to compensate truncation
  return add128(s, uvec4(0, 0, 0, 3));
}

// Return U*U truncated to keep the position of the decimal point.
// U MUST be positive.
uvec4 sqrfpu(uvec4 u) {
  // Diagonal coefficients
  uvec4 s = uvec4(u.x * u.x, mul_hi(u.y, u.y), u.y * u.y, mul_hi(u.z, u.z));
  // Off-diagonal
  uvec4 t = uvec4(mul_hi(u.x, u.y), u.x * u .y, mul_hi(u.x, u.w), u.x * u.w);
  s = add128(s, shl128(t));
  t = uvec4(0, mul_hi(u.x, u.z), u.x * u.z, mul_hi(u.y, u.w));
  s = add128(s, shl128(t));
  t = uvec4(0, 0, mul_hi(u.y, u.z), u.y * u.z);
  s = add128(s, shl128(t));
  // Add 3 to compensate truncation
  return add128(s, uvec4(0, 0, 0, 3));
}

/*
private static BigInteger LogBase2(BigInteger num) {
    if (num <= Zero)
        return MinusOne; //does not support negative values.
    BigInteger i = Zero;
    while (!(num >>= 1).IsZero)
        i++;
    return i;
}

struct BigInt() {

}
struct bfloat() {
    char main_sign;
    //int numerator;
    //int denomeinator;
    float mantissa;
    char exp_sign;
    int exponent;
}
bfloat bf_add(bfloat a, bfloat b) {
    bfloat result;
    int diff = a.exponent - b.exponent;
    b.exponent += diff;
    b.mantissa *= pow(10, diff);
    result.main_sign = 
}*/


float atan2(float y, float x){
  float t0, t1, t2, t3, t4;
  t3 = abs(x);
  t1 = abs(y);
  t0 = max(t3, t1);
  t1 = min(t3, t1);
  t3 = 1.0 / t0;
  t3 = t1 * t3;
  t4 = t3 * t3;
  t0 =         - 0.013480470;
  t0 = t0 * t4 + 0.057477314;
  t0 = t0 * t4 - 0.121239071;
  t0 = t0 * t4 + 0.195635925;
  t0 = t0 * t4 - 0.332994597;
  t0 = t0 * t4 + 0.999995630;
  t3 = t0 * t3;
  t3 = (abs(y) > abs(x)) ? 1.570796327 - t3 : t3;
  t3 = (x < 0.0) ?  3.141592654 - t3 : t3;
  t3 = (y < 0.0) ? -t3 : t3;
  return t3;
}

float interpolate(float a, float b, float i) {
    return a + (b - a) * i;
}



void execute_multi() {

}
float execute_default(inout int iters, inout vec2 z, in vec2 c) {
    float smooth_color = exp(-length(z));

    for (; iters < max_iterations; iters++) {
        float pow_temp = pow((z.x * z.x + z.y * z.y), (fractal_count / 2.0));
        float atan2_temp = atan2(z.y, z.x) * fractal_count;
        z.x = pow_temp * cos(atan2_temp) + c.x;
        z.y = pow_temp * sin(atan2_temp) + c.y;

        if (dot(z, z) >= escape_radius * escape_radius) {
            break;
        }
        smooth_color += exp(-length(z));
    }

    return smooth_color;
}


void main() {
    vec2 z, c;
    if ((flags & flags_mandelbrot) == 1) {
        z = center;
        c = (f_uv - 0.5) / zoom + offset;
    } else {
        z = (f_uv - 0.5) / zoom + offset;
        c = center;
    }
    float smooth_color = exp(-length(z));


    // Boundary checking.
    int iters = 0;
    if (bool(flags & flags_multi)) {
        for (; iters < max_iterations; iters++) {
            float pow_temp = pow((z.x * z.x + z.y * z.y), (fractal_count / 2.0));
            float atan2_temp = atan2(z.y, z.x) * fractal_count;
            z.x = pow_temp * cos(atan2_temp) + c.x;
            z.y = pow_temp * sin(atan2_temp) + c.y;

            if (dot(z, z) >= escape_radius * escape_radius) {
                break;
            }
            smooth_color += exp(-length(z));
        }
    } else {
        for (; iters < max_iterations; iters++) {
            z = cmul(z) + c;

            smooth_color += exp(-length(z));
            if (dot(z, z) >= escape_radius * escape_radius) {
                break;
            }
        }
    }
    

    // Coloring.
    if (coloring == coloring_v1) {
        float temp_i = float(iters);
        if (iters < max_iterations) {
            float log_zn = log(z.x * z.x + z.y * z.y) / 2.0;
            temp_i = temp_i + 1.0 - log2(log2(dot(z, z))) + 4.0;//log(log_zn / log(2.0)) / log(2.0);
        }

        float max_iters = float(max_iterations);
        float u_coord = smooth_color / max_iters;
        u_coord = bool(flags & flags_inverse) ? 1.0 - u_coord : u_coord;
        if (bool(flags & flags_color_mix)) {
            vec4 color1 = texture(texture_sampler, vec2(temp_i / max_iters, 0.0));
            vec4 color2 = texture(texture_sampler, vec2((temp_i + 1.0) / max_iters, 0.0));
            color = mix(color1, color1, mod(float(iters), 1.0));
        } else {
            color = texture(texture_sampler, vec2(temp_i / max_iters, 0.0));
        }
    } else if (coloring == coloring_v2) {
        float u_coord = smooth_color / float(max_iterations);
        u_coord = bool(flags & flags_inverse) ? 1.0 - u_coord : u_coord;

        if (bool(flags & flags_color_mix)) {
            float u_coord2 = (smooth_color - 1.0) / float(max_iterations);
            u_coord2 = bool(flags & flags_inverse) ? 1.0 - u_coord2 : u_coord2;
            vec4 color1 = texture(texture_sampler, vec2(u_coord, 0.0));
            vec4 color2 = texture(texture_sampler, vec2(u_coord2, 0.0));
            color = mix(color1, color1, mod(float(iters), 1.0));
        } else {
            color = texture(texture_sampler, vec2(u_coord, 0.0));
        }
    } else if (coloring == coloring_v3) {
        color.x = 0.6 + 0.4 * sin(smooth_color * 0.1 + time * 0.63);
        color.y = color.x * color.x;
        color.z = color.x * color.y;
    }
    color.w = 1.0;
}



