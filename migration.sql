DROP TABLE IF EXISTS apps;
CREATE TABLE apps (
    id INTEGER PRIMARY KEY,
    created_at DATETIME,
    updated_at DATETIME,
    name TEXT,
    description TEXT,
    model_name TEXT,
    model_max_tokens INTEGER,
    model_temperature REAL,
    model_repeat_penalty REAL,
    model_repeat_penalty_last_n_tokens INTEGER,
    model_top_k INTEGER,
    model_top_p REAL
);

DROP TABLE IF EXISTS messages;
CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    app_id INTEGER,
    role TEXT,
    content TEXT,
    FOREIGN KEY (app_id) REFERENCES apps(id)
);
