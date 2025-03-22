use std::path::PathBuf;

use lib::config::SshConfig;
use lib::ssh::SshManager;

fn main() {
    init_tracing();

    let config = SshConfig {
        host: "192.168.132.133:22".to_string(),
        username: "root".to_string(),
        public_key: PathBuf::from("tmp/id_ed25519.pub"),
        private_key: PathBuf::from("tmp/id_ed25519"),
    };

    let mut manager = SshManager::new(config);
    manager.connect().unwrap();

    let result = manager.execute_command("apt update").unwrap();
    println!("{}", result);

    manager.disconnect().unwrap();
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .init();

    tracing::debug!("Tracing initialized");
}
