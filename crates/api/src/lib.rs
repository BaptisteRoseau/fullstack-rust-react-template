mod app_state;
#[warn(unused)]
mod endpoints;
mod extractors;
mod misc;
mod models;

pub mod error;
pub mod routes;
pub use app_state::AppState;
