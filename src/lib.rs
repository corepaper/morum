mod error;

pub mod config;
pub mod matrix;
pub mod web;

pub use crate::config::Config;
pub use crate::error::Error;
pub use crate::matrix::MatrixService;
pub use crate::web::UserError;
