use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    config::Config,
    domain::{Product, UpdatePublication},
    error::{AppError, AppResult},
};

#[async_trait]
pub trait ProductStore: Send + Sync {
    async fn create_product(&self, product: Product) -> AppResult<Product>;
    async fn list_products(&self) -> AppResult<Vec<Product>>;
    async fn list_published_products(&self) -> AppResult<Vec<Product>>;
    async fn get_product_by_handle(&self, handle: &str) -> AppResult<Option<Product>>;
    async fn update_product_publication(
        &self,
        id: Uuid,
        publication: UpdatePublication,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Product>;
    async fn enqueue_import_job(&self, job: ImportJob) -> AppResult<ImportJob>;
    async fn next_import_job(&self) -> AppResult<Option<ImportJob>>;
    async fn update_import_job(&self, job: ImportJob) -> AppResult<ImportJob>;
}

pub type DynStore = Arc<dyn ProductStore>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, utoipa::ToSchema)]
pub struct ImportJob {
    pub id: Uuid,
    pub input_path: String,
    pub status: ImportJobStatus,
    pub error_message: Option<String>,
}

#[derive(
    Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ImportJobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

impl ImportJobStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

impl TryFrom<String> for ImportJobStatus {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            _ => Err(AppError::Internal(format!(
                "unknown import job status: {value}"
            ))),
        }
    }
}

#[derive(FromRow)]
struct ImportJobRow {
    id: Uuid,
    input_path: String,
    status: String,
    error_message: Option<String>,
}

impl TryFrom<ImportJobRow> for ImportJob {
    type Error = AppError;

    fn try_from(row: ImportJobRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            input_path: row.input_path,
            status: row.status.try_into()?,
            error_message: row.error_message,
        })
    }
}

#[derive(Default)]
pub struct MemoryStore {
    products: RwLock<HashMap<Uuid, Product>>,
    jobs: RwLock<HashMap<Uuid, ImportJob>>,
}

impl MemoryStore {
    pub fn shared() -> DynStore {
        Arc::new(Self::default())
    }
}

#[async_trait]
impl ProductStore for MemoryStore {
    async fn create_product(&self, product: Product) -> AppResult<Product> {
        let mut products = self.products.write().await;
        if products
            .values()
            .any(|existing| existing.handle == product.handle)
        {
            return Err(AppError::validation("handle already exists"));
        }
        products.insert(product.id, product.clone());
        Ok(product)
    }

    async fn list_products(&self) -> AppResult<Vec<Product>> {
        let mut products: Vec<_> = self.products.read().await.values().cloned().collect();
        products.sort_by_key(|p| (p.created_at, p.id));
        Ok(products)
    }

    async fn list_published_products(&self) -> AppResult<Vec<Product>> {
        let mut products: Vec<_> = self
            .products
            .read()
            .await
            .values()
            .filter(|product| product.published)
            .cloned()
            .collect();
        products.sort_by_key(|p| (std::cmp::Reverse(p.published_at), p.created_at, p.id));
        Ok(products)
    }

    async fn get_product_by_handle(&self, handle: &str) -> AppResult<Option<Product>> {
        Ok(self
            .products
            .read()
            .await
            .values()
            .find(|product| product.handle == handle)
            .cloned())
    }

    async fn update_product_publication(
        &self,
        id: Uuid,
        publication: UpdatePublication,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Product> {
        let mut products = self.products.write().await;
        let product = products
            .get_mut(&id)
            .ok_or_else(|| AppError::NotFound("product not found".to_string()))?;
        product.published = publication.published;
        product.published_at = published_at;
        product.updated_at = updated_at;
        Ok(product.clone())
    }

    async fn enqueue_import_job(&self, job: ImportJob) -> AppResult<ImportJob> {
        self.jobs.write().await.insert(job.id, job.clone());
        Ok(job)
    }

    async fn next_import_job(&self) -> AppResult<Option<ImportJob>> {
        Ok(self
            .jobs
            .read()
            .await
            .values()
            .find(|job| job.status == ImportJobStatus::Queued)
            .cloned())
    }

    async fn update_import_job(&self, job: ImportJob) -> AppResult<ImportJob> {
        self.jobs.write().await.insert(job.id, job.clone());
        Ok(job)
    }
}

pub struct PostgresStore {
    pool: PgPool,
}

impl PostgresStore {
    pub async fn connect(database_url: &str) -> AppResult<DynStore> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|err| AppError::dependency("database connection failed", err))?;
        Ok(Arc::new(Self { pool }))
    }
}

