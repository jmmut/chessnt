use crate::AnyResult;
use crate::screen::shader::names::*;
use macroquad::Error;
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
    pub const POWER: &str = "power";
    pub const REFEREE_SAW: &str = "referee_saw";
    pub const TEAM: &str = "team";
    pub const SIN_CITY: &str = "sin_city";
    pub const CURSOR_COLOR: &str = "cursor_color";
    pub const CURSOR_ON_TOP: &str = "cursor_on_top";
    pub const SHADOW_OFFSET: &str = "shadow_offset";
    pub const CODE_TOLERANCE: &str = "code_tolerance";
    pub const SCREEN: &str = "screen";
    pub const ANTIALIAS_STRENGTH: &str = "antialias_strength";
    pub const OUTLINE_THICKNESS: &str = "outline_thickness";
    pub const TEXT_COLOR: &str = "text_color";
    pub const OUTLINE_COLOR: &str = "outline_color";
}

const FLOOR_FRAGMENT_SHADER: &'static str = include_str!("../shaders/floor_fragment.glsl");
const FLOOR_VERTEX_SHADER: &'static str = include_str!("../shaders/floor_vertex.glsl");

const CHARACTER_FRAGMENT_SHADER: &'static str = include_str!("../shaders/character_fragment.glsl");
const CHARACTER_VERTEX_SHADER: &'static str = include_str!("../shaders/character_vertex.glsl");

const ANTIALIAS_FRAGMENT_SHADER: &'static str = include_str!("../shaders/antialias_fragment.glsl");
const ANTIALIAS_VERTEX_SHADER: &'static str = include_str!("../shaders/antialias_vertex.glsl");

pub const OUTLINE_FRAGMENT_SHADER: &'static str = include_str!("../shaders/outline_fragment.glsl");
pub const OUTLINE_VERTEX_SHADER: &'static str = include_str!("../shaders/outline_vertex.glsl");

pub struct Materials {
    pub floor: Material,
    pub character: Material,
    pub antialias: Material,
    pub outline: Material,
    pub sin_city: bool,
    pub shadow_offset: f32,
    pub code_tolerance: f32,
    pub antialias_enabled: bool,
    pub antialias_strength: f32,
    pub floor_antialias_strength: f32,
    pub refresh_shaders: RefreshShaders,
}
pub struct RefreshShaders {
    pub character: bool,
    pub antialias: bool,
    pub character_error: Option<String>,
    pub antialias_error: Option<String>,
}

pub fn init_shaders() -> AnyResult<Materials> {
    let floor = floor_shader(FLOOR_VERTEX_SHADER, FLOOR_FRAGMENT_SHADER)?;
    let character = character_shader(CHARACTER_VERTEX_SHADER, CHARACTER_FRAGMENT_SHADER)?;
    let antialias = antialias_shader(ANTIALIAS_VERTEX_SHADER, ANTIALIAS_FRAGMENT_SHADER)?;
    let outline = outline_shader(OUTLINE_VERTEX_SHADER, OUTLINE_FRAGMENT_SHADER)?;
    Ok(Materials {
        floor,
        character,
        antialias,
        outline,
        sin_city: false,
        shadow_offset: 0.2,
        code_tolerance: 0.4,
        antialias_enabled: false,
        antialias_strength: 1.0,
        floor_antialias_strength: 0.85,
        refresh_shaders: RefreshShaders {
            character: false,
            antialias: false,
            character_error: None,
            antialias_error: None,
        },
    })
}

fn load_shader(
    name: &str,
    vertex_code: &str,
    fragment_code: &str,
    material_params: MaterialParams,
) -> AnyResult<Material> {
    let loaded = load_material(
        ShaderSource::Glsl {
            vertex: vertex_code,
            fragment: fragment_code,
        },
        material_params,
    )
    .map_err(|e| {
        let mq_message = match e {
            // Shader Error has a good specific formatting that is lost on Error::to_string.
            Error::ShaderError(shader_error) => shader_error.to_string(),
            _ => e.to_string(),
        };
        format!("Failed to compile shader '{}': {}", name, mq_message).into()
    });
    loaded
}

pub fn floor_shader(vertex_code: &str, fragment_code: &str) -> AnyResult<Material> {
    let material_params = MaterialParams {
        pipeline_params: Default::default(),
        uniforms: vec![
            UniformDesc::new(POSITION_X_NAME, UniformType::Float1),
            UniformDesc::new(POSITION_Y_NAME, UniformType::Float1),
            UniformDesc::new(TILES, UniformType::Float2),
            UniformDesc::new(RADAR, UniformType::Float2).array(3),
            UniformDesc::new(COLOR_WHITE, UniformType::Float4),
            UniformDesc::new(COLOR_BLACK, UniformType::Float4),
            UniformDesc::new(POWER, UniformType::Float1),
        ],
        textures: vec![],
    };
    load_shader("floor", vertex_code, fragment_code, material_params)
}

pub fn character_shader(vertex_code: &str, fragment_code: &str) -> AnyResult<Material> {
    let material_params = MaterialParams {
        pipeline_params: PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            ..Default::default()
        },
        uniforms: vec![
            UniformDesc::new(REFEREE_SAW, UniformType::Int1),
            UniformDesc::new(TEAM, UniformType::Int1),
            UniformDesc::new(SIN_CITY, UniformType::Int1),
            UniformDesc::new(CURSOR_ON_TOP, UniformType::Int1),
            UniformDesc::new(CURSOR_COLOR, UniformType::Float4),
            UniformDesc::new(SHADOW_OFFSET, UniformType::Float1),
            UniformDesc::new(CODE_TOLERANCE, UniformType::Float1),
        ],
        textures: vec![],
    };
    load_shader("character", vertex_code, fragment_code, material_params)
}

pub fn antialias_shader(vertex_code: &str, fragment_code: &str) -> AnyResult<Material> {
    let material_params = MaterialParams {
        pipeline_params: Default::default(),
        uniforms: vec![
            UniformDesc::new(SCREEN, UniformType::Float2),
            UniformDesc::new(ANTIALIAS_STRENGTH, UniformType::Float1),
        ],
        textures: vec![],
    };
    load_shader("antialias", vertex_code, fragment_code, material_params)
}

pub fn outline_shader(vertex_code: &str, fragment_code: &str) -> AnyResult<Material> {
    let material_params = MaterialParams {
        pipeline_params: PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            ..Default::default()
        },
        uniforms: vec![
            UniformDesc::new(SCREEN, UniformType::Float2),
            UniformDesc::new(OUTLINE_THICKNESS, UniformType::Float1),
            UniformDesc::new(TEXT_COLOR, UniformType::Float4),
            UniformDesc::new(OUTLINE_COLOR, UniformType::Float4),
        ],
        textures: vec![],
    };
    load_shader("outline", vertex_code, fragment_code, material_params)
}
