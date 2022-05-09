#version 300 es

#ifdef GL_FRAGMENT_PRECISION_HIGH
precision highp float;
precision highp int;
#else
precision mediump float;
precision mediump int;
#endif




// -----------------------------------------------------------------
// Global state
//

uniform float time;
uniform vec2 view_offset;
uniform vec2 offset;
uniform vec2 z_size;
uniform vec2 w_size;
uniform vec2 zoom;
uniform sampler2D tex_sampler;


in vec2 f_uv;
in vec3 f_vertex;
out vec4 color;


const float epsilon = 0.0001;
const float pi = 3.141592653589793;
const float phi = 1.61803398874989484820459;  
const float e = 2.71828182845904523536;  



// -----------------------------------------------------------------
// Hashing
//

float seed;
float random() {
    float result = fract(sin(seed / 100.0 * dot(gl_FragCoord.xy, vec2(12.9898, 78.233))) * 43758.5453f);
    seed += time * 0.001f;
    return result;
}
float random(float min, float max){
    return (max - min) * random() + min;
}
float gold_noise(float seed){
    return fract(tan(distance(gl_FragCoord.xy * phi, gl_FragCoord.xy) * seed) * gl_FragCoord.x);
}
vec2 crand(vec2 seed) {
    return vec2(gold_noise(seed.x), gold_noise(seed.y));
}



// -----------------------------------------------------------------
// Utils
//

bool fequal(float a, float b, float epsilon) {
    return (a < b + epsilon && a > b - epsilon) || (a == b);
}
bool fequal(float a, float b) {
    return fequal(a, b, epsilon);
}

// For some reason bitwise does not work with uint.
int hi16(int value) {
    return (value >> 16) & 0x0000ffff;
}
int lo16(int value) {
    return value & 0x0000ffff;
}

bool flag(int value, int flag) {
    return (value & flag) > 0;
} 



// -----------------------------------------------------------------
// Math extensions
//

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
float log_hypot(float x, float y) {
    float a = abs(x);
    float b = abs(y);
    if (a == 0.0) {
        return log(b);
    }
    if (b == 0.0) {
        return log(a);
    }
    if (a < 3000.0 && b < 3000.0) {
        return log(a * a + b * b) * 0.5;
    }
    return log(cos(a / atan2(b, a)));
}




// -----------------------------------------------------------------
// Complex number operations
//

bool is_real(vec2 value) {
    return value.x != 0.0 && value.y == 0.0;
}
bool is_imaginary(vec2 value) {
    return value.x == 0.0 && value.y != 0.0;
}
bool is_zero(vec2 value) {
    return value.x == 0.0 && value.y == 0.0;
}

float cabs(vec2 value) {
    return length(value);
}
float carg(vec2 value) {
    return atan2(value.x, value.y);
}

vec2 cmul(vec2 a, vec2 b) {
    vec2 result;
    result.x = a.x * b.x - a.y * b.y;
    result.y = a.x * b.y + a.y * b.x;
    return result;
}
vec2 cdiv(vec2 a, vec2 b) {
    vec2 result;
    result.x = (a.x * b.x + a.y * b.y) / (b.x * b.x + b.y * b.y);
    result.y = (a.y * b.x - a.x * b.y) / (b.x * b.x + b.y * b.y);
    return result;
}

vec2 csqrt(vec2 a) {
    if (is_real(a)) {
        return vec2(sqrt(a.x), 0.0);
    }

    vec2 result;
    float c = length(a);
    result.x = sqrt((a.x + c) * 0.5);
    result.y = sqrt((-a.x + c) * 0.5) * (a.y / abs(a.y));
    return result;
}
vec2 cpow(vec2 value, vec2 exponent) {
    if (is_zero(value) && exponent.x > 0.0 && exponent.y >= 0.0) {
        return vec2(0.0, 0.0);
    }
    if (is_zero(exponent)) {
        return vec2(1.0, 0.0);
    }
    if (is_real(exponent)) {
        if (is_real(value)) {
            return vec2(pow(value.x, exponent.x), 0.0);
        }
    }

    float arg = atan2(value.y, value.x);
    float loh = log_hypot(value.x, value.y);

    float a = exp(exponent.x * loh - exponent.y * arg);
    float b = exponent.y * loh + exponent.x * arg;
    return vec2(a * cos(b), a * sin(b));
}
vec2 cexp(vec2 value) {
    float e = exp(value.x);
    return vec2(cos(value.y) * e, sin(value.y) * e);
}
vec2 clog(vec2 value) {
    return vec2(log_hypot(value.x, value.y), atan2(value.y, value.x));
}

