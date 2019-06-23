use actix::prelude::*;

use hoo_api::light::{LightNumber, LightState};
use hoo_api::ApiConnection;

#[derive(Debug, Message)]
struct SetLightState {
    pub light_number: LightNumber,
    pub state: LightState,
}

struct LightController {
    connection: ApiConnection,
}

impl LightController {
    fn new(connection: ApiConnection) -> Self {
        Self { connection }
    }
}

impl Actor for LightController {
    type Context = Context<Self>;
}

impl Handler<SetLightState> for LightController {
    type Result = ();

    fn handle(&mut self, msg: SetLightState, ctx: &mut Context<Self>) {
        self.connection.set_state(msg.light_number, &msg.state);
    }
}
