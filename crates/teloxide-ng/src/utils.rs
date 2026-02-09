//! Some useful utilities.

pub mod command;
pub mod html;
pub mod markdown;
pub mod render;
pub(crate) mod shutdown_token;

pub use teloxide_core_ng::net::client_from_env;
