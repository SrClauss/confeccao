CREATE TABLE IF NOT EXISTS usuarios (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    nome      TEXT NOT NULL,
    pin       TEXT NOT NULL,
    cargo     TEXT NOT NULL CHECK(cargo IN ('funcionaria', 'admin')),
    ativo     INTEGER NOT NULL DEFAULT 1,
    criado_em TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Cria admin padrão com PIN 0000 (hash bcrypt)
INSERT OR IGNORE INTO usuarios (nome, pin, cargo) VALUES ('Admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBpj2lMmJA/pXa', 'admin');
