use thiserror::Error;

#[derive(Error, Debug)]
pub enum KzgError {}

pub type Result<T> = std::result::Result<T, KzgError>;
