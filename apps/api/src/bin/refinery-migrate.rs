use api::{config::Config, migrations};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=info,refinery=info".into()),
        )
        .init();

    let config = Config::from_env();
    migrations::run(&config.database_url).await?;
    tracing::info!("Refinery migrations applied");
    Ok(())
}
