use crate::AnyResult;
use crate::screen::shader::names::*;
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
    pub const CURSOR_COLOR: &str = "cursor_color";
    pub const CURSOR_ON_TOP: &str = "cursor_on_top";
    pub const SHADOW_OFFSET: &str = "shadow_offset";
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
    pub antialias_enabled: bool,
    pub antialias_strength: f32,
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
        antialias_enabled: true,
        antialias_strength: 0.25,
        refresh_shaders: RefreshShaders {
            character: false,
            antialias: false,
            character_error: None,
            antialias_error: None,
        },
    })
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
            UniformDesc {
                name: CURSOR_ON_TOP.to_string(),
                uniform_type: UniformType::Int1,
                array_count: 1,
            },
            UniformDesc {
                name: CURSOR_COLOR.to_string(),
                uniform_type: UniformType::Float4,
                array_count: 1,
            },
            UniformDesc {
                name: SHADOW_OFFSET.to_string(),
                uniform_type: UniformType::Float1,
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

pub fn antialias_shader(vertex_code: &str, fragment_code: &str) -> AnyResult<Material> {
    let material_params = MaterialParams {
        pipeline_params: Default::default(),
        uniforms: vec![
            UniformDesc {
                name: SCREEN.to_string(),
                uniform_type: UniformType::Float2,
                array_count: 1,
            },
            UniformDesc {
                name: ANTIALIAS_STRENGTH.to_string(),
                uniform_type: UniformType::Float1,
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
    let material = load_material(
        ShaderSource::Glsl {
            vertex: vertex_code,
            fragment: fragment_code,
        },
        material_params,
    )?;
    Ok(material)
}
