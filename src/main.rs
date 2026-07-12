use ahlan_commerce::{api, config::Config, db, migrations, observability};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    observability::init();
    let config = Config::from_env();

    if config.run_refinery_migrations {
        migrations::run_refinery(&config.database_url).await?;
    }

    let store = db::store_from_config(&config).await?;
    let app = api::build_router(config.clone(), store);
    let listener = TcpListener::bind(config.bind_addr()).await?;

    info!(address = %config.bind_addr(), "starting ahlan-commerce api");
    axum::serve(listener, app).await?;
    Ok(())
}
