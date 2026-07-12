use serde::Deserialize;

use crate::{
    domain::CreateProduct,
    error::{AppError, AppResult},
};

#[derive(Debug, Deserialize)]
pub struct ExternalProduct {
    pub name: String,
    pub slug: String,
    pub body: Option<String>,
    pub price: ExternalPrice,
    pub stock: i32,
    pub live: bool,
}

#[derive(Debug, Deserialize)]
pub struct ExternalPrice {
    pub cents: i32,
}

pub fn external_to_native(input: ExternalProduct) -> AppResult<CreateProduct> {
    if input.slug.trim().is_empty() {
        return Err(AppError::validation("external slug is required"));
    }
    Ok(CreateProduct {
        title: input.name,
        handle: input.slug,
        description: input.body.unwrap_or_default(),
        price_cents: input.price.cents,
        inventory_quantity: input.stock,
        published: input.live,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_external_shape_to_native_command() {
        let native = external_to_native(ExternalProduct {
            name: "Cardamom".into(),
            slug: "cardamom".into(),
            body: Some("Spice".into()),
            price: ExternalPrice { cents: 500 },
            stock: 3,
            live: true,
        })
        .unwrap();

        assert_eq!(native.handle, "cardamom");
        assert!(native.published);
    }
}
