use std::time::SystemTimeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("out of the storage: capacity is {cap} required is at least {required}")]
    Overflow { cap: usize, required: usize },
    #[error("invalid argument:{0}")]
    InvalidArgument(String),
    #[error("No ID specified")]
    NoIDSpecified,
    #[error("No Group Code specified")]
    NoGroupCodeSpecified,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[error(transparent)]
    Core(#[from] rust_p2p_core::error::Error),
    #[error(transparent)]
    SystemTimeError(#[from] SystemTimeError),
    #[error("Node ID not available")]
    NodeIDNotAvailable,
    #[error("Pipe is shutdown")]
    Shutdown,
    #[error("Pipe has already shutdown")]
    AlreadyShutdown,
    #[error("Timeout")]
    Timeout,
    #[error(transparent)]
    RmpDecodeError(#[from] rmp_serde::decode::Error),
    #[error(transparent)]
    RmpEncodeError(#[from] rmp_serde::encode::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
