#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform vec4 text_color;
uniform vec4 outline_color;
uniform vec2 screen;


// apparently text is rendered only in the alpha channel, and both the letters and the background
// are white. This was unnecessarily hard to reverse engineer!

void main() {
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

    float alpha_surrounding = 
        sampled_left.a
        + sampled_right.a 
        + sampled_up.a 
        + sampled_down.a 
        + sampled_up_left.a 
        + sampled_up_right.a
        + sampled_down_left.a 
        + sampled_down_right.a;
    
    if (sampled.a == 0.0) {
        if (alpha_surrounding > 0.0) {
            float outline_alpha = alpha_surrounding / 8.0;
            gl_FragColor = vec4(outline_color.rgb * outline_alpha, outline_alpha); 
        } else {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        }
    } else if (sampled.a == 1.0) {
        gl_FragColor = vec4(text_color.rgb, sampled.a);
    } else {
        gl_FragColor = vec4(text_color.rgb * sampled.a + outline_color.rgb * (1.0 - sampled.a), 1.0);
    }
//    gl_FragColor = color;
//    gl_FragColor = vec4(sampled.rgb, 1.0);
}

