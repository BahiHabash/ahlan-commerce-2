use std::path::Path;

use serde::Deserialize;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    db::{DynStore, ImportJob, ImportJobStatus},
    domain::{CreateProduct, SystemClock, new_product},
    error::{AppError, AppResult},
};

#[derive(Debug, Deserialize)]
pub struct ImportFile {
    pub products: Vec<CreateProduct>,
}

pub fn new_job(input_path: String) -> AppResult<ImportJob> {
    if input_path.trim().is_empty() {
        return Err(AppError::validation("input_path is required"));
    }
    Ok(ImportJob {
        id: Uuid::now_v7(),
        input_path,
        status: ImportJobStatus::Queued,
        error_message: None,
    })
}

pub async fn enqueue(store: DynStore, input_path: String) -> AppResult<ImportJob> {
    let job = new_job(input_path)?;
    store.enqueue_import_job(job).await
}

pub async fn run_once(store: DynStore) -> AppResult<Option<ImportJob>> {
    let Some(mut job) = store.next_import_job().await? else {
        return Ok(None);
    };

    job.status = ImportJobStatus::Running;
    store.update_import_job(job.clone()).await?;

    match import_products(store.clone(), Path::new(&job.input_path)).await {
        Ok(()) => {
            job.status = ImportJobStatus::Succeeded;
            job.error_message = None;
            info!(job_id = %job.id, "import job succeeded");
        }
        Err(err) => {
            warn!(job_id = %job.id, error = %err, "import job failed");
            job.status = ImportJobStatus::Failed;
            job.error_message = Some(err.to_string());
        }
    }

    store.update_import_job(job.clone()).await?;
    Ok(Some(job))
}

async fn import_products(store: DynStore, path: &Path) -> AppResult<()> {
    let bytes = tokio::fs::read(path)
        .await
        .map_err(|err| AppError::dependency("failed to read import file", err))?;
    let input: ImportFile = serde_json::from_slice(&bytes)
        .map_err(|err| AppError::dependency("failed to parse import file", err))?;
    let clock = SystemClock;
    for command in input.products {
        let product = new_product(command, &clock)?;
        store.create_product(product).await?;
    }
    Ok(())
}
