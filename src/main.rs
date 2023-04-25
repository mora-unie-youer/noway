use tracing::Level;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_env("NOWAY_LOG") {
        tracing::info!(
            "Logging is being initialized with env filter: {}",
            env_filter
        );

        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_env_filter(env_filter)
            .init();
        tracing::info!("Initialized logging with env filter successfully");
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        tracing::info!("Initialized logging with default filter successfully");
    }

    tracing::info!("Starting NoWay");
    Ok(())
}
