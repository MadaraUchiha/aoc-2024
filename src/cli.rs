use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub day: u8,
}
