use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;

use crate::model::{SshCommandResult, SshCommandResultStatus, SshRemoteExecError};
use crate::model::{SshConfig, SshSessionIdentifier};

pub struct SshExecutor {
    config: SshConfig,
    sessions: Vec<SshSessionIdentifier>,
}

impl SshExecutor {

    pub fn new(config: SshConfig) -> Self {
        Self {
            config,
            sessions: Vec::new()
        }
    }

    pub fn connect(&mut self) -> Result<(), SshRemoteExecError> {
        for host in &self.config.hosts {
            let tcp = TcpStream::connect(host)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{host:}] TCP connection established"))
                .inspect_err(|e| tracing::error!("[{host:}] {e:}"))?;

            let mut session = Session::new()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{host:}] Session established"))
                .inspect_err(|e| tracing::error!("[{host:}] {e:}"))?;

            session.set_tcp_stream(tcp);
            session.handshake()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{host:}] Session handshake realized"))
                .inspect_err(|e| tracing::error!("[{host:}] {e:}"))?;

            session.userauth_pubkey_file(&self.config.username,
                                         Some(self.config.public_key.as_path()),
                                         self.config.private_key.as_path(),
                                         None)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{host:}] Session authenticated"))
                .inspect_err(|e| tracing::error!("[{host:}] {e:}"))?;

            self.sessions.push(SshSessionIdentifier::new(host.clone(), session));
        }

        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), SshRemoteExecError> {
        if self.sessions.is_empty() {
            let e = SshRemoteExecError::RemoteDisconnection("No existing session found".to_string());
            tracing::error!("{e:}");

            return Err(e);
        }

        for session in &self.sessions {
            session.session.disconnect(None, "", None)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{}] Session disconnected", session.host))
                .inspect_err(|e| tracing::error!("[{}] {e:}", session.host))?;
        }

        self.sessions.clear();
        Ok(())
    }

    pub fn execute_command(&self, mut command: String) -> Result<Vec<SshCommandResult>, SshRemoteExecError> {
        if self.sessions.is_empty() {
            let e = SshRemoteExecError::RemoteCommandExecution("No existing session found".to_string());
            tracing::error!("{e:}");

            return Err(e);
        }

        if command.contains("sudo") {
            command = command.replace("sudo", "");
            command = format!("echo {} | sudo -S {}", self.config.password, command);
        }

        let mut results = Vec::new();

        for session in &self.sessions {
            let mut channel = session.session.channel_session()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{}] Channel session established", session.host))
                .inspect_err(|e| tracing::error!("[{}] {e:}", session.host))?;

            channel.exec(&command)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{}] Remote command executed", session.host))
                .inspect_err(|e| tracing::error!("[{}] {e:}", session.host))?;

            let mut stdout = String::new();
            let mut stderr = String::new();

            channel.read_to_string(&mut stdout)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{}] Remote command result read", session.host))
                .inspect_err(|e| tracing::error!("[{}] {e:}", session.host))?;

            channel.stderr().read_to_string(&mut stderr)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{}] Remote command result read", session.host))
                .inspect_err(|e| tracing::error!("[{}] {e:}", session.host))
                .ok();

            channel.wait_close()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{}] Channel session closed", session.host))
                .inspect_err(|e| tracing::error!("[{}] {e:}", session.host))?;

            let status = channel.exit_status()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("[{}] Channel exit status read", session.host))
                .inspect_err(|e| tracing::error!("[{}] {e:}", session.host))?;

            if status != 0 {
                let result = SshCommandResult::new(session.host.clone(), stderr, SshCommandResultStatus::Error);
                results.push(result);
            } else {
                let result = SshCommandResult::new(session.host.clone(), stdout, SshCommandResultStatus::Success);
                results.push(result);
            }
        }

        Ok(results)
    }
}
