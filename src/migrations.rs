use crate::error::{AppError, AppResult};

mod embedded {
    refinery::embed_migrations!("db/refinery_migrations");
}

pub async fn run_refinery(database_url: &Option<String>) -> AppResult<()> {
    let database_url = database_url
        .as_deref()
        .ok_or_else(|| AppError::validation("DATABASE_URL is required for refinery migrations"))?;
    let (mut client, connection) = tokio_postgres::connect(database_url, tokio_postgres::NoTls)
        .await
        .map_err(|err| AppError::dependency("failed to connect for refinery migrations", err))?;
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            tracing::error!(%error, "refinery migration connection failed");
        }
    });
    embedded::migrations::runner()
        .run_async(&mut client)
        .await
        .map_err(|err| AppError::dependency("refinery migration failed", err))?;
    Ok(())
}
