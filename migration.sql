CREATE TABLE IF NOT EXISTS app (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    system_prompt TEXT NOT NULL,
    user_prompt TEXT NOT NULL,
    model_name TEXT NOT NULL,
    model_max_tokens INTEGER NOT NULL,
    model_temperature REAL NOT NULL,
    model_repeat_penalty REAL NOT NULL,
    model_repeat_penalty_last_n_tokens INTEGER NOT NULL,
    model_top_k INTEGER NOT NULL,
    model_top_p REAL NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);
