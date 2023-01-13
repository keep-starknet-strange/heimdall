use crate::models::Interaction;
use eyre::Result;
use influxdb::Client;
use influxdb::InfluxDbWriteable;
use influxdb::WriteQuery;
use std::collections::HashMap;

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
