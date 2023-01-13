use std::collections::HashMap;

use crate::models::Interaction;

pub fn display_summary(repo_info: &HashMap<String, Vec<Interaction>>) {
    let mut lines: Vec<Vec<String>> = vec![vec![
        "Repo".to_owned(),
        "PRs opened".to_owned(),
        "PRs merged".to_owned(),
        "Issues opened".to_owned(),
        "Issues closed".to_owned(),
    ]];
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
    let mut out = Vec::new();
    text_tables::render(&mut out, lines).unwrap();
    println!("{}", std::str::from_utf8(&out).unwrap());
}
