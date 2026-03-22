use crate::AnyResult;
use macroquad::material::{Material, MaterialParams, load_material};
use macroquad::miniquad::{ShaderSource, UniformDesc, UniformType};

pub const POSITION_X_NAME: &str = "position_x";
pub const POSITION_Y_NAME: &str = "position_y";

pub mod names {
    pub const RADAR: &str = "radar";
}

const FRAGMENT_SHADER: &'static str = include_str!("../shaders/floor_fragment.glsl");
const VERTEX_SHADER: &'static str = include_str!("../shaders/floor_vertex.glsl");

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
