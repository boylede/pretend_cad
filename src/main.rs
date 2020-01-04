use amethyst::{
    core::transform::{Transform, TransformBundle},
    input::{get_key, is_close_requested, is_key_down, Button, InputBundle, VirtualKeyCode},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
        plugins::{RenderDebugLines, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    window::{DisplayConfig, ScreenDimensions},
};
use rand::prelude::*;
use specs::prelude::*;
use winit::WindowEvent;

use nalgebra::geometry::Point as nPoint;

type Layers = GenerationVec<Layer>;

type LineTypes = GenerationVec<LineType>;

#[derive(Debug, PartialEq)]
struct Layer {
    name: String,
    color: Color,
    line_type: GenerationID<LineType>,
    hidden: bool,
    frozen: bool,
    locked: bool,
}

#[derive(Debug, PartialEq)]
struct GenerationID<M> {
    id: usize,
    generation: usize,
    _marker: std::marker::PhantomData<M>,
}

impl<M> Copy for GenerationID<M> {}

impl<M> Clone for GenerationID<M> {
    fn clone(&self) -> GenerationID<M> {
        *self
    }
}

#[derive(Clone, Debug, PartialEq)]
struct GenerationVec<T> {
    inner: Vec<(usize, Option<T>)>,
}

impl<T> GenerationVec<T> {
    fn new() -> Self {
        GenerationVec { inner: vec![] }
    }
    fn get(&self, id: GenerationID<T>) -> Option<&T> {
        let GenerationID { id, generation, .. } = id;
        if let Some((gen, Some(item))) = self.inner.get(id) {
            if *gen == generation {
                return Some(item);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    fn push(&mut self, item: T) -> GenerationID<T> {
        if let Some((index, (gen, _))) = self
            .inner
            .iter()
            .enumerate()
            .find(|(_i, (_gen, e))| e.is_none())
        {
            let new_gen = gen + 1;
            self.inner[index] = (new_gen, Some(item));
            GenerationID {
                generation: new_gen,
                id: index,
                _marker: Default::default(),
            }
        } else {
            let i = self.inner.len();
            self.inner.push((0, Some(item)));
            GenerationID {
                id: i,
                generation: 0,
                _marker: Default::default(),
            }
        }
    }
    fn remove(&mut self, index: usize) -> Option<T> {
        if let Some((_gen, found)) = self.inner.get_mut(index) {
            found.take()
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct LineType {
    draw_line: fn(f32, f32) -> bool,
}

fn line_type_continous(_position: f32, _scale: f32) -> bool {
    true
}

fn line_type_hidden(position: f32, scale: f32) -> bool {
    (position * scale) as i32 % 2 == 0
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Fixed(u8),
    Full(FullColor),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct FullColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Debug, PartialEq)]
enum Drawable {
    Point(Point),
    Line(Line),
    LineSet(Set),
    NamedGroup(Group),
}

impl Component for Drawable {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
    layer: GenerationID<Layer>,
    color: Color,
    scale: f32,
    linetype: GenerationID<LineType>,
    weight: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct Set {
    inner: Vec<Line>,
}

#[derive(Clone, Debug, PartialEq)]
struct Group {
    inner: Vec<Drawable>,
}

fn make_line(
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
    let start: nPoint<f32, nalgebra::base::dimension::U3> = nPoint::from_slice(&[a.x, a.y, 0.0]);
    let end: nPoint<f32, nalgebra::base::dimension::U3> = nPoint::from_slice(&[b.x, b.y, 0.0]);
    let color = Srgba::new(c.r as f32, c.g as f32, c.b as f32, 1.0);
    debug_lines.add_line(start, end, color);
    (line, debug_lines)
}

fn main() {
    match run_app() {
        Ok(_) => {}
        Err(e) => {
            println!("Application quit with error: {:?}", e);
        }
    }
}

struct SomeState {
    zoom_level: f64,
    domain_h: f64,
    domain_w: f64,
}

impl SomeState {
    fn reset_camera(&mut self, w: &mut World) {
        w.exec(|mut cameras: WriteStorage<Camera>| {
            for cam in (&mut cameras).join() {
                let left = ((self.domain_w * self.zoom_level) / -2.0).trunc() as f32;
                let right = left + (self.domain_w * self.zoom_level) as f32;
                let top = ((self.domain_h * self.zoom_level) / 2.0).trunc() as f32;
                let bottom = top - (self.domain_h * self.zoom_level) as f32;
                let new_cam: Projection =
                    Projection::orthographic(left, right, bottom, top, 10.0, -10.0).into();
                cam.set_projection(new_cam);
            }
        });
    }
}

impl SimpleState for SomeState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.register::<Drawable>();

        data.world.insert(DebugLines::new());
        data.world.insert(DebugLinesParams { line_width: 0.5 });

        let continous_line = LineType {
            draw_line: line_type_continous,
        };
        let hidden_line = LineType {
            draw_line: line_type_hidden,
        };
        let mut line_types = LineTypes::new();
        let linetype_id = line_types.push(continous_line);
        line_types.push(hidden_line);

        data.world.insert(line_types);

        let first_layer = Layer {
            name: "Zero".to_string(),
            color: Color::Full(FullColor { r: 0, g: 0, b: 0 }),
            line_type: linetype_id,
            hidden: false,
            frozen: false,
            locked: false,
        };
        let mut layers = Layers::new();
        let layer_id = layers.push(first_layer);

        data.world.insert(layers);
        for _ in 0..99 {
            let (a, b) = make_line(linetype_id, layer_id);
            data.world.create_entity().with(a).with(b).build();
        }

        if let Some(lays) = data.world.try_fetch::<Layers>() {
            for lay in lays.inner.iter() {
                println!("found layer {:?}.", lay);
            }
        };

        if let Some(typs) = data.world.try_fetch::<LineTypes>() {
            for typ in typs.inner.iter() {
                println!("found linetype {:?}.", typ);
            }
        };

        let mut debug_lines_component = DebugLinesComponent::new();

        let (screen_w, screen_h) = {
            let screen_dimensions = data.world.read_resource::<ScreenDimensions>();
            (screen_dimensions.width(), screen_dimensions.height())
        };

        let grid_h = self.domain_h as u16;
        let grid_w = self.domain_w as u16;

        for y in (0..=grid_h).step_by(50).map(f32::from) {
            debug_lines_component.add_line(
                [0.0, y, 1.0].into(),
                [grid_w as f32, y, 1.0].into(),
                Srgba::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        for x in (0..=grid_w).step_by(50).map(f32::from) {
            debug_lines_component.add_line(
                [x, 0.0, 1.0].into(),
                [x, grid_h as f32, 1.0].into(),
                Srgba::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        data.world
            .create_entity()
            .with(debug_lines_component)
            .build();

        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(
            self.domain_w as f32 / 2.0,
            self.domain_h as f32 / 2.0,
            10.0,
        );
        data.world
            .create_entity()
            .with(Camera::standard_2d(
                screen_w / self.zoom_level as f32,
                screen_h / self.zoom_level as f32,
            ))
            .with(local_transform)
            .build();
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        ev: StateEvent,
    ) -> SimpleTrans {
        let w = data.world;
        match &ev {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                }

                match event {
                    winit::Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::Resized(size) => {
                                let winit::dpi::LogicalSize { width, height } = size;
                                self.domain_w = *width;
                                self.domain_h = *height;
                                self.reset_camera(w);
                            }
                            WindowEvent::MouseWheel { delta, .. } => {
                                use winit::MouseScrollDelta;
                                match delta {
                                    MouseScrollDelta::LineDelta(_x, y) => {
                                        self.zoom_level = if *y > 0.0 {
                                            self.zoom_level / 1.1
                                        } else {
                                            self.zoom_level * 1.1
                                        };
                                        // println!("got mousewheel linedelta of {}", y);
                                        self.reset_camera(w);
                                    }
                                    MouseScrollDelta::PixelDelta(lp) => {
                                        // todo: test this on hardware that triggers this code path?
                                        self.zoom_level = if lp.y > 0.0 {
                                            self.zoom_level * 1.1
                                        } else {
                                            self.zoom_level / 1.1
                                        };
                                        self.reset_camera(w);
                                    }
                                }
                            }
                            _ => {
                                //
                            }
                        }
                    }
                    _ => {
                        //
                    }
                }
            }
            StateEvent::Ui(event) => {
                //uievent
                println!("found input event of type StateEvent::Ui(e)");
            }
            StateEvent::Input(event) => {
                println!("found input event of type StateEvent::Input(e)");
                use amethyst::input::InputEvent;
                use amethyst::input::ScrollDirection;
                match event {
                    InputEvent::MouseWheelMoved(direction) => {
                        match direction {
                            ScrollDirection::ScrollUp => {
                                println!("scroll moved");
                                self.zoom_level = self.zoom_level * 1.1;
                                self.reset_camera(w);
                            }
                            ScrollDirection::ScrollDown => {
                                println!("scroll moved");
                                self.zoom_level = self.zoom_level / 1.1;
                                self.reset_camera(w);
                            }
                            _ => {
                                //
                            }
                        }
                    }
                    _ => {
                        //
                    }
                }
            }
        }
        Trans::None
    }
}

/*
struct LineSyncSystem {}

impl<'a> System<'a> for LineSyncSystem {
    type SystemData = (Write<'a, DebugLines>, ReadStorage<'a, Drawable>);
    fn run(&mut self, data: Self::SystemData) {
        let (debug_lines, drawables) = data;
        for drawable in (drawables).join() {
            //
        }
        unimplemented!()
    }
}
*/
fn run_app() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = amethyst::utils::application_root_dir()?;
    let assets_path = app_root.join("assets/");

    let display_config = DisplayConfig {
        title: "Pretender".to_string(),
        fullscreen: None,
        dimensions: Some((800, 800)),
        min_dimensions: Some((300, 300)),
        max_dimensions: None,
        visibility: true,
        icon: Some(assets_path.join("icon.png")),
        always_on_top: false,
        decorations: true,
        maximized: false,
        multitouch: false,
        resizable: true,
        transparent: false,
        loaded_icon: None,
    };

    let game_data = GameDataBuilder::default()
        // .with(ExampleLinesSystem::new(), "example_lines_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config).with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default()),
        )?;
    let initial_state = SomeState {
        zoom_level: 1.0,
        domain_w: 600.0,
        domain_h: 600.0,
    };

    let mut game = Application::new(app_root, initial_state, game_data)?;

    game.run();

    Ok(())
}
