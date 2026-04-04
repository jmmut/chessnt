#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec3 code = vec3(241.0 / 255.0, 1.0, 135.0/255.0);
    vec4 sampled = texture2D(Texture, uv);
    vec3 diff = code - sampled.rgb;
    float dist = abs(diff.r) + abs(diff.g) + abs(diff.b);
    if (dist < 0.5) {
        vec3 color_out = sampled.rgb * dist + color.rgb * (1.0 - dist);
        gl_FragColor = vec4(color_out, sampled.a);
    } else {
        gl_FragColor = sampled;
    }
}
