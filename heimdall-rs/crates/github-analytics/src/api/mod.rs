use eyre::{eyre, Result};

// Graphql query that will fetch all the issues and pull requests for the specified repo.
pub const QUERY: &str = r#"query ($name: String!, $owner: String!) {
    repository(name: $name, owner: $owner) {
      pullRequests(first: 100, orderBy: {field: CREATED_AT, direction: DESC}) {
        nodes {
          createdAt
          mergedAt
          author {
            login
          }
        }
      }
      issues(first: 100, orderBy: {field: CREATED_AT, direction: DESC}) {
        nodes {
          author {
            login
          }
          createdAt
          closedAt
        }
      }
    }
  }"#;

// GitHub API url.
const URL: &str = "https://api.github.com/graphql";

/// Returns the result of a graphql query to the github api.
///
/// # Arguments
///
/// * `client` - The reqwest client.
/// * `query` - The graphql query.
/// * `token` - The github token.
/// * `owner` - The owner of the repo.
/// * `repo` - The name of the repo.
pub async fn gql_query(
    client: &reqwest::Client,
    query: &&str,
    token: &String,
    owner: &&str,
    repo: &&str,
) -> Result<serde_json::Value> {
    client
        .post(URL)
        .bearer_auth(token)
        .json(&serde_json::json!({
            "query": query,
            "variables": {
                "owner": owner,
                "name": repo,
            }
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| eyre!(e))
}
