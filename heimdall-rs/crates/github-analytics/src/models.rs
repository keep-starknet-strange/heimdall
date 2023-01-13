use chrono::{DateTime, Utc};
use clap::Parser;
use influxdb::InfluxDbWriteable;

#[derive(InfluxDbWriteable, Debug, Clone)]
pub struct Interaction {
    pub time: DateTime<Utc>,
    pub interaction_type: String,
    pub author: String,
    #[influxdb(tag)]
    pub repo: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, short, action)]
    pub print: bool,
    #[clap(long, short, action)]
    pub db: bool,
    pub start: u64,
}
