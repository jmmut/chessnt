#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

uniform vec2 screen;
uniform float antialias_strength;

float lightness(vec3 color) {
    return (color.r + color.g + color.b) / 3.0;
}

void main() {
    float delta = 0.01;
    vec4 sampled = texture2D(Texture, uv);
    vec4 sampled_left = texture2D(Texture, uv - vec2(1.0/screen.x, 0.0));
    vec4 sampled_left_2 = texture2D(Texture, uv - vec2(2.0/screen.x, 0.0));
    vec4 sampled_right = texture2D(Texture, uv + vec2(1.0/screen.x, 0.0));
    vec4 sampled_right_2 = texture2D(Texture, uv + vec2(2.0/screen.x, 0.0));
    vec4 sampled_up = texture2D(Texture, uv - vec2(0.0, 1.0/screen.y));
    vec4 sampled_up_2 = texture2D(Texture, uv - vec2(0.0, 2.0/screen.y));
    vec4 sampled_down = texture2D(Texture, uv + vec2(0.0, 1.0/screen.y));
    vec4 sampled_down_2 = texture2D(Texture, uv + vec2(0.0, 2.0/screen.y));
    
    vec4 sampled_up_left = texture2D(Texture, uv - 1.0/screen);
    vec4 sampled_up_right = texture2D(Texture, uv + vec2(1.0/screen.x, -1.0/screen.y));
    vec4 sampled_down_left = texture2D(Texture, uv + vec2(-1.0/screen.x, 1.0/screen.y));
    vec4 sampled_down_right = texture2D(Texture, uv + 1.0/screen);
    
//    gl_FragColor = (sampled * 16.0 
//        + sampled_left * 3.0
//        + sampled_right * 3.0
//        + sampled_up * 3.0
//        + sampled_down * 3.0
//        + sampled_up_left
//        + sampled_up_right
//        + sampled_down_left
//        + sampled_down_right
//    ) / 32.0;
//    float lightness = (sampled.r + sampled.g + sampled.b) / 3.0;
//    float main_coef = pow(100.0, (1.0 - antialias_strength) * lightness);// * pow(lightness, 0.5));
////    if (lightness <= 0.5 * antialias_strength) {
////        main_coef = 1.0;
////    }
//    gl_FragColor = (
//        sampled * main_coef 
//        + sampled_left * 5.0 * antialias_strength
//        + sampled_right * 5.0 * antialias_strength
//        + sampled_up * 5.0 * antialias_strength
//        + sampled_down * 5.0 * antialias_strength
//        + sampled_left_2 * 0.0 * antialias_strength
//        + sampled_right_2 * 0.0 * antialias_strength
//        + sampled_up_2 * 0.0 * antialias_strength
//        + sampled_down_2 * 0.0 * antialias_strength
//        + sampled_up_left * 1.0 * antialias_strength
//        + sampled_up_right * 1.0 * antialias_strength
//        + sampled_down_left * 1.0 * antialias_strength
//        + sampled_down_right * 1.0 * antialias_strength
//    ) / (antialias_strength * 24.0 + main_coef);
    
    vec4 vert = abs(sampled_up - sampled_down);
    vec4 horiz = abs(sampled_left - sampled_right);
    
    vec4 vert_2 = abs(sampled_up_2 - sampled_down_2);
    vec4 horiz_2 = abs(sampled_left_2 - sampled_right_2);
    
    float main_coef = 1.0;
//    vec4 average_8 = (0.0
//        + main_coef * sampled
//        + antialias_strength * (0.0
//            + sampled_left * 1.0
//            + sampled_right * 1.0
//            + sampled_up * 1.0
//            + sampled_down * 1.0
////                + sampled_left_2 * 1.0
////                + sampled_right_2 * 1.0
////                + sampled_up_2 * 1.0
////                + sampled_down_2 * 1.0
//            + sampled_up_left * 1.0
//            + sampled_up_right * 1.0
//            + sampled_down_left * 1.0
//            + sampled_down_right * 1.0
//        )
//    ) / (antialias_strength * 8.0 + main_coef) * 1.0;
    
    vec4 average = (0.0
        + main_coef * sampled
        + antialias_strength * (0.0
            + sampled_left * 1.0
            + sampled_right * 1.0
            + sampled_up * 1.0
            + sampled_down * 1.0
                + sampled_left_2 * 1.0
                + sampled_right_2 * 1.0
                + sampled_up_2 * 1.0
                + sampled_down_2 * 1.0
            + sampled_up_left * 1.0
            + sampled_up_right * 1.0
            + sampled_down_left * 1.0
            + sampled_down_right * 1.0
        )
    ) / (antialias_strength * 12.0 + main_coef) *1.05;
    
    float edge_threshold = 0.34;
    if (lightness(abs(sampled - average).rgb) < edge_threshold
//        || lightness(vert_2.rgb) < edge_threshold || lightness(horiz_2.rgb) < edge_threshold
    ) {
        // this pixel is part of a uniform border
        gl_FragColor = sampled;
    } else {
        float main_coef = 1.0;
        gl_FragColor = average;
    }


    //    gl_FragColor = (sampled * 6.0 + sampled_left + sampled_right + sampled_up + sampled_down) *0.125;
//    gl_FragColor = (sampled * 14.0 + sampled_left + sampled_right) /16.0;
//    gl_FragColor = sampled;
}
