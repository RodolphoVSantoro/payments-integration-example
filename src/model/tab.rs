use serde::Serialize;

#[derive(Serialize)]
pub struct Tab {
    title: String,
    pub alias: String,
    metadata: serde_json::Value,
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

pub struct DbTab {
    title: String,
    pub alias: String,
    metadata: String,
    pub channel_id: String,
}

#[derive(Serialize)]
pub struct CatalogTab {
    title: String,
    pub alias: String,
    metadata: serde_json::Value,
    channel: TabChannel,
}

#[derive(Serialize)]
pub struct TabChannel {
    pub id: String,
    pub slug: String,
    pub metadata: serde_json::Value,
}
