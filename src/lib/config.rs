use std::path::PathBuf;

pub struct SshConfig {
    pub hosts: Vec<String>,
    pub username: String,
    pub public_key: PathBuf,
    pub private_key: PathBuf,
}
