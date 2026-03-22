#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 tiles;
uniform vec4 color_black;
uniform vec4 color_white;
uniform float position_x;
uniform float position_y;

void main() {
    ivec2 tile = ivec2(floor(uv * tiles));
    int manhattan_distance = tile.x + tile.y;
    bool is_even = manhattan_distance / 2 * 2 == manhattan_distance;
    if (is_even){
        gl_FragColor = color_white;
    } else {
        gl_FragColor = color_black;
    }
}
