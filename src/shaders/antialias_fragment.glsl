#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

uniform vec2 screen;

void main() {
    float delta = 0.01;
    vec4 sampled = texture2D(Texture, uv);
    vec4 sampled_left = texture2D(Texture, uv - vec2(1.0/screen.x, 0.0));
    vec4 sampled_right = texture2D(Texture, uv + vec2(1.0/screen.x, 0.0));
    vec4 sampled_up = texture2D(Texture, uv - vec2(0.0, 1.0/screen.y));
    vec4 sampled_down = texture2D(Texture, uv + vec2(0.0, 1.0/screen.y));
    
    vec4 sampled_up_left = texture2D(Texture, uv - 1.0/screen);
    vec4 sampled_up_right = texture2D(Texture, uv + vec2(1.0/screen.x, -1.0/screen.y));
    vec4 sampled_down_left = texture2D(Texture, uv + vec2(-1.0/screen.x, 1.0/screen.y));
    vec4 sampled_down_right = texture2D(Texture, uv + 1.0/screen);
    
    gl_FragColor = (sampled * 16.0 
        + sampled_left * 3.0
        + sampled_right * 3.0
        + sampled_up * 3.0
        + sampled_down * 3.0
        + sampled_up_left
        + sampled_up_right
        + sampled_down_left
        + sampled_down_right
    ) / 32.0;
//    gl_FragColor = (sampled * 6.0 + sampled_left + sampled_right + sampled_up + sampled_down) *0.125;
//    gl_FragColor = (sampled * 14.0 + sampled_left + sampled_right) /16.0;
//    gl_FragColor = sampled;
}
