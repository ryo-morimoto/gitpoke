pub mod app;
pub mod domain;
pub mod error;
pub mod handlers;
pub mod infrastructure;
pub mod middlewares;
pub mod repositories;
pub mod routes;
pub mod use_cases;

// Re-export commonly used types
pub use error::{AppError, AppResult};