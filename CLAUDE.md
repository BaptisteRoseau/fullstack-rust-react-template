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

When working on a task:

- Create a branch for your task if not already on a new one
- Make a clear TODO list of the tasks
- First have a read-only approach: list what pieces of information you need to retrieve as bulletpoints
- Then, use an agent to go in the codebase, database and documentation find those information and summarize them
- Then, plan the modifications that needs to be done and split them into small chunks that will be done one by one
- Go back and forth with the plan, start an agent that will:
    - implement a chunk, read it, criticize it and refactor it until it is satisfied
    - Test it using the unit tests, compiler and linter (use the scripts)
    - Only if necessary, add unit tests
- Commit your code with: git commit --author=Claude -m "<message>"
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

If you need Rust crate documentation, instead of using `crates.io` prefer using:

```
curl file://<absolute path to current project>/target/doc/<the crate you're looking for>/index.html
```

If hitting a 404, run `cargo doc` to build the documentaion pages.
If still hitting a 404 fallback to `crates.io`.
Pipe bash commands to convert the HTML to text to reduce token usage and only get useful text.

Do not invent APIs, when necessary, use the context7 MCP to access documentation online.

## Running the services

All the services required to run the application can be launched using `docker compose up -d`.
