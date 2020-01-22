use amethyst::renderer::{debug_drawing::DebugLinesComponent, palette::Srgba};
use rand::prelude::*;
use specs::prelude::*;

use nalgebra::geometry::Point as nPoint;

use crate::common::GenerationID;

use crate::resources::Layer;
use crate::resources::LineType;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ActiveCamera;

impl Component for ActiveCamera {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FullColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Drawable {
    Point(Point),
    Line(Line),
    LineSet(Set),
    NamedGroup(Group),
}

impl Component for Drawable {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
    layer: GenerationID<Layer>,
    color: Color,
    scale: f32,
    linetype: GenerationID<LineType>,
    weight: f32,
}

impl Line {
    pub fn create(
        style: GenerationID<LineType>,
        layer: GenerationID<Layer>,
    ) -> (Drawable, DebugLinesComponent) {
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
        let line = Drawable::Line(Line {
            start: a,
            end: b,
            layer: layer,
            color: Color::Full(c),
            scale: 1.0,
            linetype: style,
            weight: 1.0,
        });
        let mut debug_lines = DebugLinesComponent::new();
        let start: nPoint<f32, nalgebra::base::dimension::U3> =
            nPoint::from_slice(&[a.x, a.y, 0.0]);
        let end: nPoint<f32, nalgebra::base::dimension::U3> = nPoint::from_slice(&[b.x, b.y, 0.0]);
        let color = Srgba::new(c.r as f32, c.g as f32, c.b as f32, 1.0);
        debug_lines.add_line(start, end, color);
        (line, debug_lines)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Fixed(u8),
    Full(FullColor),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Set {
    inner: Vec<Line>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    inner: Vec<Drawable>,
}
