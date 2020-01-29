use crate::common::{
    GenerationID, GenerationVec, ScreenSize, ScreenTranslation, WorldPos, WorldScaleFactor,
};
use crate::components::Color;
use amethyst::{prelude::*, renderer::camera::Projection};

use specs::prelude::*;
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
    alias: HashMap<String, String>,
    commands: HashMap<String, CommandDesc>,
}

impl CommandList {
    pub fn new() -> Self {
        CommandList {
            alias: HashMap::new(),
            commands: HashMap::new(),
        }
    }
    pub fn add(&mut self, key: &str, value: CommandDesc) {
        self.commands.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&CommandDesc> {
        self.commands.get(key).or_else(|| {
            if let Some(alias) = self.alias.get(key) {
                self.commands.get(alias)
            } else {
                None
            }
        })
    }
    pub fn alias(&mut self, key: &str, command: &str) {
        self.alias.insert(key.to_string(), command.to_string());
    }
}

pub type CommandFunc = Box<fn(&mut World, &[InputDesc]) -> SimpleTrans>;

// Box<dyn Command>
// trait CommandFunc {
//     fn run(&mut self, world: &mut World) -> SimpleTrans;
// }

pub struct CommandDescBuilder {
    name: Option<String>,
    inputs: Vec<InputDesc>,
    exec: Option<CommandFunc>,
}

impl CommandDescBuilder {
    pub fn new(name: &str) -> Self {
        CommandDescBuilder {
            name: Some(name.to_string()),
            inputs: vec![],
            exec: None,
        }
    }
    pub fn with_input(mut self, input: InputDesc) -> Self {
        self.inputs.push(input);
        self
    }
    pub fn with_function(mut self, func: CommandFunc) -> Self {
        self.exec = Some(func);
        self
    }
    pub fn build(self) -> CommandDesc {
        if self.name.is_none() || self.exec.is_none() {
            panic!("tried to build an incomplete command");
        } else {
            CommandDesc {
                name: self.name.unwrap(),
                inputs: self.inputs,
                exec: self.exec.unwrap(),
            }
        }
    }
}

#[derive(Clone)]
pub struct CommandDesc {
    pub name: String,
    pub inputs: Vec<InputDesc>,
    pub exec: CommandFunc,
}

#[derive(Clone)]
pub enum InputDesc {
    Point,
    Select,
    Multiselect,
}

pub enum CapturedInput {
    Point(f64, f64),
    Select(Entity),
    Multiselect(Vec<Entity>),
}

// impl From<InputDesc> for CapturedInput {
//     fn from(desc: InputDesc) -> Self {
//         match
//     }
// }

pub struct ViewInfo {
    zoom_level: WorldScaleFactor,
    depth: f32,
    origin: WorldPos,
    screen: ScreenSize,
}

impl Default for ViewInfo {
    fn default() -> Self {
        ViewInfo {
            zoom_level: Default::default(),
            depth: 20.0,
            origin: Default::default(),
            screen: Default::default(),
        }
    }
}

impl ViewInfo {
    pub fn width(&self) -> f32 {
        self.screen.width as f32 * self.zoom_level.factor
    }
    pub fn height(&self) -> f32 {
        self.screen.height as f32 * self.zoom_level.factor
    }
    pub fn projection(&self) -> Projection {
        let half_width = self.width() / 2.0;
        let half_height = self.height() / 2.0;
        let half_depth = self.depth / 2.0;

        let o_x = self.origin.x as f32;
        let o_y = self.origin.y as f32;
        let o_z = self.origin.z as f32;

        let left = o_x - half_width;
        let right = o_x + half_width;
        let bottom = o_y - half_height;
        let top = o_y + half_height;
        let z_near = o_z - half_depth;
        let z_far = o_z + half_depth;

        Projection::orthographic(left, right, bottom, top, z_near, z_far)
    }
    pub fn pan(&mut self, delta: ScreenTranslation) {
        self.origin = self.origin - (self.zoom_level * delta);
    }
    pub fn zoom(&mut self, z: i32) {
        use std::cmp::Ordering;
        match z.cmp(&0) {
            Ordering::Greater => self.zoom_level.increase(),
            Ordering::Less => self.zoom_level.decrease(),
            Ordering::Equal => (),
        }
    }

    // pub fn reset_camera(&self, cam: &mut Camera) {
        // world.exec(|(mut cameras, mut view_info, active_camera): (WriteStorage<Camera>, WriteExpect<ViewInfo>, ReadStorage<ActiveCamera>)| {
            // for (cam, _) in (&mut cameras, &active_camera).join() {
        // cam.set_projection(self.projection());
            // }
        // });
    // }
    pub fn resize(&mut self, width: f64, height: f64) {
        self.screen.width = width;
        self.screen.height = height;
    }
}

