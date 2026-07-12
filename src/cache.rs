use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use tracing::info;

pub const PRODUCT_PAGE_KEY_PREFIX: &str = "storefront:product:";

#[derive(Clone, Default)]
pub struct StorefrontCache {
    pages: Arc<RwLock<HashMap<String, String>>>,
}

impl StorefrontCache {
    pub fn key_for_handle(handle: &str) -> String {
        format!("{PRODUCT_PAGE_KEY_PREFIX}{handle}")
    }

    pub async fn get_product_page(&self, handle: &str) -> Option<String> {
        let key = Self::key_for_handle(handle);
        let page = self.pages.read().await.get(&key).cloned();
        info!(cache_key = %key, cache_hit = page.is_some(), "storefront cache lookup");
        page
    }

    pub async fn set_product_page(&self, handle: &str, html: String) {
        let key = Self::key_for_handle(handle);
        self.pages.write().await.insert(key.clone(), html);
        info!(cache_key = %key, "storefront cache write");
    }

    pub async fn invalidate_product_page(&self, handle: &str) {
        let key = Self::key_for_handle(handle);
        self.pages.write().await.remove(&key);
        info!(cache_key = %key, "storefront cache invalidated");
    }
}
