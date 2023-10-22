use serde::Serialize;

#[derive(Serialize)]
pub struct Content {
    kind: String,
    markdown: String,
}
#[derive(Serialize)]
pub struct Gallery {
    thumb: String,
    url: String,
}
#[derive(Serialize)]
pub struct Price {
    amount: f64,
    currency: String,
}
#[derive(Serialize)]
pub struct Package {
    width: i64,
    depth: i64,
    height: i64,
    weight: f64,
}
#[derive(Serialize)]
pub struct Blocked {
    reason: String,
}
