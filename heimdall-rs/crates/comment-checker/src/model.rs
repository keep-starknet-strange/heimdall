use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Main struct for the Heimdall CLI args.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the directory to check.
    #[arg(short, long)]
    pub dir_path: PathBuf,
    /// List of supported commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// List of supported commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Comments related subcommands
    Comments(CommentsCommands),
}

/// Comments related commands.
#[derive(Parser, Debug)]
pub struct CommentsCommands {
    /// Comments related subcommands.
    #[command(subcommand)]
    pub command: CommentsSubCommands,
}

/// Comments related subcommands.
#[derive(Subcommand, Debug)]
pub enum CommentsSubCommands {
    /// Check if the comments are properly formatted.
    CheckComments,
}
