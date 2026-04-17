pub fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "segredo_padrao_inseguro".to_string())
}
