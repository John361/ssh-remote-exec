use std::path::PathBuf;

use ssh2::Session;
use thiserror::Error;

pub struct SshSessionIdentifier {
    pub host: String,
    pub session: Session,
}

impl SshSessionIdentifier {
    pub fn new(host: String, session: Session) -> Self {
        Self { host, session }
    }
}

pub struct SshConfig {
    pub hosts: Vec<String>,
    pub username: String,
    pub public_key: PathBuf,
    pub private_key: PathBuf,
}

impl SshConfig {
    pub fn new(hosts: Vec<String>, username: String, public_key: PathBuf, private_key: PathBuf) -> Self {
        Self {
            hosts,
            username,
            public_key,
            private_key,
        }
    }
}

#[derive(Debug, Error)]
pub enum SshRemoteExecError {
    #[error("Error during remote ssh connection: {0}")]
    RemoteConnection(String),

    #[error("Error during remote ssh disconnection: {0}")]
    RemoteDisconnection(String),

    #[error("Error during remote ssh command execution: {0}")]
    RemoteCommandExecution(String),
}
