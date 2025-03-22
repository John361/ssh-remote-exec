use std::path::PathBuf;

use lib::executor::SshExecutor;
use lib::model::SshConfig;

fn main() {
    init_tracing();

    let config = SshConfig {
        hosts: vec!["192.168.132.133:22".to_string(), "192.168.132.133:22".to_string()],
        username: "root".to_string(),
        public_key: PathBuf::from("tmp/id_ed25519.pub"),
        private_key: PathBuf::from("tmp/id_ed25519"),
    };

    let mut manager = SshExecutor::new(config);
    manager.connect().unwrap();

    let results = manager.execute_command("apt update").unwrap();
    results.iter().for_each(|r| println!("{}", r));

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
