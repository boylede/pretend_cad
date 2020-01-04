use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::camera::{Camera, Orthographic, Projection},
};
use specs::prelude::*;
use winit::WindowEvent;

pub struct PanState {
    pub initial: (f64, f64),
    pub last: (f64, f64),
}

fn move_camera(w: &mut World, dx: f64, dy: f64) {
    w.exec(|mut cameras: WriteStorage<Camera>| {
        for cam in (&mut cameras).join() {
            let p = cam.projection();
            let left;
            let right;
            let top;
            let bottom;
            match p {
                Projection::Orthographic(o) => {
                    // println!("moving camera {},{}", dx, dy);
                    left = o.left() - dx as f32;
                    right = o.right() - dx as f32;
                    top = o.top() - dy as f32;
                    bottom = o.bottom() - dy as f32;
                    // o.set_bottom_and_top(bottom, top);
                    // o.set_left_and_right(left, right);
                }
                _ => unimplemented!(),
            }
            cam.set_projection(Projection::Orthographic(Orthographic::new(
                left, right, bottom, top, 10.0, -10.0,
            )));
        }
    });
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
                                move_camera(w, dx, dy);
                            }
                            WindowEvent::MouseWheel { .. } => {
                                // unexpected
                            }
                            WindowEvent::MouseInput { state, button, .. } => {
                                use winit::ElementState::*;
                                use winit::MouseButton::*;
                                match (button, state) {
                                    (Middle, Released) => {
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
