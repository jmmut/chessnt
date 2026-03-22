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

void main() {
    vec2 tile = uv * tiles;
    ivec2 tile_i = ivec2(floor(tile));
    int manhattan_distance = tile_i.x + tile_i.y;
    bool is_even = manhattan_distance / 2 * 2 == manhattan_distance;
    bool inside_radar = triangle_contains(radar, tile);
    if (is_even ^^ inside_radar){
        gl_FragColor = color_white;
    } else {
        gl_FragColor = color_black;
    }
}
