use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[clap(short, long)]
    pub format: Option<Format>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Print all .env variables
    List(ListArgs),

    /// List all env files
    ListFiles,

    /// Set environment
    Set { key: String, value: String },

    // don't use flags
    /// Get environment
    Get { key: String },
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: SubCommand,

    /// Set stage
    #[clap(short, long, global = true)]
    pub stage: Option<String>,
}
