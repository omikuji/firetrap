//! The RFC 959 Rename From (`RNFR`) command

use crate::server::commands::Cmd;
use crate::server::error::FTPError;
use crate::server::reply::{Reply, ReplyCode};
use crate::server::CommandArgs;
use crate::storage;
use std::path::PathBuf;

pub struct Rnfr {
    path: PathBuf,
}

impl Rnfr {
    pub fn new(path: PathBuf) -> Self {
        Rnfr { path }
    }
}

impl<S, U> Cmd<S, U> for Rnfr
where
    U: Send + Sync + 'static,
    S: 'static + storage::StorageBackend<U> + Sync + Send,
    S::File: tokio_io::AsyncRead + Send,
    S::Metadata: storage::Metadata,
{
    fn execute(&self, args: &CommandArgs<S, U>) -> Result<Reply, FTPError> {
        let mut session = args.session.lock()?;
        session.rename_from = Some(session.cwd.join(self.path.clone()));
        Ok(Reply::new(ReplyCode::FileActionPending, "Tell me, what would you like the new name to be?"))
    }
}
