use async_graphql::{Context, EmptySubscription, Object, Schema};

use crate::{
    api::AppState,
    domain::{CreateProduct, Product, SystemClock, UpdatePublication, new_product},
    error::AppError,
};

pub type AhlanSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn schema(state: AppState) -> AhlanSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state)
        .finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn products(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Product>> {
        let state = ctx.data::<AppState>()?;
        state.store.list_products().await.map_err(to_graphql_error)
    }

    async fn published_products(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Product>> {
        let state = ctx.data::<AppState>()?;
        state
            .store
            .list_published_products()
            .await
            .map_err(to_graphql_error)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_product(
        &self,
        ctx: &Context<'_>,
        input: CreateProduct,
    ) -> async_graphql::Result<Product> {
        let state = ctx.data::<AppState>()?;
        let product = new_product(input, &SystemClock).map_err(to_graphql_error)?;
        let created = state
            .store
            .create_product(product)
            .await
            .map_err(to_graphql_error)?;
        state.cache.invalidate_product_page(&created.handle).await;
        Ok(created)
    }

    async fn update_product_publication(
        &self,
        ctx: &Context<'_>,
        id: uuid::Uuid,
        input: UpdatePublication,
    ) -> async_graphql::Result<Product> {
        let state = ctx.data::<AppState>()?;
        let now = chrono::Utc::now();
        let product = state
            .store
            .update_product_publication(id, input.clone(), input.published.then_some(now), now)
            .await
            .map_err(to_graphql_error)?;
        state.cache.invalidate_product_page(&product.handle).await;
        Ok(product)
    }
}

fn to_graphql_error(error: AppError) -> async_graphql::Error {
    async_graphql::Error::new(error.to_string())
}
