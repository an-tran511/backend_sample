pub mod uom {
    mod list_uoms_query;
    pub use self::list_uoms_query::{
        ListUomsError, ListUomsQuery, ListUomsQueryHandler, ListUomsResponse,
    };
}

use discern::{query::QueryBus, query_bus};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use uom::{ListUomsQuery, ListUomsQueryHandler};

pub fn create_query_bus(db: Arc<DatabaseConnection>) -> QueryBus {
    query_bus! {
        ListUomsQuery => ListUomsQueryHandler { db },
    }
}
