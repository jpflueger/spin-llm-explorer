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
    'system_prompt',
    'model_name',
    'model_max_tokens',
    'model_temperature',
    'model_repeat_penalty',
    'model_repeat_penalty_last_n_tokens',
    'model_top_k',
    'model_top_p'
) VALUES (
    'basic-chat',
    'A basic chat assistant with default values for all parameters.',
    'You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe. Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don''t know the answer to a question, please don''t share false information.',
    'llama2-chat',
    100,
    0.8,
    1.1,
    64,
    40,
    0.9
);

INSERT INTO apps (
    'name',
    'description',
    'system_prompt',
    'model_name',
    'model_max_tokens',
    'model_temperature',
    'model_repeat_penalty',
    'model_repeat_penalty_last_n_tokens',
    'model_top_k',
    'model_top_p'
) VALUES (
    'basic-code',
    'A basic code assistant with default values for all parameters.',
    'You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe. Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don''t know the answer to a question, please don''t share false information.',
    'codellama-instruct',
    100,
    0.8,
    1.1,
    64,
    40,
    0.9
);

INSERT INTO apps (
    'name',
    'description',
    'system_prompt',
    'model_name',
    'model_max_tokens',
    'model_temperature',
    'model_repeat_penalty',
    'model_repeat_penalty_last_n_tokens',
    'model_top_k',
    'model_top_p'
) VALUES (
    'python-code-generator',
    'A code generator example re-produced from github.com/ai-examples/code-generator-rs.',
    'You are an expert python programmer. Your answer should start with a [CODE] tag and end with a [/CODE] tag. Write a full python script that does in accordance with the user''s prompt.',
    'codellama-instruct',
    400,
    0.8,
    1.1,
    64,
    40,
    0.9
);

INSERT INTO apps (
    'name',
    'description',
    'system_prompt',
    'model_name',
    'model_max_tokens',
    'model_temperature',
    'model_repeat_penalty',
    'model_repeat_penalty_last_n_tokens',
    'model_top_k',
    'model_top_p'
) VALUES (
    'sentiment',
    'A sentiment analysis example re-produced from github.com/ai-examples/sentiment-analysis-rs.',
    'You are a bot that generates sentiment analysis responses. Respond with a single positive, negative, or neutral.

Follow the pattern of the following examples:

User: Hi, my name is Bob
Bot: neutral

User: I am so happy today
Bot: positive

User: I am so sad today
Bot: negative',
    'llama2-chat',
    6,
    0.8,
    1.1,
    64,
    40,
    0.9
);
