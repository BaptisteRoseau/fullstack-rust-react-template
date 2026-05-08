# Models

These are the core models for the application objects used mainly by the [app_core](../app_core) crate.
Other layer crates convert from/to these models by implementing `From<T>` and `Into<T>` trait.

For example, the [app_core](../app_core)

## Structure

Each model has its own file where they implement their structure definition, traits, methods, and tests.
