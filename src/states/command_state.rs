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
use winit::WindowEvent;
use std::fmt::Write;

use crate::resources::{CommandList, Command};

pub struct CommandEntryState {
    pub command: String,
}

impl SimpleState for CommandEntryState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        //
        // println!("started entering command with {}", self.command);
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        ev: StateEvent,
    ) -> SimpleTrans {
        let w = data.world;
        match &ev {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    return Trans::Quit;
                }
                match event {
                    winit::Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::KeyboardInput { input, .. } => {
                                let (keycode, state) = (input.virtual_keycode, input.state);
                                if let Some(key) = keycode {
                                    use crate::common::{as_alphanumeric, is_confirmation};
                                    if let Some(letter) = as_alphanumeric(key) {
                                        use winit::ElementState::*;
                                        match state {
                                            Pressed => {
                                                self.command.write_char(letter);
                                                // println!("command is {}", self.command);
                                            },
                                            Released => (),
                                        }
                                    }
                                    if let Some(activate) = is_confirmation(key) {
                                        println!("command: {}", self.command);
                                        use winit::ElementState::*;
                                        match state {
                                            Pressed => {
                                                if activate {
                                                    return interpret_command(w, &self.command);
                                                } else {
                                                    // println!("cancelled command");
                                                    return Trans::Pop;
                                                }
                                            },
                                            Released => (),
                                        }
                                        
                                    }
                                }
                            }
                            _ => ()
                        }
                    }
                    _ => ()
                }
            }
            StateEvent::Ui(event) => (),
            StateEvent::Input(event) => (),
        }
        Trans::None
    }
}

fn interpret_command(w: &mut World, name: &String) -> SimpleTrans {
    let command;
    {
        let commands = w.read_resource::<CommandList>();
        command = commands.get(name).cloned();
    }
    
    if let Some(func) = command {
        return func(w);
    }
    return Trans::Pop;
}
