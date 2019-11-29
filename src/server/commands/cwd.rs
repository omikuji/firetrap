//! The RFC 959 Change Working Directory (`CWD`) command
//
// This command allows the user to work with a different
// directory or dataset for file storage or retrieval without
// altering his login or accounting information.  Transfer
// parameters are similarly unchanged.  The argument is a
// pathname specifying a directory or other system dependent
// file group designator.

use crate::server::chancomms::InternalMsg;
use crate::server::commands::Cmd;
use crate::server::error::FTPError;
use crate::server::reply::Reply;
use crate::server::CommandArgs;
use crate::storage;
use crate::storage::{Error, ErrorKind};
use futures::future::Future;
use futures::sink::Sink;
use log::warn;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Cwd {
    path: PathBuf,
}

impl Cwd {
    pub fn new(path: PathBuf) -> Self {
        Cwd { path }
    }
}

impl<S, U> Cmd<S, U> for Cwd
where
    U: Send + Sync + 'static,
    S: 'static + storage::StorageBackend<U> + Sync + Send,
    S::File: tokio_io::AsyncRead + Send,
    S::Metadata: storage::Metadata,
{
    fn execute(&self, args: &CommandArgs<S, U>) -> Result<Reply, FTPError> {
        let session_arc = args.session.clone();
        let session = args.session.lock()?;
        let storage = Arc::clone(&session.storage);
        let path = session.cwd.join(self.path.clone());
        let tx_success = args.tx.clone();
        let tx_fail = args.tx.clone();

        let cwd_path = self.path.clone();

        tokio::spawn(
            storage
                .cwd(&session.user, path)
                .and_then(|_| tx_success.send(InternalMsg::DelSuccess).map_err(|_| Error::from(ErrorKind::LocalError)))
                .or_else(|e| tx_fail.send(InternalMsg::StorageError(e)))
                .map(move |_| {
                    session_arc.lock().unwrap().cwd.push(cwd_path.clone());
                    ()
                })
                .map_err(|e| {
                    warn!("Failed to cwd directory: {}", e);
                }),
        );

        Ok(Reply::none())
    }
}
