use discern::query::QueryBus;

#[derive(Clone)]
pub struct AppState {
    pub query_bus: QueryBus,
}
