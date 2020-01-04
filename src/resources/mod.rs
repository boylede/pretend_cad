use crate::common::GenerationID;
use crate::common::GenerationVec;
use crate::components::Color;

pub type Layers = GenerationVec<Layer>;

pub type LineTypes = GenerationVec<LineType>;

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub name: String,
    pub color: Color,
    pub line_type: GenerationID<LineType>,
    pub hidden: bool,
    pub frozen: bool,
    pub locked: bool,
}

#[derive(Debug, PartialEq)]
pub struct LineType {
    pub draw_line: fn(f32, f32) -> bool,
}

impl LineType {
    pub fn line_type_continous(_position: f32, _scale: f32) -> bool {
        true
    }
    pub fn line_type_hidden(position: f32, scale: f32) -> bool {
        (position * scale) as i32 % 2 == 0
    }
}
