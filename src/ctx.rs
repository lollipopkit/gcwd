use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "gcwd", author = "lollipopkit", version = env!("CARGO_PKG_VERSION"))]
pub struct Ctx {
    #[arg(short, long, default_value_t = false, help = "Check gcwd update")]
    pub update: bool,

    #[arg(short, long, help = "Commit message")]
    pub message: Option<String>,

    pub time: Option<String>,
}
