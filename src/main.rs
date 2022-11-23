use comment_parser::{CommentParser, Event};
use std::fs::read_to_string;
use walkdir::WalkDir;

fn main() {
    let rules = comment_parser::get_syntax("rust").unwrap();
    let files = WalkDir::new(".").into_iter();
    let res = files
        .filter(|x| {
            x.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .ends_with(".rs")
        })
        .map(|x| read_to_string(x.unwrap().path()).unwrap())
        .reduce(|a, b| a + &b)
        .unwrap();
    let parser: Vec<String> = CommentParser::new(&res, rules)
        .map(|comment| comment.raw().trim().to_owned())
        .into_iter()
        .filter(|comment| {
            !comment.starts_with("///")
                && (!comment.chars().nth(3).unwrap().is_uppercase() || !comment.ends_with("."))
        })
        .collect();

    assert!(
        parser.len() == 0,
        "{}",
        format!("Badly formatted comments {:?}", parser)
    )
}
