use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::{AppError, AppResult};

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Clone, Copy, Debug)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    async_graphql::SimpleObject,
    ToSchema,
    sqlx::FromRow,
)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub handle: String,
    pub description: String,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, async_graphql::InputObject, ToSchema)]
pub struct CreateProduct {
    pub title: String,
    pub handle: String,
    pub description: String,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, async_graphql::InputObject, ToSchema)]
pub struct UpdatePublication {
    pub published: bool,
}

pub fn new_product(input: CreateProduct, clock: &dyn Clock) -> AppResult<Product> {
    validate_product(&input)?;
    let now = clock.now();
    Ok(Product {
        id: Uuid::now_v7(),
        title: input.title.trim().to_string(),
        handle: input.handle.trim().to_string(),
        description: input.description.trim().to_string(),
        price_cents: input.price_cents,
        inventory_quantity: input.inventory_quantity,
        published: input.published,
        published_at: input.published.then_some(now),
        created_at: now,
        updated_at: now,
    })
}

pub fn publish_product(
    mut product: Product,
    input: UpdatePublication,
    clock: &dyn Clock,
) -> Product {
    let now = clock.now();
    product.published = input.published;
    product.published_at = input.published.then_some(now);
    product.updated_at = now;
    product
}

fn validate_product(input: &CreateProduct) -> AppResult<()> {
    if input.title.trim().is_empty() {
        return Err(AppError::validation("title is required"));
    }
    if input.handle.trim().is_empty() {
        return Err(AppError::validation("handle is required"));
    }
    if !input
        .handle
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(AppError::validation(
            "handle must use lowercase letters, numbers, or hyphens",
        ));
    }
    if input.price_cents < 0 {
        return Err(AppError::validation("price_cents must be non-negative"));
    }
    if input.inventory_quantity < 0 {
        return Err(AppError::validation(
            "inventory_quantity must be non-negative",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    struct FixedClock;

    impl Clock for FixedClock {
        fn now(&self) -> DateTime<Utc> {
            Utc.with_ymd_and_hms(2026, 1, 2, 3, 4, 5).unwrap()
        }
    }

    #[test]
    fn scenario_product_create_sets_id_and_time() {
        let product = new_product(
            CreateProduct {
                title: "Coffee".into(),
                handle: "coffee".into(),
                description: "Fresh beans".into(),
                price_cents: 1200,
                inventory_quantity: 10,
                published: true,
            },
            &FixedClock,
        )
        .unwrap();

        assert_eq!(product.created_at, FixedClock.now());
        assert_eq!(product.published_at, Some(FixedClock.now()));
        assert_eq!(product.id.get_version_num(), 7);
    }

    #[test]
    fn scenario_product_create_validates_handle() {
        let error = new_product(
            CreateProduct {
                title: "Coffee".into(),
                handle: "Bad Handle".into(),
                description: String::new(),
                price_cents: 1200,
                inventory_quantity: 10,
                published: false,
            },
            &FixedClock,
        )
        .unwrap_err();
        assert!(matches!(error, AppError::Validation(_)));
    }
}
