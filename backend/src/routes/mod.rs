use axum::{Router, middleware};
use sqlx::SqlitePool;

pub mod auth;
pub mod cortes;
pub mod producao;
pub mod dashboard;
pub mod usuarios;

pub fn create_router(pool: SqlitePool) -> Router {
    let rotas_publicas = Router::new()
        .merge(auth::rotas(pool.clone()));

    let rotas_autenticadas = Router::new()
        .merge(cortes::rotas(pool.clone()))
        .merge(producao::rotas(pool.clone()))
        .merge(dashboard::rotas(pool.clone()))
        .merge(usuarios::rotas(pool.clone()))
        .layer(middleware::from_fn(crate::auth::middleware::autenticar));

    Router::new()
        .merge(rotas_publicas)
        .merge(rotas_autenticadas)
}
