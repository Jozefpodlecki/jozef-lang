use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    about = "The compiler that interprets a jozef-lang"
)]
pub struct CommandArgs {
    /// Input source file
    #[arg(long)]
    pub input: String,

    /// Output file (optional for interpreter mode)
    #[arg(long)]
    pub output: Option<String>,
}