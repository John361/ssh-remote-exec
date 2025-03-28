use std::path::PathBuf;

use colored::Colorize;
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

pub struct SshCommandResult {
    pub host: String,
    pub result: String,
    pub status: SshCommandResultStatus,
}

impl SshCommandResult {
    pub fn new(host: String, result: String, status: SshCommandResultStatus) -> Self {
        Self { host, result, status }
    }

    pub fn print(&self) {
        println!("[{}]", self.host.blue());

        match self.status {
            SshCommandResultStatus::Success => println!("{}", self.result.green()),
            SshCommandResultStatus::Error => println!("{}", self.result.red()),
        }
    }
}

pub enum SshCommandResultStatus {
    Success,
    Error,
}

pub struct SshConfig {
    pub hosts: Vec<String>,
    pub username: String,
    pub password: String,
    pub public_key: PathBuf,
    pub private_key: PathBuf,
}

impl SshConfig {
    pub fn new(hosts: Vec<String>, username: String, password: String, identity: PathBuf) -> Self {
        let public_key = format!("{}.pub", identity.display());
        let public_key = PathBuf::from(&public_key);

        Self {
            hosts,
            username,
            password,
            public_key,
            private_key: identity,
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