vec2 csin(vec2 value) {
    return vec2(sin(value.x) * cosh(value.y), cos(value.x) * sinh(value.y));
}
vec2 ccos(vec2 value) {
    return vec2(cos(value.x) * cosh(value.y), -sin(value.x) * sinh(value.y));
}
vec2 ctan(vec2 value) {
    float re = 2.0 * value.x;
    float im = 2.0 * value.y;
    float det = cos(re) + cosh(im);
    return vec2(sin(re) / det, sinh(im) / det);
}











// TODO?
const int NodeSyntaxIgnore         = -1;

const int NodeSyntaxReal           = 0;
const int NodeSyntaxImaginary      = 1;
const int NodeSyntaxVariable       = 2;
const int NodeSyntaxFunction       = 3;
const int NodeSyntaxParenthesis    = 4;
const int NodeSyntaxAddition       = 5;
const int NodeSyntaxSubtraction    = 6;
const int NodeSyntaxMultiplication = 7;
const int NodeSyntaxDivision       = 8;
const int NodeSyntaxExponent       = 9;
const int NodeSyntaxAbsolute       = 10;

// Vars
const int NodeVariableTime  = 0;

const int NodeVariableZ     = 1;
const int NodeVariableZr    = 2;
const int NodeVariableZi    = 3;

const int NodeVariableUv    = 4;
const int NodeVariableUvx   = 5;
const int NodeVariableUvy   = 6;

const int NodeVariableZoom  = 7;
const int NodeVariableZoomx = 8;
const int NodeVariableZoomy = 9;

const int NodeVariableOffset    = 10;
const int NodeVariableOffsetx   = 11;
const int NodeVariableOffsety   = 12;

const int NodeVariableE     = 13;
const int NodeVariablePi    = 14;
const int NodeVariablePhi   = 15;

// Funcs
const int NodeFunctionSin   = 0;
const int NodeFunctionCos   = 1;
const int NodeFunctionTan   = 2;
const int NodeFunctionSqrt  = 3;
const int NodeFunctionLog   = 4;
const int NodeFunctionExp   = 5;
const int NodeFunctionAbs   = 6;
const int NodeFunctionArg   = 7;
const int NodeFunctionRound = 8;
const int NodeFunctionCeil  = 9;
const int NodeFunctionFloor = 10;
const int NodeFunctionRand  = 11;


// 16 bytes total size per node
const int max_nodes = 50;
struct NodeData {
    // 16 bit hi: right_index
    // 16 bit lo: left_index
    uint indices;

    // 16 bit hi: keyword
    // 16 bit lo: syntax
    uint identifier;

    vec2 value;
};

uniform int nodes_count;
layout(std140) uniform NodesBlock {
    NodeData nodes[max_nodes];
};


