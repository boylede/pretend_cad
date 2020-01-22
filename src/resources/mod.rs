use crate::common::{GenerationID, GenerationVec};
use crate::components::Color;
use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
    window::ScreenDimensions,
};
use std::collections::HashMap;
use specs::prelude::*;

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


pub struct ViewInfo {
    pub zoom_level: f64,
    pub origin_x: f64,
    pub origin_y: f64,
    pub domain_h: f64,
    pub domain_w: f64,
}

impl ViewInfo {
    fn pan(&mut self, x: f64, y: f64) {
        //
    }
    fn zoom(&mut self, z: f64) {
        //
    }
    fn reset_camera(&self, world: &mut World) {
        world.exec(|(mut cameras, mut view_info): (WriteStorage<Camera>, WriteExpect<ViewInfo>)| {
            for cam in (&mut cameras).join() {
                let half_width = view_info.domain_w / 2.0;
                let half_height = view_info.domain_h / 2.0;
                let left = view_info.origin_x - half_width;
                // let left = ((self.domain_w * self.zoom_level) / -2.0).trunc() as f32;
                let right = view_info.origin_x + half_width;
                // let right = left + (self.domain_w * self.zoom_level) as f32;
                let top = view_info.origin_y + half_height;
                // let top = ((self.domain_h * self.zoom_level) / 2.0).trunc() as f32;
                let bottom = view_info.origin_y - half_height;
                // let bottom = top - (self.domain_h * self.zoom_level) as f32;
                let new_cam: Projection =
                    Projection::orthographic(left as f32, right as f32, bottom as f32, top as f32, 10.0, -10.0).into();
                cam.set_projection(new_cam);
            }
        });
    }
}

impl Default for ViewInfo {
    fn default() -> Self {
        ViewInfo {
            zoom_level: 1.0,
            origin_x: 0.0,
            origin_y: 0.0,
            domain_h: 10.0,
            domain_w: 10.0,
        }
    }
}
