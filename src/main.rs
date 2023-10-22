pub mod api;
pub mod items;
pub mod model;
pub mod tabs;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = api::arguments::AppArguments::parse();
    if args.populate {
        model::populate::populate().await;
    } else {
        api::server::serve(args).await;
    }
}
