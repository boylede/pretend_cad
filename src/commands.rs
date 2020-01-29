use amethyst::{
    prelude::*,
    renderer::{
        debug_drawing::{DebugLinesComponent},
        palette::Srgba,
    },
};

use crate::{
    components::{ FullColor, Point},
    resources::{CommandList, CommandDesc, CommandDescBuilder, InputDesc},
};

pub fn register_commands() -> CommandList {
    let mut commands = CommandList::new();
    // quit
    // commands.add("quit".to_string(), Box::new(quit_command));
    let quit = CommandDescBuilder::new("quit")
        .with_function(Box::new(quit_command))
        .build();
    commands.add("quit", quit);
    commands.alias("exit", "quit");
    //line
    let line = CommandDescBuilder::new("line")
        .with_function(Box::new(line_command))
        .with_input(InputDesc::Point)
        .with_input(InputDesc::Point)
        .build();
    commands.add("line", line);
    commands.alias("l", "line");

    let arc = CommandDescBuilder::new("arc")
        .with_function(Box::new(arc_command))
        .with_input(InputDesc::Point)
        .with_input(InputDesc::Point)
        .with_input(InputDesc::Point)
        .build();
    commands.add("arc", arc);

    commands
}

fn arc_command(_: &mut World, _: &Vec<InputDesc>) -> SimpleTrans {
    Trans::Quit
}

fn quit_command(_: &mut World, _: &Vec<InputDesc>) -> SimpleTrans {
    Trans::Quit
}

fn line_command(w: &mut World, _: &Vec<InputDesc>) -> SimpleTrans {
    use nalgebra::geometry::Point as nPoint;
    use rand::prelude::*;
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
    let start: nPoint<f32, nalgebra::base::dimension::U3> = nPoint::from_slice(&[a.x, a.y, 0.0]);
    let end: nPoint<f32, nalgebra::base::dimension::U3> = nPoint::from_slice(&[b.x, b.y, 0.0]);
    let color = Srgba::new(c.r as f32, c.g as f32, c.b as f32, 1.0);
    debug_lines.add_line(start, end, color);

    w.create_entity().with(debug_lines).build();
    Trans::Pop
}
