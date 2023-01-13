use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};
use eyre::{eyre, Result};
use serde_json::Value;

use crate::api::*;
use crate::models::*;

/// Fetch the data from the github api.
///
/// # Arguments
///
/// * `start` - The number of days to go back from today.
///
/// # Returns
///
/// A hashmap with the repo name as key and a vector of interactions as value.
pub async fn pull_data(start: u64) -> Result<HashMap<String, Vec<Interaction>>> {
    // End date of the interactions (today).
    let end = chrono::NaiveDateTime::from_timestamp_millis(chrono::Local::now().timestamp_millis())
        .ok_or(eyre!("Could not create the end date"))?;
    // Start date of the interactions (today - specified number of days).
    let start = end
        .checked_sub_days(chrono::Days::new(start))
        .ok_or(eyre!("Could not create the start date"))?;
    // Repositories to fetch the data from.
    let repos = vec![
        ("keep-starknet-strange", "beerus"),
        ("keep-starknet-strange", "garaga"),
        ("keep-starknet-strange", "quaireaux"),
        ("sayajin-labs", "kakarot"),
    ];
    // Github token.
    let token = std::env::var("GH_TOKEN").unwrap();
    // Reqwest client.
    let client = reqwest::Client::builder().user_agent("keep-starknet-strange").build()?;
    // HashMap that will contain the interactions.
    let mut repos_info: HashMap<String, Vec<Interaction>> = HashMap::new();
    for (owner, repo) in repos {
        // Query to fetch the data.
        let prs_and_issues = gql_query(&client, &QUERY, &token, &owner, &repo).await?;
        // Vector that will contain the interactions.
        let mut infos = vec![];
        // Parse the pull requests.
        parse_interaction(
            prs_and_issues["data"]["repository"]["pullRequests"]["nodes"]
                .as_array()
                .ok_or(eyre!("Could not fetch the pull requests"))?,
            &mut infos,
            &start,
            &end,
            "pr".to_owned(),
            &repo,
        )?;

        // Parse the issues.
        parse_interaction(
            prs_and_issues["data"]["repository"]["issues"]["nodes"]
                .as_array()
                .ok_or(eyre!("Could not fetch the issues"))?,
            &mut infos,
            &start,
            &end,
            "issue".to_owned(),
            &repo,
        )?;
        repos_info.insert(repo.to_owned(), infos);
    }
    Ok(repos_info)
}

/// Parse the interactions.
///
/// # Arguments
///
/// * `interactions` - The interactions to parse.
/// * `target` - The vector that will contain the parsed interactions.
/// * `start` - The start date of the interactions.
/// * `end` - The end date of the interactions.
/// * `interaction_type` - The type of the interaction (pr or issue).
/// * `repo` - The name of the repository.
fn parse_interaction(
    interactions: &Vec<Value>,
    target: &mut Vec<Interaction>,
    start: &NaiveDateTime,
    end: &NaiveDateTime,
    interaction_type: String,
    repo: &&str,
) -> Result<()> {
    for interaction in interactions {
        // Parse the created_at date.
        let created_at = chrono::NaiveDateTime::parse_from_str(
            interaction["createdAt"].as_str().ok_or(eyre!("Could not parse created_at"))?,
            "%Y-%m-%dT%H:%M:%SZ",
        )?;

        // Parse the closed_at date.
        let ended = if interaction_type == "pr" { "mergedAt" } else { "closedAt" };
        // If the interaction is closed, parse the closed_at date.
        let closed_at: Option<NaiveDateTime> = interaction[ended].as_str().map(|closed_at| {
            chrono::NaiveDateTime::parse_from_str(closed_at, "%Y-%m-%dT%H:%M:%SZ").unwrap()
        });
        // Declare the time variable.
        let time;
        // Declare the interaction type variable.
        let mut inter = interaction_type.clone();

        // If the interaction was created in the desired timeframe add created to the interaction
        // type.
        if created_at.ge(start) && created_at.lt(end) {
            time = created_at.and_local_timezone(Utc).unwrap();
            inter += " created";
            // If the interaction was closed in the desired timeframe add ended to the interaction
            // type.
        } else if closed_at.is_some()
            && (closed_at.unwrap().ge(start) && closed_at.unwrap().lt(end))
        {
            time = created_at.and_local_timezone(Utc).unwrap();
            inter += " ended";
            // If the interaction was not created or closed in the desired timeframe, end the loop.
        } else {
            break;
        }
        // Parse the author.
        let author =
            interaction["author"]["login"].as_str().ok_or(eyre!("Could not parse author"))?;
        // Push the interaction to the vector.
        target.push(Interaction {
            time,
            author: author.to_string(),
            interaction_type: inter,
            repo: repo.to_string(),
        });
    }
    Ok(())
}
