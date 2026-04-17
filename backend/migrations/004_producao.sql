CREATE TABLE IF NOT EXISTS producao (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    funcionaria_id INTEGER NOT NULL REFERENCES usuarios(id),
    corte_id       INTEGER NOT NULL REFERENCES cortes(id),
    linha_corte_id INTEGER NOT NULL REFERENCES linhas_corte(id),
    operacao       TEXT NOT NULL,
    quantidade     INTEGER NOT NULL,
    data_hora      TEXT NOT NULL DEFAULT (datetime('now'))
);