pub async fn store_from_config(config: &Config) -> AppResult<DynStore> {
    match &config.database_url {
        Some(url) => PostgresStore::connect(url).await,
        None => Ok(MemoryStore::shared()),
    }
}

#[async_trait]
impl ProductStore for PostgresStore {
    async fn create_product(&self, product: Product) -> AppResult<Product> {
        sqlx::query_as::<_, Product>(
            r#"
            insert into products (
                id, title, handle, description, price_cents, inventory_quantity,
                published, published_at, created_at, updated_at
            )
            values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            returning id, title, handle, description, price_cents, inventory_quantity,
                published, published_at, created_at, updated_at
            "#,
        )
        .bind(product.id)
        .bind(product.title)
        .bind(product.handle)
        .bind(product.description)
        .bind(product.price_cents)
        .bind(product.inventory_quantity)
        .bind(product.published)
        .bind(product.published_at)
        .bind(product.created_at)
        .bind(product.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to create product", err))
    }

    async fn list_products(&self) -> AppResult<Vec<Product>> {
        sqlx::query_as::<_, Product>(
            r#"
            select id, title, handle, description, price_cents, inventory_quantity,
                published, published_at, created_at, updated_at
            from products
            order by created_at asc, id asc
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to list products", err))
    }

    async fn list_published_products(&self) -> AppResult<Vec<Product>> {
        sqlx::query_as::<_, Product>(
            r#"
            select id, title, handle, description, price_cents, inventory_quantity,
                published, published_at, created_at, updated_at
            from products
            where published = true
            order by published_at desc nulls last, created_at asc, id asc
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to list published products", err))
    }

    async fn get_product_by_handle(&self, handle: &str) -> AppResult<Option<Product>> {
        sqlx::query_as::<_, Product>(
            r#"
            select id, title, handle, description, price_cents, inventory_quantity,
                published, published_at, created_at, updated_at
            from products
            where handle = $1
            "#,
        )
        .bind(handle)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to get product", err))
    }

    async fn update_product_publication(
        &self,
        id: Uuid,
        publication: UpdatePublication,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Product> {
        sqlx::query_as::<_, Product>(
            r#"
            update products
            set published = $2, published_at = $3, updated_at = $4
            where id = $1
            returning id, title, handle, description, price_cents, inventory_quantity,
                published, published_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(publication.published)
        .bind(published_at)
        .bind(updated_at)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to update publication", err))?
        .ok_or_else(|| AppError::NotFound("product not found".to_string()))
    }

    async fn enqueue_import_job(&self, job: ImportJob) -> AppResult<ImportJob> {
        let row = sqlx::query_as::<_, ImportJobRow>(
            r#"
            insert into import_jobs (id, input_path, status, error_message)
            values ($1, $2, $3, $4)
            returning id, input_path, status, error_message
            "#,
        )
        .bind(job.id)
        .bind(job.input_path)
        .bind(job.status.as_str())
        .bind(job.error_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to enqueue import job", err))?;
        row.try_into()
    }

    async fn next_import_job(&self) -> AppResult<Option<ImportJob>> {
        let row = sqlx::query_as::<_, ImportJobRow>(
            r#"
            select id, input_path, status, error_message
            from import_jobs
            where status = 'queued'
            order by created_at asc, id asc
            limit 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to fetch import job", err))?;
        row.map(TryInto::try_into).transpose()
    }

    async fn update_import_job(&self, job: ImportJob) -> AppResult<ImportJob> {
        let row = sqlx::query_as::<_, ImportJobRow>(
            r#"
            update import_jobs
            set status = $2, error_message = $3, updated_at = now()
            where id = $1
            returning id, input_path, status, error_message
            "#,
        )
        .bind(job.id)
        .bind(job.status.as_str())
        .bind(job.error_message)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| AppError::dependency("failed to update import job", err))?;
        row.try_into()
    }
}
