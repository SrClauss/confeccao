use axum::{
    extract::Request,
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use crate::{auth::jwt::validar_token, errors::AppError};

pub async fn autenticar(
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::NaoAutorizado)?;

    let claims = validar_token(token).map_err(|_| AppError::NaoAutorizado)?;
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

pub async fn requer_admin(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = req
        .extensions()
        .get::<crate::auth::jwt::Claims>()
        .ok_or(AppError::NaoAutorizado)?;

    if claims.cargo != "admin" {
        return Err(AppError::Proibido);
    }
    Ok(next.run(req).await)
}
