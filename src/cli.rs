use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Branch(BranchArgs),
}

#[derive(Args, Debug)]
pub struct BranchArgs {
    #[arg(short, long)]
    pub delete: bool,
}
