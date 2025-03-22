use thiserror::Error;

#[derive(Debug, Error)]
pub enum SshRemoteExecError {
    #[error("Error during remote ssh connection: {0}")]
    RemoteConnection(String),

    #[error("Error during remote ssh disconnection: {0}")]
    RemoteDisconnection(String),

    #[error("Error during remote ssh command execution: {0}")]
    RemoteCommandExecution(String),
}
