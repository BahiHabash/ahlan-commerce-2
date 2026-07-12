use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use uuid::Uuid;

use crate::{
    cache::StorefrontCache,
    config::Config,
    db::{DynStore, ImportJob},
    domain::{CreateProduct, Product, SystemClock, UpdatePublication, new_product},
    error::{AppResult, ErrorEnvelope},
    graphql::{self, AhlanSchema},
    storefront, worker,
};

pub const ROUTE_HEALTH: &str = "/health";
pub const ROUTE_PRODUCTS: &str = "/api/products";
pub const ROUTE_PRODUCT_PUBLICATION: &str = "/api/products/{id}/publication";
pub const ROUTE_IMPORT_JOBS: &str = "/api/import-jobs";
pub const ROUTE_GRAPHQL: &str = "/graphql";
pub const ROUTE_STOREFRONT_PRODUCT: &str = "/products/{handle}";

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub store: DynStore,
    pub cache: StorefrontCache,
    pub schema: AhlanSchema,
}

impl AppState {
    fn new(config: Config, store: DynStore) -> Self {
        let cache = StorefrontCache::default();
        let placeholder = graphql::schema(Self {
            config: config.clone(),
            store: store.clone(),
            cache: cache.clone(),
            schema: async_graphql::Schema::build(
                graphql::QueryRoot,
                graphql::MutationRoot,
                async_graphql::EmptySubscription,
            )
            .finish(),
        });
        Self {
            config,
            store,
            cache,
            schema: placeholder,
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(health, create_product, list_products, update_publication, create_import_job),
    components(schemas(Product, CreateProduct, UpdatePublication, ImportJob, ImportJobRequest, ErrorEnvelope)),
    tags((name = "ahlan-commerce", description = "Ahlan Commerce API"))
)]
struct ApiDoc;

pub fn build_router(config: Config, store: DynStore) -> Router {
    let mut state = AppState::new(config, store);
    state.schema = graphql::schema(state.clone());

    Router::new()
        .route(ROUTE_HEALTH, get(health))
        .route(ROUTE_PRODUCTS, get(list_products).post(create_product))
        .route(ROUTE_PRODUCT_PUBLICATION, put(update_publication))
        .route(ROUTE_IMPORT_JOBS, post(create_import_job))
        .route(ROUTE_GRAPHQL, post(graphql_handler))
        .route(ROUTE_STOREFRONT_PRODUCT, get(storefront::product_page))
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[utoipa::path(get, path = "/health", responses((status = 200, body = HealthResponse)))]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[utoipa::path(get, path = "/api/products", responses((status = 200, body = [Product])))]
pub async fn list_products(State(state): State<AppState>) -> AppResult<Json<Vec<Product>>> {
    Ok(Json(state.store.list_products().await?))
}

#[utoipa::path(
    post,
    path = "/api/products",
    request_body = CreateProduct,
    responses((status = 201, body = Product), (status = 400, body = ErrorEnvelope))
)]
pub async fn create_product(
    State(state): State<AppState>,
    Json(input): Json<CreateProduct>,
) -> AppResult<(StatusCode, Json<Product>)> {
    let product = new_product(input, &SystemClock)?;
    let created = state.store.create_product(product).await?;
    state.cache.invalidate_product_page(&created.handle).await;
    Ok((StatusCode::CREATED, Json(created)))
}

#[utoipa::path(
    put,
    path = "/api/products/{id}/publication",
    request_body = UpdatePublication,
    responses((status = 200, body = Product), (status = 404, body = ErrorEnvelope))
)]
pub async fn update_publication(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePublication>,
) -> AppResult<Json<Product>> {
    let now = chrono::Utc::now();
    let product = state
        .store
        .update_product_publication(id, input.clone(), input.published.then_some(now), now)
        .await?;
    state.cache.invalidate_product_page(&product.handle).await;
    Ok(Json(product))
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ImportJobRequest {
    pub input_path: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ImportJobResponse {
    pub job: ImportJob,
}

#[utoipa::path(
    post,
    path = "/api/import-jobs",
    request_body = ImportJobRequest,
    responses((status = 202, body = ImportJobResponse), (status = 400, body = ErrorEnvelope))
)]
pub async fn create_import_job(
    State(state): State<AppState>,
    Json(input): Json<ImportJobRequest>,
) -> AppResult<(StatusCode, Json<ImportJobResponse>)> {
    let job = worker::enqueue(state.store, input.input_path).await?;
    Ok((StatusCode::ACCEPTED, Json(ImportJobResponse { job })))
}

async fn graphql_handler(
    State(state): State<AppState>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(request.into_inner()).await.into()
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::db::MemoryStore;

    #[tokio::test]
    async fn scenario_health_returns_ok() {
        let app = build_router(Config::from_env(), MemoryStore::shared());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn scenario_validation_error_uses_public_contract() {
        let app = build_router(Config::from_env(), MemoryStore::shared());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/products")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"title":"","handle":"bad","description":"","price_cents":1,"inventory_quantity":1,"published":false}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
