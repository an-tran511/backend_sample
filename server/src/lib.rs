use axum::Router;
use def::state::AppState;
use route::uom_route;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

async fn connect_db() -> Result<DatabaseConnection, sea_orm::DbErr> {
    tracing::info!("Connecting to database");
    let mut opt = ConnectOptions::new(&env::var("DATABASE_URL").unwrap());
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;
    Ok(db)
}

#[tokio::main]
pub async fn start() {
    dotenvy::dotenv().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db = match connect_db().await {
        Ok(db) => {
            tracing::info!("Connected to database");
            db
        }
        Err(_) => {
            tracing::error!("Failed to connect to database");
            std::process::exit(1);
        }
    };

    let state = Arc::new(AppState { db });

    let router = Router::new()
        .nest("/api", Router::new().merge(uom_route::new()))
        .with_state(state.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("Listening on http://{}", addr);
    axum::serve(tcp, router).await.unwrap();
}
