# Models

This crate contains the main models used throughout the core of the application.

Each layer should implement From<T> and/or Into<T> where T is defined in this crate for interoperation.

For example, this crate may defined a `User` model. The `api` and `database` crates may have their own `User` models for API communication or database storage.
These will have to implement the above trait to pass the user to inner/outer layer.
