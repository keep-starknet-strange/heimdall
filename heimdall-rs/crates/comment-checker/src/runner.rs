use clap::Parser;

use super::comments::check_comments;
use super::model::{Cli, Commands, CommentsSubCommands};

/// Main entry point for the Heimdall CLI.
pub fn run() {
    // Parse the CLI arguments.
    let cli = Cli::parse();
    // Dispatch the CLI command.
    match &cli.command {
        Commands::Comments(comments_commands) => match &comments_commands.command {
            CommentsSubCommands::CheckComments => check_comments(&cli.dir_path),
        },
    }
}
