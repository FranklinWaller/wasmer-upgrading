use std::{num::ParseIntError, string::FromUtf8Error};

use thiserror::Error;
use wasmer::{CompileError, ExportError, InstantiationError};
use wasmer_wasix::{FsError, WasiError, WasiStateCreationError};

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(transparent)]
    WasmCompileError(#[from] CompileError),

    #[error(transparent)]
    WasmInstantiationError(Box<InstantiationError>),

    #[error(transparent)]
    WasiError(#[from] WasiError),
    #[error(transparent)]
    WasiStateCreationError(#[from] WasiStateCreationError),

    #[error(transparent)]
    FunctionNotFound(#[from] ExportError),

    #[error("Error while running: {0}")]
    ExecutionError(#[from] wasmer::RuntimeError),

    #[error("VM Host Error: {0}")]
    VmHostError(String),

    #[error("{0}")]
    WasiFsError(#[from] FsError),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[cfg(test)]
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    MemoryAccessError(#[from] wasmer::MemoryAccessError),

    #[error(transparent)]
    WasmSerializeError(#[from] wasmer::SerializeError),

    #[error(transparent)]
    WasmDeserializeError(#[from] wasmer::DeserializeError),

    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
}

impl From<InstantiationError> for RuntimeError {
    fn from(r: InstantiationError) -> Self {
        Self::WasmInstantiationError(Box::new(r))
    }
}

impl From<serde_json::Error> for RuntimeError {
    fn from(s: serde_json::Error) -> Self {
        Self::VmHostError(s.to_string())
    }
}

impl From<String> for RuntimeError {
    fn from(s: String) -> Self {
        Self::VmHostError(s)
    }
}

impl From<&str> for RuntimeError {
    fn from(s: &str) -> Self {
        Self::VmHostError(s.into())
    }
}

pub type Result<T, E = RuntimeError> = core::result::Result<T, E>;
