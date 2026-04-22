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

float dot_product(vec2 a, vec2 b) {
    return a.x * b.x + a.y * b.y;
}

bool counter_clockwise_triangle(vec2 a, vec2 b, vec2 c)  {
    return perp_dot((b - a), (c - a)) >= 0.0;
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
vec4 splat(float value) {
    return vec4(value, value, value, 1.0);
}
vec4 splat2(vec2 value) {
    return splat(value.x + value.y);
}

float distance_to_segment(vec2 a, vec2 b, vec2 point) {
    float thinness = 1.0;
    vec2 segment_0 = b - a;
    float segment_0_length = length(segment_0);
    vec2 segment_0_unit = segment_0 / segment_0_length;
    float coef_to_closest = dot_product(point - a, segment_0_unit);
    vec2 closest = a + segment_0_unit * coef_to_closest;
    float linear_dist;
    if (0.0 <= coef_to_closest && coef_to_closest <= segment_0_length) {
        float dist_to_segment = length(closest - point) * thinness;
        float dist = clamp(dist_to_segment, 0.0, 1.0);
    //    dist_to_segment = pow(dist_to_segment, 5.0);
    //    dist_to_segment = dist_to_segment * 0.5;
        linear_dist = dist;
    } else {
        float dist = length(point - b);
//        float dist = min(length(point - a), length(point - b));
//        float dist = length(point - (a + b) * 0.5);
        linear_dist = clamp(thinness * dist, 0.0, 1.0);
    }
    float inverted = 1.0 - linear_dist;
    return pow(inverted, 40.0);
}

void main() {
    vec4 color_super_black = vec4(0.0, 0.0, 0.0, 1.0);
    vec4 color_super_white = vec4(1.0, 1.0, 1.0, 1.0);
    vec2 tile = uv * tiles;
    ivec2 tile_i = ivec2(floor(tile));
    int manhattan_distance = tile_i.x + tile_i.y;
    bool is_even = manhattan_distance / 2 * 2 == manhattan_distance;
    bool inside_radar = triangle_contains(radar, tile);
    
    float blur = 1.0 - 2.0 * min2(abs(tile - floor(tile + 0.5)));
    blur = pow(blur, (1.0 - power) * 400.0);
    blur = blur * 0.5;
    
    vec4 color_inside;
    vec4 color_outside;
    if (is_even) {
        color_inside = lerp(color_super_black, color_super_white, blur);
    } else {
        color_inside = lerp(color_super_white, color_super_black, blur);
    }
    if (is_even){
        color_outside = lerp(color_black, color_white, blur);
    } else {
        color_outside = lerp(color_white, color_black, blur);
    }
    vec2 distance_sum = ((tile - radar[0]) + (tile - radar[1]) + (tile - radar[2]));
    float dist_0 = length(tile - radar[0]);
    float dist_1 = length(tile - radar[1]);
    float dist_center = length(tile - (radar[0] + radar[1] + radar[2]) /3.0);
    
    float blur_radar = 0.0;
    blur_radar += distance_to_segment(radar[1], radar[0], tile);
    blur_radar += distance_to_segment(radar[2], radar[0], tile);
    blur_radar += distance_to_segment(radar[2], radar[1], tile);
    blur_radar = clamp(blur_radar, 0.0, 1.0);
    blur_radar = blur_radar * 0.5;
//    vec4 color_blur_radar = splat(blur_radar);
    
    if (inside_radar) {
        gl_FragColor = lerp(color_inside, color_outside, blur_radar);
    } else {
//        gl_FragColor = color_outside;
        gl_FragColor = lerp(color_outside, color_inside, blur_radar);
    }
//    gl_FragColor = splat2(distance_sum);
//    gl_FragColor = splat(dist_0);
//    gl_FragColor = splat(dist_1);
//    gl_FragColor = splat(dist_center);
//    gl_FragColor = splat(blur_radar);
    
//    gl_FragColor = splat(dist_to_closest * 1.0);
//    gl_FragColor = splat(debug_dist);
//    gl_FragColor = splat2(tile / tiles);
}
