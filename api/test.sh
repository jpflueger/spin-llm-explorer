#!/usr/bin/env bash

curl -v -X POST http://127.0.0.1:3000/api \
-H "Content-Type: application/json" \
-d @- << EOF
{
    "model": "llama2-chat",
    "prompt": "Hello, what is your name?"
}
EOF