use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::camera::{Camera},
};
use specs::prelude::*;
use winit::WindowEvent;

use crate::{resources::ViewInfo, components::ActiveCamera, common::ScreenTranslation};

pub struct PanState {
    pub initial: (f64, f64),
    pub last: (f64, f64),
}

impl PanState {
    pub fn new(initial: (f64, f64)) -> Self {
        PanState {
            initial,
            last: initial,
        }
    }
    fn move_camera(&mut self, w: &mut World, dx: f32, dy: f32) {
        w.exec(|(mut cameras, active_cameras, mut view_info): (WriteStorage<Camera>, ReadStorage<ActiveCamera>, WriteExpect<ViewInfo>)| {
            view_info.pan(ScreenTranslation{dx, dy});
            for (cam, _) in (&mut cameras, &active_cameras).join() {
                cam.set_projection(view_info.projection());
            }
        });
    }
}

impl SimpleState for PanState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        //
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
                            WindowEvent::Resized(_size) => {
                                // shouldnt be possible?
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                let (x, y) = (position.x, position.y);
                                let dx = x - self.last.0;
                                let dy = y - self.last.1;
                                // println!("dragged to {},{}", dx, dy);
                                self.last.0 = x;
                                self.last.1 = y;
                                self.move_camera(w, dx as f32, dy as f32);
                            }
                            WindowEvent::MouseWheel { .. } => {
                                // unexpected
                            }
                            WindowEvent::MouseInput { state, button, .. } => {
                                use winit::ElementState::*;
                                use winit::MouseButton::*;
                                match (button, state) {
                                    (Middle, Released) => {
                                        // println!("ended drag, mouse moved {}, {}", dx, dy);
                                        return Trans::Pop;
                                    }
                                    _ => {}
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
            }
            StateEvent::Input(_event) => {}
        }
        Trans::None
    }
}
