#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;


void main() {
    float delta = 0.01;
    vec4 sampled = texture2D(Texture, uv);
    vec4 sampled_left = texture2D(Texture, uv - vec2(delta, 0.0));
    vec4 sampled_right = texture2D(Texture, uv + vec2(delta, 0.0));
    gl_FragColor = (sampled * 2.0 + sampled_left + sampled_right) *0.25;
}