vec2 evaluate_tree_variable(int variable) {
    switch (variable) {
        case NodeVariableTime:
            return vec2(time, 0.0);
            break;

        case NodeVariableZ:
            return (f_uv - 0.5) / zoom + view_offset;
            break;
        case NodeVariableZr:
            return vec2((f_uv.x - 0.5) / zoom.x + view_offset.x, 0.0);
            break;
        case NodeVariableZi:
            return vec2(0.0, (f_uv.y - 0.5) / zoom.y + view_offset.y);
            break;

        case NodeVariableUv:
            return f_uv;
            break;
        case NodeVariableUvx:
            return vec2(f_uv.x, 0.0);
            break;
        case NodeVariableUvy:
            return vec2(0.0, f_uv.y);
            break;

        case NodeVariableZoom:
            return zoom;
            break;
        case NodeVariableZoomx:
            return vec2(zoom.x, 0.0);
            break;
        case NodeVariableZoomy:
            return vec2(0.0, zoom.y);
            break;

        case NodeVariableOffset:
            return view_offset;
            break;
        case NodeVariableOffsetx:
            return vec2(view_offset.x, 0.0);
            break;
        case NodeVariableOffsety:
            return vec2(0.0, view_offset.y);
            break;

        case NodeVariableE:
            return vec2(e, 0.0);
            break;
        case NodeVariablePi:
            return vec2(pi, 0.0);
            break;
        case NodeVariablePhi:
            return vec2(phi, 0.0);
            break;
    }
}
vec2 evaluate_tree_function(int function, vec2 value) {
    switch (function) {
        case NodeFunctionSin:
            return csin(value);
            break;
        case NodeFunctionCos:
            return ccos(value);
            break;
        case NodeFunctionTan:
            return ctan(value);
            break;

        case NodeFunctionSqrt:
            return csqrt(value);
            break;
        case NodeFunctionExp:
            return cexp(value);
            break;
        case NodeFunctionLog:
            return clog(value);
            break;
        case NodeFunctionAbs:
            return vec2(cabs(value), 0.0);
            break;
        case NodeFunctionArg:
            return vec2(carg(value), 0.0);
            break;

        case NodeFunctionRound:
            return round(value);
            break;
        case NodeFunctionCeil:
            return ceil(value);
            break;
        case NodeFunctionFloor:
            return floor(value);
            break;

        case NodeFunctionRand:
            return crand(value);
            break;
    }
}

NodeData nodes_buffer[max_nodes];
vec2 evaluate_tree() {
    for (int index = 0; index < nodes_count; index++) {
        nodes_buffer[index] = nodes[index];
    }

    // For some reason index has to be an int.
    // If it's an uint, then this loop will never exit.
    // Also uint(0xffff0000) does not equal what you think
    // so instead of (x & 0xffff0000) >> 16 do (x >> 16) & 0x0000ffff
    for(int index = nodes_count - 1; index >= 0; index--) {
        int left = lo16(int(nodes_buffer[index].indices));
        int right = hi16(int(nodes_buffer[index].indices));

        // The compiler can't optimize this because of the branching.
        // So the performance is ass, even if it only runs once.
        int id = int(nodes_buffer[index].identifier);
        switch(lo16(id)) {
            case NodeSyntaxReal: break;
            case NodeSyntaxImaginary: break;

            case NodeSyntaxVariable: 
                nodes_buffer[index].value = evaluate_tree_variable(hi16(id));
                break; 
            case NodeSyntaxFunction:
                nodes_buffer[index].value = evaluate_tree_function(hi16(id), nodes_buffer[left].value);
                break;

            case NodeSyntaxParenthesis:
                nodes_buffer[index].value = nodes_buffer[left].value;
                break;
            case NodeSyntaxAbsolute:
                nodes_buffer[index].value = vec2(cabs(nodes_buffer[left].value), 0.0);
                break;

            case NodeSyntaxAddition:
                nodes_buffer[index].value = nodes_buffer[left].value + nodes_buffer[right].value;
                break;
            case NodeSyntaxSubtraction:
                nodes_buffer[index].value = nodes_buffer[left].value - nodes_buffer[right].value;
                break;
            case NodeSyntaxMultiplication:
                nodes_buffer[index].value = cmul(nodes_buffer[left].value, nodes_buffer[right].value);
                break;
            case NodeSyntaxDivision:
                nodes_buffer[index].value = cdiv(nodes_buffer[left].value, nodes_buffer[right].value);
                break;
            case NodeSyntaxExponent:
                nodes_buffer[index].value = cpow(nodes_buffer[left].value, nodes_buffer[right].value);
                break;
        }
    }
    return nodes_buffer[0].value;
}










