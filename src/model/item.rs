use serde::Serialize;

use super::{
    layers::{Blocked, Content, Gallery, Package, Price},
    tab::CatalogTab,
};

#[derive(Serialize)]
pub struct Item {
    alias: String,
    metadata: serde_json::Value,
    name: String,
    caption: String,

    #[serde(rename = "inventoryId")]
    inventory_id: Option<String>,
    content: Vec<Content>,
    gallery: Vec<Gallery>,
    price: Price,

    #[serde(rename = "channelId")]
    channel_id: String,

    #[serde(rename = "catalogTab")]
    catalog_tab: CatalogTab,

    #[serde(rename = "priceFrom")]
    price_from: Option<Price>,
    categories: Vec<String>,
    kind: String,
    package: Package,
    shippable: bool,

    #[serde(rename = "itemTyingGroup")]
    item_tying_group: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    chain: Option<Vec<String>>,

    blocked: Option<Blocked>,
}

pub struct DbItem {
    id: String,
    alias: String,
    metadata: String,
    name: String,
    caption: String,
    inventory_id: Option<String>,
    content_id: String,
    gallery_id: String,
    price_amount: i64,
    price_currency: String,
    channel_id: String,
    tab_id: CatalogTab,
    price_from_amount: Option<i64>,
    price_from_currency: Option<String>,
    kind: String,
    package_width: i64,
    package_depth: i64,
    package_height: i64,
    package_weight: f64,
    shippable: bool,
    item_tying_group: String,
    blocked_reason: Option<String>,
}

struct DbItemCategories {
    item_id: String,
    category_id: String,
}

struct DbCategory {
    id: String,
    name: String,
}
