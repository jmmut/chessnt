#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform int referee_saw;
uniform int team;
uniform int sin_city;

void main() {
    vec3 code = vec3(0.0, 1.0, 1.0); // cyan
    vec4 sampled = texture2D(Texture, uv);
    vec3 diff = abs(code - sampled.rgb);
    float dist = length(diff);
    bool is_code_color = dist < 0.01;
    
    if (is_code_color) {
        if (referee_saw > 0) {
            int white_or_black_i = 1 - team;
            float white_or_black = float(white_or_black_i);
            gl_FragColor = vec4(white_or_black, white_or_black, white_or_black, sampled.a);
        } else {
            vec3 color_out = color.rgb;
            gl_FragColor = vec4(color_out, sampled.a);
        }
    } else {
        if (referee_saw > 0) {
            float avg = (sampled.r + sampled.g + sampled.b) / 3.0;
            vec3 grey = vec3(avg, avg, avg);
            gl_FragColor = vec4(grey, sampled.a);
        } else {
            gl_FragColor = sampled;
        }
    }
    
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
