#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 tiles;
uniform vec4 color_black;
uniform vec4 color_white;
uniform vec2 radar[3];
uniform float position_x;
uniform float position_y;
uniform float power;

float perp_dot(vec2 a, vec2 b)  {
    return (a.x * b.y) - (a.y * b.x);
}

bool counter_clockwise_triangle(vec2 a, vec2 b, vec2 c)  {
    return perp_dot((b -a ), (c - a)) >= 0.0;
}

bool triangle_contains(vec2 triangle[3], vec2 point) {
    return counter_clockwise_triangle(triangle[0], triangle[1], point)
        && counter_clockwise_triangle(triangle[1], triangle[2], point)
        && counter_clockwise_triangle(triangle[2], triangle[0], point)
    ;
}

float min2(vec2 a) {
    return min(a.x, a.y);
}

/// coef 0 means all a, coef 1 means all b
vec4 lerp(vec4 a, vec4 b, float coef) {
    return a * (1.0 - coef) + b * coef;
}

void main() {
    vec4 color_super_black = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 color_super_white = vec4(1.0, 1.0, 1.0, 1.0);
    vec2 tile = uv * tiles;
    ivec2 tile_i = ivec2(floor(tile));
    int manhattan_distance = tile_i.x + tile_i.y;
    bool is_even = manhattan_distance / 2 * 2 == manhattan_distance;
    bool inside_radar = triangle_contains(radar, tile);
    if (inside_radar) {
        float blur = 1.0 - 2.0 * min2(abs(tile - floor(tile + 0.5)));
        blur = pow(blur, (1.0 - power) * 400.0);
        blur = blur * 0.5;
        
        if (is_even) {
            gl_FragColor = lerp(color_super_black, color_super_white, blur);
        } else {
            gl_FragColor = lerp(color_super_white, color_super_black, blur);
        }
    } else {
        if (is_even ^^ inside_radar){
            gl_FragColor = color_black;
        } else {
            gl_FragColor = color_white;
        }
    }
}
