mod embedded {
    refinery::embed_migrations!("../../db/refinery_migrations");
}

pub async fn run(database_url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut client, connection) =
        tokio_postgres::connect(database_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            tracing::error!(%error, "refinery migration connection failed");
        }
    });

    embedded::migrations::runner()
        .run_async(&mut client)
        .await?;
    Ok(())
}
