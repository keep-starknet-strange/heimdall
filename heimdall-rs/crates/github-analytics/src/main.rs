use github_analytics::pull_data::pull_data;
use github_analytics::push_data::push_data;

#[tokio::main]
async fn main() {
    let repo_infos = pull_data().await.unwrap();
    push_data(repo_infos).await.unwrap();
}
