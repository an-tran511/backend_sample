use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use discern::{
    async_trait,
    command::{Command, CommandHandler},
};
use entity::uom;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct CreateUomCommand {
    pub name: String,
}

#[derive(Debug, Error)]
pub enum CreateUomError {
    #[error("internal_server_error")]
    InternalServerError(#[from] DbErr),
}

impl IntoResponse for CreateUomError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            CreateUomError::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        (
            status,
            Json(json!({
              "ok": false,
              "code": code,
            })),
        )
            .into_response()
    }
}

impl Command for CreateUomCommand {
    type Metadata = uom::Model;
    type Error = CreateUomError;
}

pub struct CreateUomCommandHandler {
    pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl CommandHandler<CreateUomCommand> for CreateUomCommandHandler {
    async fn handle(&self, command: CreateUomCommand) -> Result<uom::Model, CreateUomError> {
        let uom = uom::ActiveModel {
            name: Set(command.name.to_owned()),
            ..Default::default()
        };

        let uom = match uom.insert(self.db.as_ref()).await {
            Ok(uom) => uom,
            Err(error) => return Err(CreateUomError::InternalServerError(error)),
        };

        Ok(uom)
    }
}
