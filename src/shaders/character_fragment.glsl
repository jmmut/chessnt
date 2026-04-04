#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec3 code = vec3(241.0 / 255.0, 1.0, 135.0/255.0);
    vec4 sampled = texture2D(Texture, uv);
    
    vec3 p = sampled.rgb / code;  // NOTE: division by 0!
    float p_avg = (p.r + p.g + p.b) / 3.0;
    float dist = abs(p.r - p_avg) + abs(p.g - p_avg) + abs(p.b - p_avg);
    if (dist < 10000.0 && p_avg > 0.0) {
        vec3 color_out = sampled.rgb * dist + color.rgb * p * (1.0 - dist);
        gl_FragColor = vec4(color_out, sampled.a);
    } else {
        gl_FragColor = sampled;
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
