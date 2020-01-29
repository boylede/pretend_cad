use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        camera::Camera,
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
    // window::ScreenDimensions,
};
use winit::WindowEvent;

use crate::{
    commands,
    common::reset_camera,
    components::{ActiveCamera, Color, Drawable, FullColor},
    resources::{Layer, Layers, LineType, LineTypes, ViewInfo},
    states::{CommandEntryState, PanState},
};

pub struct RootState {
    pub cursor: (f64, f64),
}

impl SimpleState for RootState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let w = data.world;
        w.register::<Drawable>();
        w.register::<ActiveCamera>();

        w.insert(DebugLines::new());
        w.insert(DebugLinesParams { line_width: 0.5 });
        w.insert(ViewInfo::default());

        let continous_line = LineType {
            draw_line: LineType::line_type_continous,
        };
        let hidden_line = LineType {
            draw_line: LineType::line_type_hidden,
        };
        let mut line_types = LineTypes::new();
        let linetype_id = line_types.push(continous_line);
        line_types.push(hidden_line);

        w.insert(line_types);

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

        w.insert(layers);
        // for _ in 0..99 {
        //     let (a, b) = Line::create(linetype_id, layer_id);
        //     w.create_entity().with(a).with(b).build();
        // }
        let commands = commands::register_commands();
        w.insert(commands);

        let mut debug_lines_component = DebugLinesComponent::new();

        // let (screen_w, screen_h) = {
        //     let screen_dimensions = w.read_resource::<ScreenDimensions>();
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

        w.create_entity().with(debug_lines_component).build();

        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(10.0 / 2.0, 10.0 / 2.0, 10.0);
        w.create_entity()
            .with(Camera::standard_2d(1.0, 1.0))
            .with(local_transform)
            .with(ActiveCamera)
            .build();
        reset_camera(w);
    }

    // fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    // note: needs to call data.data.update(&mut w)
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
                                    view_info.resize(*width, *height);
                                }
                                reset_camera(w);
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
                                            view_info.zoom(*y);
                                        }
                                        // println!("got mousewheel linedelta of {}", y);
                                        reset_camera(w);
                                    }
                                    MouseScrollDelta::PixelDelta(lp) => {
                                        println!("got mousewheel pixeldelta of {}", lp.y);
                                        // todo: test this on hardware that triggers this code path?
                                        panic!("zooming with pixeldelta is unimplemented");
                                    }
                                }
                            }
                            WindowEvent::MouseInput { state, button, .. } => {
                                use winit::ElementState::*;
                                use winit::MouseButton::*;
                                match (button, state) {
                                    (Middle, Pressed) => {
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
                                    view_info.zoom(1)
                                }
                                reset_camera(w);
                            }
                            ScrollDirection::ScrollDown => {
                                // println!("scroll moved");
                                {
                                    let mut view_info = w.write_resource::<ViewInfo>();
                                    view_info.zoom(-1.0)
                                }
                                reset_camera(w);
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
