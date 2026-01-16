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

When working on a new task, first search for pieces of code that are similar to what you need to implement. Read their related README.md and copy the style of the codebase.

### Planning

When working on a task, use the `llm_memory/` directory. This is your place, you will have an SQLite database with an MCP and maybe even a RAG to store and retrieve relevant pieces of information.

When working on a task:

- Create a branch for your task if not already on a new one
- First have a read-only approach: list what pieces of information you need to retrieve as bulletpoints
- Then, go in the codebase, database and documentation find those information and summarize them
- Then, plan the modifications that needs to be done and split them into small chunks that will be done one by one
- Go back and forth with the plan, implement a chunk, read it, criticize it and refactor it until you are satisfied
- Test it using the unit tests, compiler and linter
- Only if necessary, add unit tests
- Commit your code
- Go onto the next chunk and repeat
- When finished, tell the user your task is done and ask for a review. Do not merge the branch yourself.

Always store relevant information, delete obsolete information and find a clever way to index them to find them quickly whenever possible.

Do not invent, if you need information or documentation, use the codebase, your `llm_memory/` directory and MCP server, and use the context7 MCP when you need documentation.

### Testing

Always run the unit tests and linters. If running in an IDE environment, use its agent protocol to find linter issues. Use the `test_lint.sh` and `test_units.sh` files to run the tests. Focus on fixing the issues before going any further.

Be critical on the issues: is the problem from the test or the codebase ? If in doubt, consider it is from the codebase and do not update the test, otherwise carefully update the test.

## Documentation

Whenever you work in a directory, read the README.md in this directory and the one in all its parent directories if they exists.
They contain information about how the code should be handled as well as helpful guidelines.

Do not invent APIs, when necessary, use the context7 MCP to access documentation online.

## Versionning

When working on a task, frequently run git commit with:

```bash
git commit --author=LLM -m "<message>"
```
