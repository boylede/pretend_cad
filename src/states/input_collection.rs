use amethyst::{
    assets::{
        AssetStorage, Loader,
    },
    ecs::prelude::*,
    input::{is_close_requested},
    prelude::*,
    ui::{
        get_default_font, Anchor, FontAsset, TextEditing,
         UiText, UiTransform,
    },

};

use crate::{
    resources::{
        CommandList, InputDesc, CapturedInput, CommandDesc
    },
    common::reset_camera,
};


pub struct InputCollectionState {
    pub command: CommandDesc,
    pub current_input: usize,
    pub found_inputs: Vec<CapturedInput>,
}

impl SimpleState for InputCollectionState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        ev: StateEvent,
    ) -> SimpleTrans {
        let w = data.world;
        match &ev {
            StateEvent::Window(_event) => (),
            StateEvent::Ui(_event) => (),
            StateEvent::Input(_event) => (),
        }
        Trans::None
    }
}