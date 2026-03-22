use crate::AnyResult;
use macroquad::material::{Material, MaterialParams, load_material};
use macroquad::miniquad::{ShaderSource, UniformDesc, UniformType};

pub const POSITION_X_NAME: &str = "position_x";
pub const POSITION_Y_NAME: &str = "position_y";

const FRAGMENT_SHADER: &'static str = r#"#version 100
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
"#;

const VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";

pub fn init_shaders() -> AnyResult<Material> {
    let material_params = MaterialParams {
        pipeline_params: Default::default(),
        uniforms: vec![
            UniformDesc {
                name: POSITION_X_NAME.to_string(),
                uniform_type: UniformType::Float1,
                array_count: 1,
            },
            UniformDesc {
                name: POSITION_Y_NAME.to_string(),
                uniform_type: UniformType::Float1,
                array_count: 1,
            },
        ],
        textures: vec![],
    };
    Ok(load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        material_params,
    )?)
}
