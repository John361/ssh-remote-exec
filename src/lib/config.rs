use std::path::PathBuf;

pub struct SshConfig {
    pub host: String,
    pub username: String,
    pub public_key: PathBuf,
    pub private_key: PathBuf,
}
