# Guidelines

This project is a Rust/React fullstack web application.

## Repository Architecture

```
- crates/ # The Rust backend of the application
- frontend/ # The React code of the frontend
- infrastructure/ # Containers and production services
- scripts/ # Helpers and scripts
- tools/ # Standalone crates or tools that are more than just a script
```

Each should contain a README.md file further describing how to work with it.

## Best Practices

Prefer writing small and dedicated files than a giant all-included one. This helps to have a small context for LLMs and makes each file more readable.

## Documentation

Whenever you work in a directory, read the README.md in this directory and the one in all its parent directories if they exists.
They contain information about how the code should be handled as well as helpful guidelines.

Do not invent APIs, when necessary, use the context7 MCP to access documentation online.

## Versionning

When working on a task, frequently run git commit with:

```bash
git commit --author=LLM -m "<message>"
```
