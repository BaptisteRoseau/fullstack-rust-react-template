#!/bin/bash
urls=(
    "http://127.0.0.1:8080"
    "http://127.0.0.1:3001"
    "http://127.0.0.1:7070/v1/docs/swagger-ui/"
    "http://127.0.0.1:9090"
    "http://127.0.0.1:3000"
)

for url in "${urls[@]}"; do
    echo "Opening $url"
    firefox "$url" 2>/dev/null &
done
