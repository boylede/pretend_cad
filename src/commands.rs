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

use crate::{
    components::{Color, Drawable, FullColor, Line},
    resources::{Layer, Layers, LineType, LineTypes, CommandList},
    states::{CommandEntryState, PanState},
};

use std::collections::HashMap;

pub fn register_commands() -> CommandList {
    let mut commands = CommandList::new();
    commands.add("quit".to_string(), Box::new(quit_command));
    commands.add("line".to_string(), Box::new(line_command));
    commands
}

fn quit_command(w: &mut World) -> SimpleTrans {
    Trans::Quit
}

fn line_command(w: &mut World) -> SimpleTrans {
    unimplemented!()
}