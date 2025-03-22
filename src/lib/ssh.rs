use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;

use crate::config::SshConfig;
use crate::error::SshRemoteExecError;

pub struct SshManager {
    config: SshConfig,
    session: Option<Session>,
}

impl SshManager {

    pub fn new(config: SshConfig) -> Self {
        Self {
            config,
            session: None
        }
    }

    pub fn connect(&mut self) -> Result<(), SshRemoteExecError> {
        let tcp = TcpStream::connect(&self.config.host)
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

        self.session = Some(session);
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), SshRemoteExecError> {
        if let Some(session) = self.session.as_ref() {
            session.disconnect(None, "", None)
                .map_err(|e| SshRemoteExecError::RemoteConnection(e.to_string()))
                .inspect(|_| tracing::debug!("Session disconnected"))
                .inspect_err(|e| tracing::error!("{e:}"))?;

            Ok(())
        } else {
            let e = SshRemoteExecError::RemoteDisconnection("No existing session found".to_string());
            tracing::error!("{e:}");

            Err(e)
        }
    }

    pub fn execute_command(&self, command: &str) -> Result<String, SshRemoteExecError> {
        if let Some(session) = self.session.as_ref() {
            tracing::info!("Executing command on {}", self.config.host);

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

            Ok(result)
        } else {
            let e = SshRemoteExecError::RemoteCommandExecution("No existing session found".to_string());
            tracing::error!("{e:}");

            Err(e)
        }
    }
}
