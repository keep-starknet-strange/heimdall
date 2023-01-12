use eyre::{eyre, Result};

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

const URL: &str = "https://api.github.com/graphql";

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
