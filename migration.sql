DROP TABLE IF EXISTS apps;

CREATE TABLE apps (
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    system_prompt TEXT,
    model_name TEXT NOT NULL,
    model_max_tokens INTEGER,
    model_temperature REAL,
    model_repeat_penalty REAL,
    model_repeat_penalty_last_n_tokens INTEGER,
    model_top_k INTEGER,
    model_top_p REAL
);

CREATE TRIGGER apps_update_updated_at_trigger
AFTER UPDATE ON apps FOR EACH ROW
WHEN OLD.updated_at = NEW.updated_at OR OLD.updated_at IS NULL
BEGIN
   UPDATE apps SET updated_at = CURRENT_TIMESTAMP WHERE name = NEW.name;
END;

INSERT INTO apps (
    'name',
    'description',
    'model_name',
    'model_max_tokens',
    'model_temperature',
    'model_repeat_penalty',
    'model_repeat_penalty_last_n_tokens',
    'model_top_k',
    'model_top_p'
) VALUES (
    'my-first-prompt-app',
    'This is my first prompt app!',
    'llama2-chat',
    75,
    0.0,
    1.1,
    64,
    40,
    0.9
);
