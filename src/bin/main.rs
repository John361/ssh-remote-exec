use lib::cli::Cli;
use lib::executor::SshExecutor;
use lib::model::SshConfig;

fn main() {
    init_tracing();

    let args = Cli::load();
    let config = SshConfig::new(args.hosts, args.username, args.public_key, args.private_key);

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
