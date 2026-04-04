#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec4 sampled = texture2D(Texture, uv);
    vec3 color_out = sampled.rgb * 0.5 + color.rgb * 0.5;
    gl_FragColor = vec4(color_out, sampled.a);
}
