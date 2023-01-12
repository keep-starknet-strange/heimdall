use std::collections::HashMap;

use crate::models::Interaction;
use chrono::{DateTime, Utc};
use eyre::Result;
use influxdb::Client;
use influxdb::InfluxDbWriteable;

pub async fn push_data(repo_infos: HashMap<String, Vec<Interaction>>) -> Result<()> {
    #[derive(InfluxDbWriteable)]
    pub struct DbRepoInfo {
        time: DateTime<Utc>,
        interaction_type: String,
        ended_at: String,
        author: String,
        #[influxdb(tag)]
        repo: String,
    }
    let client = Client::new("http://localhost:8086", "test");
    let mut interactions = vec![];
    for (repo, inters) in repo_infos.into_iter() {
        for inter in inters {
            interactions.push(
                DbRepoInfo {
                    time: inter.created_at.and_local_timezone(Utc).unwrap(),
                    interaction_type: inter.interaction_type,
                    ended_at: if inter.ended_at.is_some() {
                        inter.ended_at.unwrap().to_string()
                    } else {
                        "Not yet".to_owned()
                    },
                    author: inter.author,
                    repo: repo.clone(),
                }
                .into_query("interactions"),
            );
        }
    }
    let write_result = client.query(interactions).await;
    write_result.unwrap();

    Ok(())
}
