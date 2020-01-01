use legion::prelude::*;
use rand::prelude::*;


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

fn make_line(style: GenerationID<LineType>, layer: GenerationID<Layer>) -> Drawable {
    let mut rng = rand::thread_rng();
    Drawable::Line(Line {
        start: Point {
            x: rng.gen_range(0, 10) as f32,
            y: rng.gen_range(0, 10) as f32
        },
        end: Point {
            x: rng.gen_range(0, 10) as f32,
            y: rng.gen_range(0, 10) as f32
        },
        layer: layer,
        color: Color::Full(FullColor{r: 234, g: 65, b: 212}),
        scale: 1.0,
        linetype: style,
        weight: 1.0,
    })
}

fn main() {
    let universe = Universe::new();
    let mut world = universe.create_world();
    
    let continous_line = LineType {draw_line: line_type_continous};
    let hidden_line = LineType {draw_line: line_type_hidden};
    let mut line_types = LineTypes::new();
    let linetype_id = line_types.push(continous_line);
    line_types.push(hidden_line);

    world.resources.insert(line_types);

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

    world.resources.insert(layers);
    
    world.insert(
        (),
        (0..999).map(|_| (make_line(linetype_id, layer_id), 0))
    );
    
    if let Some(lays) = world.resources.get::<Layers>() {
        for lay in lays.inner.iter() {
            println!("found layer {:?}.", lay);
        }
    };

    if let Some(typs) = world.resources.get::<LineTypes>() {
        for typ in typs.inner.iter() {
            println!("found linetype {:?}.", typ);
        }
    };
    

}
