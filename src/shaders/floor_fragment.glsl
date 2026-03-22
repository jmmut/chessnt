#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform float position_x;
uniform float position_y;


const int max_iterations = 100;

/// complex product (imaginary numbers): (a+bi)*(c+di) = a*c - b*d + (a*d + b*c)*i
vec2 complex_product(vec2 v1, vec2 v2)
{
    return vec2(v1.x*v2.x - v1.y*v2.y, v1.x*v2.y + v1.y*v2.x);
}

/// this returns an int to make further iterations maintain the color of outer patterns,
/// if it returned a float [0, 1] the colors would stretch, making more difficult to discern borders
int mandelbrot_float_precision(vec2 uv)
{
    float zoom = 4.0;
    vec2 z = vec2((uv.x - 0.5)*zoom, (uv.y- 0.5)*zoom);
    //vec2 c = vec2(c_x, c_y);  // comment/uncomment: test wether vec2 works or not
    float radius = 30.0;
    vec2 c = vec2(position_x, position_y);

    for (int n=0; n<max_iterations; n++)
    {
        z = complex_product(z, z) + c;
        if(sqrt(z.x*z.x + z.y*z.y) > radius)
            return n;
    }

    return max_iterations;
}

void main() {
    int diverges = mandelbrot_float_precision(uv);
    float color = float(diverges) / float(max_iterations);
    vec3 res = vec3(color, color, color);
    gl_FragColor = vec4(res, 1.0);
}
