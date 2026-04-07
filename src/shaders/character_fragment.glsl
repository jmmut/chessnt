#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform int referee_saw;
uniform int team;
uniform int sin_city;
uniform vec4 cursor_color;
uniform int cursor_on_top;
uniform float shadow_offset;

void main() {
    vec3 code = vec3(0.0, 1.0, 1.0); // cyan
    vec4 sampled = texture2D(Texture, uv);
    vec3 diff = abs(code - sampled.rgb);
    float dist = length(diff);
    bool is_code_color = dist < 0.01;
    vec3 cursor_white = vec3(0.48,0.78, 0.24);
    vec3 cursor_black = vec3(0.18, 0.59, 0.45);

    vec4 color_out;

    vec2 right_uv = uv + vec2(shadow_offset, 0.0);
    if (right_uv.x >= 0.0 && right_uv.x <= 1.0) {
        vec4 right = texture2D(Texture, right_uv);
        if (right.a > 0.5 && cursor_on_top == 1) {
            color_out = cursor_color;
        }
    }
    if (uv.x >= 0.0 && uv.x <= 1.0 && sampled.a >= 0.1) {
        if (is_code_color) {
            if (referee_saw == 1 && sin_city == 0) {
                int white_or_black_i = 1 - team;
                float white_or_black = float(white_or_black_i);
                color_out = vec4(white_or_black, white_or_black, white_or_black, sampled.a);
            } else {
                color_out = vec4(color.rgb, sampled.a);
            }
        } else {
            if (referee_saw > 0) {
                float avg = (sampled.r + sampled.g + sampled.b) / 3.0;
                vec3 grey = vec3(avg, avg, avg);
                color_out = vec4(grey, sampled.a);
            } else {
                color_out = sampled;
            }
        }
    }
    gl_FragColor = color_out;
    
//    
//    vec3 diff = code - sampled.rgb;
//    float dist_code = (abs(diff.r) + abs(diff.g) + abs(diff.b)) / 3.0;
//    float dist_black = (code.r + code.g + code.b) / 3.0;
//    float dist_line = (dist_code + dist_black) * 0.5;
////    float dist_2 = dist * dist;
//    if (dist_code < 0.3 ) {
//        vec3 color_out = sampled.rgb * dist_code + color.rgb * (1.0 - dist_code);
//        gl_FragColor = vec4(color_out, sampled.a);
//    } else {
//        gl_FragColor = sampled;
//    }
}