const int GridTypeNone          = 0x0;
const int GridTypeLineX         = 0x1;
const int GridTypeLineY         = 0x2;
const int GridTypeLineCenter    = 0x4;
const int GridTypePolar         = 0x8;

const int GridColorSampleNone    = 0x0;
const int GridColorSampleInvert  = 0x1;

int grid_type = GridTypeLineX | GridTypeLineY;
int grid_color_sample = GridColorSampleNone;
vec2 grid_scale = vec2(3.0, 3.0);
float grid_width = 1.0;
vec3 grid_color_line_x = vec3(1.0, 0.0, 0.0);
vec3 grid_color_line_y = vec3(0.0, 1.0, 0.0);
vec3 grid_color_line_center = vec3(1.0, 0.0, 0.0);
vec3 grid_color_polar = vec3(1.0, 0.0, 0.0);

float render_grid_line(float point, float scale) {
    float coord = point * scale;
    return abs(fract(coord - 0.5) - 0.5) / (fwidth(coord) * grid_width);
}
float render_grid_line_center(vec2 point, vec2 scale) {
    vec2 coord = vec2(atan(point.x, point.y) * scale.x / pi, atan(point.x, point.y) * scale.y / pi);
    vec2 grid = abs(fract(coord - 0.5) - 0.5) / (fwidth(coord) * grid_width);
    return min(grid.x, grid.y);
}
float render_grid_polar(vec2 point, vec2 scale) {
    return render_grid_line(length(point), length(scale));
}
vec3 render_grid(vec2 point, vec3 input_color) {
    float grid = 1.0;
    vec3 color = vec3(1.0, 1.0, 1.0);
    bool selected = false;
    if (flag(grid_type, GridTypeLineX)) {
        float value = render_grid_line(point.x, grid_scale.x);
        if (value > 0.9 && !selected) {
            selected = true;
            color = grid_color_line_x;
        }
        grid = min(grid, value);
    }
    if (flag(grid_type, GridTypeLineY)) {
        float value = render_grid_line(point.y, grid_scale.y);
        if (value > 0.9 && !selected) {
            selected = true;
            color = grid_color_line_y;
        }
        grid = min(grid, value);
    }
    if (flag(grid_type, GridTypeLineCenter)) {
        float value = render_grid_line_center(point, grid_scale);
        if (value > 0.9 && !selected) {
            selected = true;
            color = grid_color_line_center;
        }
        grid = min(grid, value);
    }
    if (flag(grid_type, GridTypePolar)) {
        float value = render_grid_polar(point, grid_scale);
        if (value > 0.9 && !selected) {
            selected = true;
            color = grid_color_polar;
        }
        grid = min(grid, value);
    }

    
    vec3 mask = vec3(1.0 - min(grid, 1.0));
    if (grid_color_sample == GridColorSampleNone) {
        return ((1.0 - mask) * input_color) + (mask * color);
    }
    else if (grid_color_sample == GridColorSampleInvert) {
        return ((1.0 - mask * mask) * input_color) + (mask * mask * (1.0 - input_color));
    }
    return input_color;
}








void main() {
    vec2 sample_point = evaluate_tree();
    
    if (sample_point.x < -0.5 || sample_point.y < -0.5 ||
        sample_point.x > 0.5 || sample_point.y > 0.5
    ) {
        float dist = length(sample_point);
        //color = vec4((sin(dist) + 1.0) * 0.25, (cos(dist) + 1.0) * 0.25, 0.5, 1.0);
        color = vec4(vec3(0.5), 1.0);
    } else {
        color = texture(tex_sampler, sample_point + 0.5);
        //if (fequal(length(color), 0.0, 0.00001)) {
        //    color = texture(tex_sampler, -sample_point + 0.5);
        //}

        color.w = 1.0;
    }
    
    color.xyz = render_grid(sample_point, color.xyz);
}



