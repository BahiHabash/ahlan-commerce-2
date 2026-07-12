use ahlan_commerce::{config::Config, db, migrations, observability, worker};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    observability::init();
    let config = Config::from_env();

    if config.run_refinery_migrations {
        migrations::run_refinery(&config.database_url).await?;
    }

    let store = db::store_from_config(&config).await?;
    let result = worker::run_once(store).await?;
    info!(job_processed = result.is_some(), "worker tick complete");
    Ok(())
}
