use axum::{
    extract::{State, Path, Extension},
    routing::{get, post, delete},
    Router, Json,
};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use crate::{
    auth::jwt::Claims,
    errors::AppError,
    models::usuario::CriarUsuario,
};

async fn listar_usuarios(
    State(pool): State<SqlitePool>,
) -> Result<Json<Value>, AppError> {
    let usuarios = sqlx::query!(
        "SELECT id, nome, cargo, ativo FROM usuarios ORDER BY nome"
    )
    .fetch_all(&pool)
    .await?;

    let lista: Vec<_> = usuarios.iter().map(|u| json!({
        "id": u.id,
        "nome": u.nome,
        "cargo": u.cargo,
        "ativo": u.ativo
    })).collect();

    Ok(Json(json!(lista)))
}

async fn criar_usuario(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Json(body): Json<CriarUsuario>,
) -> Result<Json<Value>, AppError> {
    if claims.cargo != "admin" {
        return Err(AppError::Proibido);
    }

    if body.cargo != "funcionaria" && body.cargo != "admin" {
        return Err(AppError::BadRequest("Cargo inválido".to_string()));
    }

    let pin_hash = bcrypt::hash(&body.pin, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let result = sqlx::query!(
        "INSERT INTO usuarios (nome, pin, cargo) VALUES (?, ?, ?) RETURNING id",
        body.nome,
        pin_hash,
        body.cargo
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(json!({
        "id": result.id,
        "nome": body.nome,
        "cargo": body.cargo
    })))
}

async fn deletar_usuario(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, AppError> {
    if claims.cargo != "admin" {
        return Err(AppError::Proibido);
    }

    sqlx::query!("UPDATE usuarios SET ativo = 0 WHERE id = ?", id)
        .execute(&pool)
        .await?;

    Ok(Json(json!({ "mensagem": "Usuário desativado" })))
}

pub fn rotas(pool: SqlitePool) -> Router {
    Router::new()
        .route("/usuarios", get(listar_usuarios).post(criar_usuario))
        .route("/usuarios/:id", delete(deletar_usuario))
        .with_state(pool)
}
