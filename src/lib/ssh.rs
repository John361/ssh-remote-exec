use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;

use crate::config::SshConfig;
use crate::error::SshRemoteExecError;

pub struct SshManager {
    config: SshConfig,
    sessions: Vec<Session>,
}

impl SshManager {

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

            self.sessions.push(session);
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
            session.disconnect(None, "", None)
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
            // tracing::info!("Executing command on {}", self.config.host); // TODO

            let mut channel = session.channel_session()
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

            results.push(result);
        }

        Ok(results)
    }
}
