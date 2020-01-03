use specs::prelude::*;
use rand::prelude::*;
use amethyst::{
    core::transform::{Transform, TransformBundle},
    prelude::*,
    window::{DisplayConfig, ScreenDimensions},
    renderer::{
        RenderingBundle,
        camera::Camera,
        types::DefaultBackend,
        palette::Srgba,
        debug_drawing::{DebugLine, DebugLines, DebugLinesComponent, DebugLinesParams},
        plugins::{
            RenderDebugLines,
            RenderToWindow
        },
        rendy::{
            mesh::{Color as RendyColor, Position},
            util::types::vertex::PosColor,
            
        }
    },
};

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

impl<M> Copy for GenerationID<M> { }

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
        GenerationVec {
            inner: vec![],
        }
    }
    fn get(&self, id: GenerationID<T>) -> Option<&T> {
        let GenerationID{id, generation, ..} = id;
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
        if let Some((index, (gen, _))) = self.inner.iter().enumerate().find(|(i, (gen, e))| e.is_none()) {
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

fn make_line(style: GenerationID<LineType>, layer: GenerationID<Layer>) -> (Drawable, DebugLinesComponent) {
    let mut rng = rand::thread_rng();
    let a = Point {
        x: rng.gen_range(0, 10) as f32,
        y: rng.gen_range(0, 10) as f32
    };
    let b = Point {
        x: rng.gen_range(0, 10) as f32,
        y: rng.gen_range(0, 10) as f32
    };
    let c = FullColor{r: 234, g: 65, b: 212};
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
    let color = Srgba::new(c.r as f32, c.g  as f32, c.b  as f32, 1.0);
    debug_lines.add_line(
        start,
        end,
        color,
    );
    (line, debug_lines)
}

fn main() {
    run_app();
}

struct SomeState {
    zoom_level: f32,
    domain_h: f32,
    domain_w: f32,
}

impl SimpleState for SomeState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        data.world.register::<Drawable>();
        
        data.world.insert(DebugLines::new());
        data.world.insert(DebugLinesParams { line_width: 2.0 });

        let continous_line = LineType {draw_line: line_type_continous};
        let hidden_line = LineType {draw_line: line_type_hidden};
        let mut line_types = LineTypes::new();
        let linetype_id = line_types.push(continous_line);
        line_types.push(hidden_line);

        data.world.insert(line_types);

        let first_layer = Layer {
            name: "Zero".to_string(),
            color: Color::Full(FullColor{r: 0, g: 0, b: 0}),
            line_type: linetype_id,
            hidden: false,
            frozen: false,
            locked: false,
        };
        let mut layers = Layers::new();
        let layer_id = layers.push(first_layer);

        data.world.insert(layers);
        for _ in 0..999 {
            let (a, b) = make_line(linetype_id, layer_id);
            data.world.create_entity()
            .with(a)
            .with(b)
            .build();
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

        for y in (0..grid_h).step_by(50).map(f32::from) {
            debug_lines_component.add_line(
                [0.0, y, 1.0].into(),
                [grid_w as f32, y, 1.0].into(),
                Srgba::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        for x in (0..grid_w).step_by(50).map(f32::from) {
            debug_lines_component.add_line(
                [x, 0.0, 1.0].into(),
                [x, grid_h as f32, 1.0].into(),
                Srgba::new(0.3, 0.3, 0.3, 1.0),
            );
        }

        debug_lines_component.add_line(
            [20.0, 20.0, 1.0].into(),
            [780.0, 580.0, 1.0].into(),
            Srgba::new(1.0, 0.0, 0.2, 1.0), // Red
        );

        data.world
            .create_entity()
            .with(debug_lines_component)
            .build();

        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(self.domain_w / 2.0, self.domain_h / 2.0, 10.0);
        data.world
            .create_entity()
            .with(Camera::standard_2d(screen_w / self.zoom_level, screen_h / self.zoom_level))
            .with(local_transform)
            .build();
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

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


fn run_app() ->  amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = amethyst::utils::application_root_dir()?;

    let display_config = DisplayConfig {
        title: "Pretender".to_string(),
        fullscreen: None,
        dimensions: Some((800,800)),
        min_dimensions: Some((300,300)),
        max_dimensions: None,
        visibility: true,
        icon: None,
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
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default()),
        )?;
    let initial_state = SomeState {
        zoom_level: 0.5,
        domain_w: 200.0,
        domain_h: 200.0,
    };

    let mut game = Application::new(app_root, initial_state, game_data)?;

    game.run();


    Ok(())
}