use clap::Parser;
use github_analytics::models::Cli;
use github_analytics::print_data::display_summary;
use github_analytics::pull_data::pull_data;
use github_analytics::push_data::push_data;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let repo_infos = pull_data(cli.start).await.unwrap();
    if cli.db {
        push_data(&repo_infos).await.unwrap();
    }
    if cli.print {
        display_summary(&repo_infos);
    }
}
