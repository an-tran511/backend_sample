pub mod uom {
    mod create_uom_command;
    pub use self::create_uom_command::{CreateUomCommand, CreateUomCommandHandler, CreateUomError};
}

use discern::{command::CommandBus, command_bus};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use uom::{CreateUomCommand, CreateUomCommandHandler};

pub fn create_command_bus(db: Arc<DatabaseConnection>) -> CommandBus {
    command_bus! {
        CreateUomCommand => CreateUomCommandHandler { db },
    }
}
