use std::fs::read_to_string;
use std::path::PathBuf;

use comment_parser::CommentParser;
use walkdir::WalkDir;

/// Check if all the comments are properly formatted panics if not.
pub fn check_comments(path: &PathBuf) {
    // Get rust parser.
    let rules = comment_parser::get_syntax("rust").unwrap();
    // Get all the rust files in the dir and sub dirs, read them and concat them into a single
    // String.
    let rust_code = WalkDir::new(path)
        .into_iter()
        .filter(|x| x.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".rs"))
        .map(|x| read_to_string(x.unwrap().path()).unwrap())
        .reduce(|a, b| a + "\n" + &b)
        .unwrap();

    // For each assert if it's properly formatted.
    CommentParser::new(&rust_code, rules).for_each(|comment| {
        let com = comment.raw().trim();
        if com.starts_with("// ") {
            assert!(
                com.chars().nth(3).unwrap().is_uppercase() && com.ends_with('.'),
                "{}",
                format!("Badly formatted comment {:#?}", com)
            )
        }
    });
    // Everything went through.
    println!("All comments are properly formatted")
}
