use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;

use crate::model::SshRemoteExecError;
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
                .inspect(|_| tracing::debug!("TCP connection established"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            let mut session = Session::new()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Session established"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            session.set_tcp_stream(tcp);
            session.handshake()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Session handshake realized"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            session.userauth_pubkey_file(&self.config.username,
                                         Some(self.config.public_key.as_path()),
                                         self.config.private_key.as_path(),
                                         None)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Session authenticated"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

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
                .inspect(|_| tracing::debug!("Session disconnected"))
                .inspect_err(|e| tracing::error!("{e:}"))?;
        }

        self.sessions.clear();
        Ok(())
    }

    pub fn execute_command(&self, command: &str) -> Result<Vec<String>, SshRemoteExecError> {
        if self.sessions.is_empty() {
            let e = SshRemoteExecError::RemoteCommandExecution("No existing session found".to_string());
            tracing::error!("{e:}");

            return Err(e);
        }

        let mut results = Vec::new();

        for session in &self.sessions {
            let mut channel = session.session.channel_session()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Channel session established"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            channel.exec(command)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Remote command executed"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            let mut result = String::new();
            channel.read_to_string(&mut result)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Remote command result read"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            channel.wait_close()
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Channel session closed"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            result = format!("[{}]\n{result:}", session.host);
            results.push(result);
        }

        Ok(results)
    }
}
