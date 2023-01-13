use crate::api::*;
use crate::models::*;
use chrono::NaiveDateTime;
use chrono::Utc;
use eyre::{eyre, Result};
use serde_json::Value;
use std::collections::HashMap;

pub async fn pull_data(start: u64) -> Result<HashMap<String, Vec<Interaction>>> {
    let end = chrono::NaiveDateTime::from_timestamp_millis(chrono::Local::now().timestamp_millis())
        .ok_or(eyre!("Could not create the end date"))?;
    let start = end
        .checked_sub_days(chrono::Days::new(start))
        .ok_or(eyre!("Could not create the start date"))?;
    let repos = vec![
        ("keep-starknet-strange", "beerus"),
        ("keep-starknet-strange", "garaga"),
        ("keep-starknet-strange", "quaireaux"),
        ("sayajin-labs", "kakarot"),
    ];
    let token = std::env::var("GH_TOKEN").unwrap();
    let client = reqwest::Client::builder()
        .user_agent("keep-starknet-strange")
        .build()?;
    let mut repos_info: HashMap<String, Vec<Interaction>> = HashMap::new();
    for (owner, repo) in repos {
        let prs_and_issues = gql_query(&client, &QUERY, &token, &owner, &repo).await?;

        let mut infos = vec![];
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

fn parse_interaction(
    interactions: &Vec<Value>,
    target: &mut Vec<Interaction>,
    start: &NaiveDateTime,
    end: &NaiveDateTime,
    interaction_type: String,
    repo: &&str,
) -> Result<()> {
    for interaction in interactions {
        let created_at = chrono::NaiveDateTime::parse_from_str(
            interaction["createdAt"]
                .as_str()
                .ok_or(eyre!("Could not parse created_at"))?,
            "%Y-%m-%dT%H:%M:%SZ",
        )?;

        let ended = if interaction_type == "pr" {
            "mergedAt"
        } else {
            "closedAt"
        };
        let closed_at: Option<NaiveDateTime> = interaction[ended].as_str().map(|closed_at| {
            chrono::NaiveDateTime::parse_from_str(closed_at, "%Y-%m-%dT%H:%M:%SZ").unwrap()
        });
        let time;
        let mut inter = interaction_type.clone();

        if created_at.ge(start) && created_at.lt(end) {
            time = created_at.and_local_timezone(Utc).unwrap();
            inter += " created";
        } else if closed_at.is_some()
            && (closed_at.unwrap().ge(start) && closed_at.unwrap().lt(end))
        {
            time = created_at.and_local_timezone(Utc).unwrap();
            inter += " ended";
        } else {
            break;
        }
        let author = interaction["author"]["login"]
            .as_str()
            .ok_or(eyre!("Could not parse author"))?;
        target.push(Interaction {
            time,
            author: author.to_string(),
            interaction_type: inter,
            repo: repo.to_string(),
        });
    }
    Ok(())
}
