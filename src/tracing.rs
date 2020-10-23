use std::io;
use tracing_subscriber::EnvFilter;

pub fn setup_global_subscriber() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info,wgpu=warn"))
        .unwrap();
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing global default subscriber");
}
