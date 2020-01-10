use amethyst::{
    assets::{
        AssetPrefab, AssetStorage, Format, Handle, Loader, Prefab, PrefabData, PrefabLoaderSystem,
        PrefabLoaderSystemDesc, Progress, ProgressCounter,
    },
    core::transform::Transform,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
    ui::{
        get_default_font, Anchor, FontAsset, RenderUi, Stretch, TextEditing, UiBundle, UiCreator,
        UiEvent, UiFinder, UiText, UiTransform,
    },
    window::ScreenDimensions,
};

use std::fmt::Write;
use winit::WindowEvent;

use crate::resources::{Command, CommandList};

pub struct CommandEntryState {
    pub command: String,
    pub command_ui: Option<Entity>,
}

impl SimpleState for CommandEntryState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let font = {
            let loader = world.read_resource::<Loader>();
            let font_store = world.read_resource::<AssetStorage<FontAsset>>();
            get_default_font(&loader, &font_store)
        };
        let command_entity = world
        .create_entity()
        .with(UiText::new(
            font,
            "command:> ".to_string(),
            [0.5, 0.5, 0.5, 1.0],
            20.0,
        ))
        .with(TextEditing::new(
            100,
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 0.5, 0.5, 1.0],
            false,
        ))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::BottomMiddle,
            Anchor::BottomMiddle,
            // Stretch::NoStretch,
            0.0,
            0.0,
            0.0,
            400.0,
            40.0,
        ))
        .build();
        self.command_ui = Some(command_entity);
        
        world.exec(|mut ui_text: WriteStorage<UiText>| {
            //UiText
            let text = ui_text.get_mut(command_entity).expect("failed to find UiText");
            text.text.push_str(&self.command);
        });
        
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(command_ui) = self.command_ui {
            data.world.delete_entity(command_ui);
        }
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
                                                if let Some(ui) = self.command_ui {
                                                    w.exec(|mut ui_text: WriteStorage<UiText>| {
                                                        //UiText
                                                        let text = ui_text.get_mut(ui).expect("failed to find UiText");
                                                        text.text.push(letter);
                                                    });
                                                }
                                            }
                                            Released => (),
                                        }
                                    }
                                    if let Some(activate) = is_confirmation(key) {
                                        // println!("command: {}", self.command);
                                        use winit::ElementState::*;
                                        match state {
                                            Pressed => {
                                                if activate {
                                                    return interpret_command(w, &self.command);
                                                } else {
                                                    // println!("cancelled command");
                                                    return Trans::Pop;
                                                }
                                            }
                                            Released => (),
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
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
