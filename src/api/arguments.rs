use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct AppArguments {
    #[arg(long)]
    pub populate: bool,
    #[arg(long, default_value = "5000")]
    pub port: u16,
}
