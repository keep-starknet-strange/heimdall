use crate::models::Interaction;
use eyre::Result;
use influxdb::{Client, InfluxDbWriteable, WriteQuery};
use std::collections::HashMap;

/// Push the data to the influxdb database.
///
/// # Arguments
///
/// * `repo_infos` - A HashMap containing the interactions
pub async fn push_data(repo_infos: &HashMap<String, Vec<Interaction>>) -> Result<()> {
    let client = Client::new("http://localhost:8086", "test");
    let interactions = repo_infos
        .clone()
        .into_values()
        .flatten()
        .map(|inter| inter.into_query("interaction"))
        .collect::<Vec<WriteQuery>>();

    let write_result = client.query(interactions).await;
    write_result.unwrap();

    Ok(())
}
