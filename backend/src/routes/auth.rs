use axum::{extract::State, routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use crate::{errors::AppError, models::usuario::UsuarioPublico};

#[derive(Deserialize)]
struct LoginBody {
    pin: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    usuario: UsuarioPublico,
}

async fn login(
    State(pool): State<SqlitePool>,
    Json(body): Json<LoginBody>,
) -> Result<Json<Value>, AppError> {
    let usuarios = sqlx::query!(
        "SELECT id, nome, pin, cargo FROM usuarios WHERE ativo = 1"
    )
    .fetch_all(&pool)
    .await?;

    for u in usuarios {
        if bcrypt::verify(&body.pin, &u.pin).unwrap_or(false) {
            let token = crate::auth::jwt::criar_token(u.id, &u.nome, &u.cargo)
                .map_err(|e| AppError::Internal(e.to_string()))?;

            return Ok(Json(json!({
                "token": token,
                "usuario": {
                    "id": u.id,
                    "nome": u.nome,
                    "cargo": u.cargo
                }
            })));
        }
    }

    Err(AppError::NaoAutorizado)
}

async fn me(
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::jwt::Claims>,
) -> Json<Value> {
    Json(json!({
        "id": claims.sub,
        "nome": claims.nome,
        "cargo": claims.cargo
    }))
}

pub fn rotas(pool: SqlitePool) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/me", axum::routing::get(me)
            .layer(axum::middleware::from_fn(crate::auth::middleware::autenticar)))
        .with_state(pool)
}
