use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
    // window::ScreenDimensions,
};
use specs::prelude::*;
use winit::WindowEvent;

use crate::{
    commands,
    components::{Color, Drawable, FullColor, ActiveCamera},
    resources::{ Layer, Layers, LineType, LineTypes, ViewInfo},
    states::{CommandEntryState, PanState},
};

pub struct RootState {
    // pub zoom_level: f64,
    // pub origin_x: f64,
    // pub origin_y: f64,
    // pub domain_h: f64,
    // pub domain_w: f64,
    pub cursor: (f64, f64),
}

impl RootState {
    fn reset_camera(&mut self, w: &mut World) {
        w.exec(|(mut cameras, view_info): (WriteStorage<Camera>, ReadExpect<ViewInfo>)| {
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

impl SimpleState for RootState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.register::<Drawable>();
        data.world.register::<ActiveCamera>();

        data.world.insert(DebugLines::new());
        data.world.insert(DebugLinesParams { line_width: 0.5 });
        data.world.insert(ViewInfo::default());

        let continous_line = LineType {
            draw_line: LineType::line_type_continous,
        };
        let hidden_line = LineType {
            draw_line: LineType::line_type_hidden,
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
        let _layer_id = layers.push(first_layer);

        data.world.insert(layers);
        // for _ in 0..99 {
        //     let (a, b) = Line::create(linetype_id, layer_id);
        //     data.world.create_entity().with(a).with(b).build();
        // }
        let commands = commands::register_commands();
        data.world.insert(commands);

        let mut debug_lines_component = DebugLinesComponent::new();

        // let (screen_w, screen_h) = {
        //     let screen_dimensions = data.world.read_resource::<ScreenDimensions>();
        //     (screen_dimensions.width(), screen_dimensions.height())
        // };

        // let ()

        let grid_h: u16 = 1000;
        let grid_w: u16 = 1000;

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
            10.0 / 2.0,
            10.0 / 2.0,
            10.0,
        );
        data.world
            .create_entity()
            .with(Camera::standard_2d(
                1.0,
                1.0,
            ))
            .with(local_transform)
            .with(ActiveCamera)
            .build();
        self.reset_camera(data.world);
    }

    // fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    // note: needs to call data.data.update(&mut data.world)
    //     Trans::None
    // }
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
                                {
                                    let mut view_info = w.write_resource::<ViewInfo>();
                                    view_info.domain_w = *width * view_info.zoom_level;
                                    view_info.domain_h = *height * view_info.zoom_level;
                                }
                                self.reset_camera(w);
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                self.cursor.0 = position.x;
                                self.cursor.1 = position.y;
                            }
                            WindowEvent::MouseWheel { delta, .. } => {
                                use winit::MouseScrollDelta;
                                match delta {
                                    MouseScrollDelta::LineDelta(_x, y) => {
                                        {
                                            let mut view_info = w.write_resource::<ViewInfo>();
                                            if *y > 0.0 {
                                                // println!("zooming in");
                                                view_info.zoom_level = view_info.zoom_level / 1.1;
                                                view_info.domain_h = view_info.domain_h / 1.1;
                                                view_info.domain_w = view_info.domain_w / 1.1;
                                            } else {
                                                // println!("zooming out");
                                                view_info.zoom_level = view_info.zoom_level * 1.1;
                                                view_info.domain_h = view_info.domain_h * 1.1;
                                                view_info.domain_w = view_info.domain_w * 1.1;
                                            };
                                        }
                                        // println!("got mousewheel linedelta of {}", y);
                                        self.reset_camera(w);
                                    }
                                    MouseScrollDelta::PixelDelta(lp) => {
                                        println!("got mousewheel pixeldelta of {}", lp.y);
                                        {
                                            let mut view_info = w.write_resource::<ViewInfo>();
                                        // todo: test this on hardware that triggers this code path?
                                            view_info.zoom_level = if lp.y > 0.0 {
                                                view_info.zoom_level * 1.1
                                            } else {
                                                view_info.zoom_level / 1.1
                                            };
                                        }
                                        self.reset_camera(w);
                                    }
                                }
                            }
                            WindowEvent::MouseInput { state, button, .. } => {
                                use winit::ElementState::*;
                                use winit::MouseButton::*;
                                match (button, state) {
                                    (Middle, Pressed) => {
                                        // w.exec(|position: WriteStorage<Camera>| {});
                                        let pan_state = PanState::new(self.cursor);
                                        return Trans::Push(Box::new(pan_state));
                                    }
                                    _ => {}
                                }
                            }
                            WindowEvent::KeyboardInput { input, .. } => {
                                let (keycode, state) = (input.virtual_keycode, input.state);
                                if let Some(key) = keycode {
                                    use crate::common::as_alphanumeric;
                                    if let Some(letter) = as_alphanumeric(key) {
                                        use winit::ElementState::*;
                                        match state {
                                            Pressed => {
                                                let command_state = CommandEntryState {
                                                    command: letter.to_string(),
                                                    command_ui: None,
                                                };
                                                return Trans::Push(Box::new(command_state));
                                            }
                                            Released => {
                                                //
                                            }
                                        }
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
            StateEvent::Ui(_event) => {
                //uievent

                // println!("found input event of type StateEvent::Ui({:?})", event.event_type);
            }
            StateEvent::Input(event) => {
                // println!("found input event of type StateEvent::Input({:?})", event);
                use amethyst::input::InputEvent;
                use amethyst::input::ScrollDirection;
                match event {
                    InputEvent::MouseWheelMoved(direction) => {
                        
                        match direction {
                            ScrollDirection::ScrollUp => {
                                // println!("scroll moved");
                                {
                                    let mut view_info = w.write_resource::<ViewInfo>();
                                    view_info.zoom_level = view_info.zoom_level * 1.1;
                                }
                                self.reset_camera(w);
                            }
                            ScrollDirection::ScrollDown => {
                                // println!("scroll moved");
                                {
                                    let mut view_info = w.write_resource::<ViewInfo>();
                                    view_info.zoom_level = view_info.zoom_level / 1.1;
                                }
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
