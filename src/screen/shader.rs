use crate::AnyResult;
use crate::screen::shader::names::*;
use crate::screen::theme::Materials;
use macroquad::material::{Material, MaterialParams, load_material};
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, ShaderSource, UniformDesc, UniformType,
};
use macroquad::prelude::PipelineParams;

pub mod names {
    pub const POSITION_X_NAME: &str = "position_x";
    pub const POSITION_Y_NAME: &str = "position_y";
    pub const RADAR: &str = "radar";
    pub const TILES: &str = "tiles";
    pub const COLOR_WHITE: &str = "color_white";
    pub const COLOR_BLACK: &str = "color_black";
    pub const REFEREE_SAW: &str = "referee_saw";
    pub const TEAM: &str = "team";
    pub const SIN_CITY: &str = "sin_city";
}

const FLOOR_FRAGMENT_SHADER: &'static str = include_str!("../shaders/floor_fragment.glsl");
const FLOOR_VERTEX_SHADER: &'static str = include_str!("../shaders/floor_vertex.glsl");

const CHARACTER_FRAGMENT_SHADER: &'static str = include_str!("../shaders/character_fragment.glsl");
const CHARACTER_VERTEX_SHADER: &'static str = include_str!("../shaders/character_vertex.glsl");

pub fn init_shaders() -> AnyResult<Materials> {
    let floor = floor_shader(FLOOR_VERTEX_SHADER, FLOOR_FRAGMENT_SHADER)?;
    let character = character_shader(CHARACTER_VERTEX_SHADER, CHARACTER_FRAGMENT_SHADER)?;
    Ok(Materials { floor, character })
}

pub fn floor_shader(vertex_code: &str, fragment_code: &str) -> AnyResult<Material> {
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
            UniformDesc {
                name: TILES.to_string(),
                uniform_type: UniformType::Float2,
                array_count: 1,
            },
            UniformDesc {
                name: RADAR.to_string(),
                uniform_type: UniformType::Float2,
                array_count: 3,
            },
            UniformDesc {
                name: COLOR_WHITE.to_string(),
                uniform_type: UniformType::Float4,
                array_count: 1,
            },
            UniformDesc {
                name: COLOR_BLACK.to_string(),
                uniform_type: UniformType::Float4,
                array_count: 1,
            },
        ],
        textures: vec![],
    };
    let floor = load_material(
        ShaderSource::Glsl {
            vertex: vertex_code,
            fragment: fragment_code,
        },
        material_params,
    )?;
    Ok(floor)
}

pub fn character_shader(vertex_code: &str, fragment_code: &str) -> AnyResult<Material> {
    let material_params = MaterialParams {
        pipeline_params: PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            alpha_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Zero,
                BlendFactor::One,
            )),
            ..Default::default()
        },
        uniforms: vec![
            UniformDesc {
                name: REFEREE_SAW.to_string(),
                uniform_type: UniformType::Int1,
                array_count: 1,
            },
            UniformDesc {
                name: TEAM.to_string(),
                uniform_type: UniformType::Int1,
                array_count: 1,
            },
            UniformDesc {
                name: SIN_CITY.to_string(),
                uniform_type: UniformType::Int1,
                array_count: 1,
            },
        ],
        textures: vec![],
    };
    let character = load_material(
        ShaderSource::Glsl {
            vertex: vertex_code,
            fragment: fragment_code,
        },
        material_params,
    )?;
    Ok(character)
}
