use amethyst::prelude::*;

pub struct CommandEntryState {
    pub command: String,
}

impl SimpleState for CommandEntryState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        //
        println!("started entering command with {}", self.command);
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        ev: StateEvent,
    ) -> SimpleTrans {
        let _w = data.world;
        match &ev {
            _ => Trans::None,
        }
    }
}
