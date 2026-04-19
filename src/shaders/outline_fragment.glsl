#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

// apparently text is rendered only in the alpha channel, and both the letters and the background
// are white. This was unnecessarily hard to reverse engineer!
void main() {
    vec4 sampled = texture2D(Texture, uv);
    gl_FragColor = vec4(0.0, 0.0, 0.0, sampled.a);
//    gl_FragColor = color;
//    gl_FragColor = vec4(sampled.rgb, 1.0);
}


//
//varying lowp vec2 uv;
//
//uniform sampler2D Texture;
//uniform lowp vec4 test_color;
//
//void main() {
//    gl_FragColor = test_color * texture2D(Texture, uv);
//}