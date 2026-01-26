use cp_unfold::unfold::{split_by_coloncolon, unfold_curly_bracket};

#[test]
fn test_split_by_coloncolon_simple() {
    let result = split_by_coloncolon("std::io::Read".to_string());
    assert_eq!(result, vec!["std", "io", "Read"]);
}

#[test]
fn test_split_by_coloncolon_with_curly_bracket() {
    let result = split_by_coloncolon("std::{io,fs}".to_string());
    assert_eq!(result, vec!["std", "{io,fs}"]);
}

#[test]
fn test_split_by_coloncolon_nested_curly_bracket() {
    let result = split_by_coloncolon("std::{io::{self,Read},fs::File}".to_string());
    assert_eq!(result, vec!["std", "{io::{self,Read},fs::File}"]);
}

#[test]
fn test_unfold_curly_bracket_simple() {
    let input = vec!["std".to_string(), "{io,fs}".to_string()];
    let result = unfold_curly_bracket(&input);
    assert_eq!(result, vec![
        vec!["std", "io"],
        vec!["std", "fs"],
    ]);
}

#[test]
fn test_unfold_curly_bracket_nested() {
    let input = vec!["std".to_string(), "{io::{self,Read},fs::File}".to_string()];
    let result = unfold_curly_bracket(&input);
    assert_eq!(result, vec![
        vec!["std", "io", "self"],
        vec!["std", "io", "Read"],
        vec!["std", "fs", "File"],
    ]);
}

#[test]
fn test_unfold_curly_bracket_double_nested() {
    let input = vec!["std".to_string(), "{io::{self,Read,Write},fs::File}".to_string()];
    let result = unfold_curly_bracket(&input);
    assert_eq!(result, vec![
        vec!["std", "io", "self"],
        vec!["std", "io", "Read"],
        vec!["std", "io", "Write"],
        vec!["std", "fs", "File"],
    ]);
}

#[test]
fn test_unfold_curly_bracket_wildcard() {
    let input = vec!["std".to_string(), "io".to_string(), "*".to_string()];
    let result = unfold_curly_bracket(&input);
    assert_eq!(result, vec![
        vec!["std", "io", "*"],
    ]);
}

#[test]
fn test_unfold_curly_bracket_complex() {
    // use std::{io::{self, Read}, fs::File, collections::HashMap}
    let input = vec![
        "std".to_string(),
        "{io::{self,Read},fs::File,collections::HashMap}".to_string()
    ];
    let result = unfold_curly_bracket(&input);
    assert_eq!(result, vec![
        vec!["std", "io", "self"],
        vec!["std", "io", "Read"],
        vec!["std", "fs", "File"],
        vec!["std", "collections", "HashMap"],
    ]);
}
