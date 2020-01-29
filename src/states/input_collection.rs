use amethyst::prelude::*;

use crate::resources::{CapturedInput, CommandDesc};

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