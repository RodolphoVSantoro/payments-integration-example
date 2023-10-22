use serde::Serialize;

use super::{
    item::Item,
    layers::{Content, Gallery, Price},
};

#[derive(Serialize)]
pub struct List {
    alias: String,
    metadata: serde_json::Value,
    name: String,
    caption: String,
    content: Vec<Content>,
    gallery: Vec<Gallery>,
    items: Vec<Item>,

    #[serde(rename = "wholePrice")]
    whole_price: Price,

    #[serde(rename = "unifiedKitName")]
    unified_kit_name: String,
}
