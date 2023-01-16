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
    /// Should print the summary.
    #[clap(long, short, action)]
    pub print: bool,
    /// Should save the data in an influxdb instance.
    #[clap(long, short, action)]
    pub db: bool,
    /// Number of days in the past the data analysis should start.
    pub start: u64,
}
