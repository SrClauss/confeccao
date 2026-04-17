use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

pub enum AppError {
    NaoAutorizado,
    Proibido,
    NaoEncontrado,
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, mensagem) = match self {
            AppError::NaoAutorizado => (StatusCode::UNAUTHORIZED, "Não autorizado".to_string()),
            AppError::Proibido => (StatusCode::FORBIDDEN, "Acesso proibido".to_string()),
            AppError::NaoEncontrado => (StatusCode::NOT_FOUND, "Recurso não encontrado".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({ "erro": mensagem }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Erro de banco de dados: {}", e);
        AppError::Internal(format!("Erro de banco de dados: {}", e))
    }
}
