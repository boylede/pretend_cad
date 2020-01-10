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
use specs::prelude::*;
use winit::WindowEvent;

use crate::{
    components::{Color, Drawable, FullColor, Line},
    resources::{Layer, Layers, LineType, LineTypes, CommandList},
    states::{CommandEntryState, PanState},
    commands,
};

use std::collections::HashMap;

pub struct RootState {
    pub zoom_level: f64,
    pub domain_h: f64,
    pub domain_w: f64,
    pub cursor: (f64, f64),
}

impl RootState {
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

impl SimpleState for RootState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.register::<Drawable>();

        data.world.insert(DebugLines::new());
        data.world.insert(DebugLinesParams { line_width: 0.5 });

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
        let layer_id = layers.push(first_layer);

        data.world.insert(layers);
        // for _ in 0..99 {
        //     let (a, b) = Line::create(linetype_id, layer_id);
        //     data.world.create_entity().with(a).with(b).build();
        // }
        let commands = commands::register_commands();
        data.world.insert(commands);

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
                                self.domain_w = *width;
                                self.domain_h = *height;
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
                            WindowEvent::MouseInput { state, button, .. } => {
                                use winit::ElementState::*;
                                use winit::MouseButton::*;
                                match (button, state) {
                                    (Middle, Pressed) => {
                                        // w.exec(|position: WriteStorage<Camera>| {});
                                        let pan_state = PanState {
                                            initial: self.cursor,
                                            last: self.cursor,
                                        };
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
