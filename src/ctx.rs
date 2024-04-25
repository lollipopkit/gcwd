use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "gcwd", author = "lollipopkit")]
pub struct Ctx {
    #[arg(short, long, default_value_t = false, help = "Skip update check")]
    pub skip_update: bool,

    #[arg(short, long, help = "Commit message")]
    pub message: Option<String>,

    pub time: String,
}
