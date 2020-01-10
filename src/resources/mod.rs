use crate::common::{GenerationID, GenerationVec};
use crate::components::Color;
use amethyst::prelude::*;
use std::collections::HashMap;

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

pub struct CommandList {
    inner: HashMap<String, Command>,
}

impl CommandList {
    pub fn new() -> Self {
        CommandList {
            inner: HashMap::new(),
        }
    }
    pub fn add(&mut self, key: String, value: Command) {
        self.inner.insert(key, value);
    }
    pub fn get(&self, key: &str) -> Option<&Command> {
        self.inner.get(key)
    }
}

pub type Command = Box<fn(&mut World) -> SimpleTrans>;

// Box<dyn Command>
// trait Command {
//     fn run(&mut self, world: &mut World) -> SimpleTrans;
// }

struct CommandBuilder {
    name: String,
    inputs: Vec<InputDesc>,
    exec: Box<fn(&mut World) -> SimpleTrans>,
}

struct InputDesc {}
