//! The RFC 959 Transfer Mode (`MODE`) command
//
// The argument is a single Telnet character code specifying
// the data transfer modes described in the Section on
// Transmission Modes.
//
// The following codes are assigned for transfer modes:
//
// S - Stream
// B - Block
// C - Compressed
//
// The default transfer mode is Stream.

use crate::server::commands::Cmd;
use crate::server::error::FTPError;
use crate::server::reply::{Reply, ReplyCode};
use crate::server::CommandArgs;
use crate::storage;

/// The parameter that can be given to the `MODE` command. The `MODE` command is obsolete, and we
/// only support the `Stream` mode. We still have to support the command itself for compatibility
/// reasons, though.
#[derive(Debug, PartialEq, Clone)]
pub enum ModeParam {
    /// Data is sent in a continuous stream of bytes.
    Stream,
    /// Data is sent as a series of blocks preceded by one or more header bytes.
    Block,
    /// Some round-about way of sending compressed data.
    Compressed,
}

pub struct Mode {
    params: ModeParam,
}

impl Mode {
    pub fn new(params: ModeParam) -> Self {
        Mode { params }
    }
}

impl<S, U> Cmd<S, U> for Mode
where
    U: Send + Sync + 'static,
    S: 'static + storage::StorageBackend<U> + Sync + Send,
    S::File: tokio_io::AsyncRead + Send,
    S::Metadata: storage::Metadata,
{
    fn execute(&self, _args: &CommandArgs<S, U>) -> Result<Reply, FTPError> {
        match &self.params {
            ModeParam::Stream => Ok(Reply::new(ReplyCode::CommandOkay, "Using Stream transfer mode")),
            _ => Ok(Reply::new(
                ReplyCode::CommandNotImplementedForParameter,
                "Only Stream transfer mode is supported",
            )),
        }
    }
}
