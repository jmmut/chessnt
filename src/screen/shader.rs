use crate::AnyResult;
use crate::screen::shader::names::{COLOR_BLACK, COLOR_WHITE, RADAR, TILES};
use crate::screen::theme::Materials;
use macroquad::material::{Material, MaterialParams, load_material};
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, ShaderSource, UniformDesc, UniformType,
};
use macroquad::prelude::PipelineParams;

pub const POSITION_X_NAME: &str = "position_x";
pub const POSITION_Y_NAME: &str = "position_y";

pub mod names {
    pub const RADAR: &str = "radar";
    pub const TILES: &str = "tiles";
    pub const COLOR_WHITE: &str = "color_white";
    pub const COLOR_BLACK: &str = "color_black";
}

const FLOOR_FRAGMENT_SHADER: &'static str = include_str!("../shaders/floor_fragment.glsl");
const FLOOR_VERTEX_SHADER: &'static str = include_str!("../shaders/floor_vertex.glsl");

const CHARACTER_FRAGMENT_SHADER: &'static str = include_str!("../shaders/character_fragment.glsl");
const CHARACTER_VERTEX_SHADER: &'static str = include_str!("../shaders/character_vertex.glsl");

pub fn init_shaders() -> AnyResult<Materials> {
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
            vertex: FLOOR_VERTEX_SHADER,
            fragment: FLOOR_FRAGMENT_SHADER,
        },
        material_params,
    )?;
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
        uniforms: vec![],
        textures: vec![],
    };
    let character = load_material(
        ShaderSource::Glsl {
            vertex: CHARACTER_VERTEX_SHADER,
            fragment: CHARACTER_FRAGMENT_SHADER,
        },
        material_params,
    )?;
    Ok(Materials { floor, character })
}
