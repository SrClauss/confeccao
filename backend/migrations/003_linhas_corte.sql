CREATE TABLE IF NOT EXISTS linhas_corte (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    corte_id    INTEGER NOT NULL REFERENCES cortes(id) ON DELETE CASCADE,
    num         INTEGER NOT NULL,
    n_folhas    REAL NOT NULL,
    a_mais      REAL NOT NULL DEFAULT 0,
    cor         TEXT NOT NULL,
    total       INTEGER NOT NULL,
    soma_a_g    INTEGER NOT NULL,
    total_geral INTEGER NOT NULL
);
