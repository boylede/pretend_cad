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
    components::{Color, Drawable, FullColor, Line, Point},
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
    use rand::prelude::*;
    use nalgebra::geometry::Point as nPoint;
    let mut rng = rand::thread_rng();
    let a = Point {
        x: rng.gen_range(0, 600) as f32,
        y: rng.gen_range(0, 600) as f32,
    };
    let b = Point {
        x: rng.gen_range(0, 600) as f32,
        y: rng.gen_range(0, 600) as f32,
    };
    let c = FullColor {
        r: 234,
        g: 65,
        b: 212,
    };
    // let line = Drawable::Line(Line {
    //     start: a,
    //     end: b,
    //     layer: layer,
    //     color: Color::Full(c),
    //     scale: 1.0,
    //     linetype: style,
    //     weight: 1.0,
    // });
    let mut debug_lines = DebugLinesComponent::new();
    let start: nPoint<f32, nalgebra::base::dimension::U3> =
        nPoint::from_slice(&[a.x, a.y, 0.0]);
    let end: nPoint<f32, nalgebra::base::dimension::U3> = nPoint::from_slice(&[b.x, b.y, 0.0]);
    let color = Srgba::new(c.r as f32, c.g as f32, c.b as f32, 1.0);
    debug_lines.add_line(start, end, color);

    w.create_entity().with(debug_lines).build();
    Trans::Pop
}