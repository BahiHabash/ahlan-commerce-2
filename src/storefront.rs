use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};

use crate::{
    api::AppState,
    error::{AppError, AppResult},
};

#[derive(Clone, Debug)]
pub struct ProductPageContext {
    pub title: String,
    pub description: String,
    pub price: String,
    pub available: bool,
}

pub async fn product_page(
    State(state): State<AppState>,
    Path(handle): Path<String>,
) -> AppResult<impl IntoResponse> {
    if let Some(cached) = state.cache.get_product_page(&handle).await {
        return Ok(Html(cached));
    }

    let product = state
        .store
        .get_product_by_handle(&handle)
        .await?
        .filter(|product| product.published)
        .ok_or_else(|| AppError::NotFound("product page not found".to_string()))?;

    let context = ProductPageContext {
        title: product.title,
        description: product.description,
        price: format!("${:.2}", product.price_cents as f64 / 100.0),
        available: product.inventory_quantity > 0,
    };
    let html = render_product_page(&context);
    state.cache.set_product_page(&handle, html.clone()).await;
    Ok(Html(html))
}

pub fn render_product_page(context: &ProductPageContext) -> String {
    let availability = if context.available {
        "In stock"
    } else {
        "Sold out"
    };
    format!(
        r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>{}</title></head>
<body>
<main>
<h1>{}</h1>
<p>{}</p>
<strong>{}</strong>
<p>{}</p>
</main>
</body>
</html>"#,
        escape(&context.title),
        escape(&context.title),
        escape(&context.description),
        escape(&context.price),
        availability
    )
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
