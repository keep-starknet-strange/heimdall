use std::fs::OpenOptions;
use std::collections::HashMap;
use csv::Writer;
use chrono::Utc;

use crate::models::Interaction;

/// Display the summary of the interactions
///
/// # Arguments
///
/// * `repo_info` - A HashMap containing the interactions
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
///
/// use github_analytics::models::Interaction;
/// use github_analytics::print_data::display_summary;
///
/// let mut repo_info: HashMap<String, Vec<Interaction>> = HashMap::new();
/// repo_info.insert("repo".to_owned(), vec![]);
/// display_summary(&repo_info);
/// ```
pub fn display_summary(repo_info: &HashMap<String, Vec<Interaction>>, csv: bool) {
    // Create the header of the table.
    let mut lines: Vec<Vec<String>> = vec![vec![
        "Repo".to_owned(),
        "PRs opened".to_owned(),
        "PRs merged".to_owned(),
        "Issues opened".to_owned(),
        "Issues closed".to_owned(),
    ]];
    // Count the different interactions for each repo.
    repo_info.into_iter().for_each(|(repo, interactions)| {
        let mut issues_opened = 0;
        let mut issues_closed = 0;
        let mut prs_opened = 0;
        let mut prs_closed = 0;

        interactions.into_iter().for_each(|interaction| {
            match interaction.interaction_type.as_str() {
                "pr created" => prs_opened += 1,
                "pr ended" => prs_closed += 1,
                "issue created" => issues_opened += 1,
                "issue ended" => issues_closed += 1,
                _ => panic!("Unknown interaction type"),
            }
        });
        lines.push(
            [
                repo.to_owned(),
                prs_opened.to_string(),
                prs_closed.to_string(),
                issues_opened.to_string(),
                issues_closed.to_string(),
            ]
            .to_vec(),
        );
    });
    if csv {
        let now = Utc::now();
        let file_name = format!("kss-report-{}.csv", now.format("%Y-%m-%d"));

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();

        let mut wtr = Writer::from_writer(file);

        lines
            .into_iter()
            .for_each(|line| wtr.serialize(line).unwrap());
    } else {
        let mut out = Vec::new();
        // Format the table.
        text_tables::render(&mut out, lines).unwrap();
        // Print the table.
        println!("{}", std::str::from_utf8(&out).unwrap());
    }
}
