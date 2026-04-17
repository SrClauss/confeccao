CREATE TABLE IF NOT EXISTS cortes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    numero      INTEGER NOT NULL,
    data        TEXT NOT NULL,
    tamanho     TEXT NOT NULL CHECK(tamanho IN ('P','M','G','GG','EXG','EXGG')),
    rendimento  REAL NOT NULL,
    divisor     REAL NOT NULL,
    total_pecas INTEGER NOT NULL DEFAULT 0,
    observacao  TEXT,
    criado_em   TEXT NOT NULL DEFAULT (datetime('now'))
);
