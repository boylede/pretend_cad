use crate::common::{GenerationID, GenerationVec, WorldScaleFactor, WorldPos, ScreenSize, ScreenTranslation};
use crate::components::{Color, ActiveCamera};
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


}

pub enum CapturedInput {
    Point(f64, f64),
    Select(Entity),
    Multiselect(Vec<Entity>),
}


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
        
        Projection::orthographic(
            left,
            right,
            bottom,
            top,
            z_near,
            z_far,
        )
    }
    pub fn pan(&mut self, delta: ScreenTranslation) {
        self.origin = self.origin - (self.zoom_level * delta);
    }
    pub fn zoom(&mut self, z: f32) {
        if z > 0.0 {
            self.zoom_level.increase();
        } else if z < 0.0 {
            self.zoom_level.decrease();
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

    }
}

