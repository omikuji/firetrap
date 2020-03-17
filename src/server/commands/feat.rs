//! The RFC 2389 Feature (`FEAT`) command

use crate::server::commands::Cmd;
use crate::server::error::FTPError;
use crate::server::reply::{Reply, ReplyCode};
use crate::server::CommandArgs;
use crate::storage;

pub struct Feat;

impl<S, U> Cmd<S, U> for Feat
where
    U: Send + Sync + 'static,
    S: 'static + storage::StorageBackend<U> + Sync + Send,
    S::File: tokio_io::AsyncRead + Send,
    S::Metadata: storage::Metadata,
{
    fn execute(&self, args: &CommandArgs<S, U>) -> Result<Reply, FTPError> {
        let mut feat_text = vec![
            " SIZE",
            " MDTM",
            " MLST modify*;perm*;size*;type*;unique*;UNIX.group*;UNIX.mode*;UNIX.owner*;",
            "UTF8",
        ];

        // Add the features. According to the spec each feature line must be
        // indented by a space.
        if args.tls_configured {
            feat_text.push(" AUTH TLS");
            feat_text.push(" PBSZ");
            feat_text.push(" PROT");
        }
        if args.storage_features & storage::FEATURE_RESTART > 0 {
            feat_text.push(" REST STREAM");
        }

        // Show them in alphabetical order.
        feat_text.sort();
        feat_text.insert(0, "Extensions supported:");
        feat_text.push("END");

        let reply = Reply::new_multiline(ReplyCode::SystemStatus, feat_text);
        Ok(reply)
    }
}
