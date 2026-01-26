use std::fs;
use std::path::PathBuf;
use std::process::Command;
use cp_unfold::{Unfold, Config};

fn setup_test_project(test_name: &str) -> PathBuf {
    let test_dir = PathBuf::from(format!("/tmp/cp_unfold_test_{}", test_name));
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).unwrap();
    }
    fs::create_dir_all(&test_dir).unwrap();
    test_dir
}

fn run_cp_unfold(file_dir: &str, src: &str, library_name: &str) -> String {
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_dir, "-s", src, "-l", library_name])
        .output()
        .expect("Failed to execute cp_unfold");

    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_simple_library_import() {
    let test_dir = setup_test_project("simple");
    let src_dir = test_dir.join("src");
    let lib_dir = src_dir.join("library");
    fs::create_dir_all(&lib_dir).unwrap();

    // library/math.rs
    fs::write(
        lib_dir.join("math.rs"),
        r#"pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
"#,
    )
    .unwrap();

    // library/mod.rs
    fs::write(lib_dir.join("mod.rs"), "pub mod math;\n").unwrap();

    // main.rs
    fs::write(
        src_dir.join("main.rs"),
        r#"mod library;
use library::math::gcd;

fn main() {
    println!("{}", gcd(12, 8));
}
"#,
    )
    .unwrap();

    let output = run_cp_unfold(src_dir.to_str().unwrap(), "main.rs", "library");

    assert!(output.contains("pub fn gcd(a: i64, b: i64) -> i64"));
    assert!(output.contains("fn main()"));
    assert!(!output.contains("mod library"));
    assert!(!output.contains("use library::math::gcd"));

    fs::remove_dir_all(&test_dir).unwrap();
}

#[test]
fn test_multiline_use_statement() {
    let test_dir = setup_test_project("multiline");
    let src_dir = test_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // main.rs with multiline use
    fs::write(
        src_dir.join("main.rs"),
        r#"use std::{
    io::{self, Read},
    fs::File,
    collections::HashMap
};

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 2);
}
"#,
    )
    .unwrap();

    let output = run_cp_unfold(src_dir.to_str().unwrap(), "main.rs", "library");

    // すべての import が展開されていることを確認
    assert!(output.contains("use std::io::Read"));
    assert!(output.contains("use std::io::self"));
    assert!(output.contains("use std::fs::File"));
    assert!(output.contains("use std::collections::HashMap"));
    assert!(output.contains("fn main()"));

    fs::remove_dir_all(&test_dir).unwrap();
}

#[test]
fn test_nested_curly_brackets() {
    let test_dir = setup_test_project("nested");
    let src_dir = test_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // main.rs with deeply nested use
    fs::write(
        src_dir.join("main.rs"),
        r#"use std::{io::{self, Read, Write}, fs::File};

fn main() {}
"#,
    )
    .unwrap();

    let output = run_cp_unfold(src_dir.to_str().unwrap(), "main.rs", "library");

    assert!(output.contains("use std::io::self"));
    assert!(output.contains("use std::io::Read"));
    assert!(output.contains("use std::io::Write"));
    assert!(output.contains("use std::fs::File"));

    fs::remove_dir_all(&test_dir).unwrap();
}

#[test]
fn test_wildcard_import() {
    let test_dir = setup_test_project("wildcard");
    let src_dir = test_dir.join("src");
    let lib_dir = src_dir.join("library");
    fs::create_dir_all(&lib_dir).unwrap();

    // library/utils.rs
    fs::write(
        lib_dir.join("utils.rs"),
        r#"pub fn helper1() {}
pub fn helper2() {}
"#,
    )
    .unwrap();

    // library/mod.rs
    fs::write(lib_dir.join("mod.rs"), "pub mod utils;\n").unwrap();

    // main.rs
    fs::write(
        src_dir.join("main.rs"),
        r#"mod library;
use library::utils::*;

fn main() {
    helper1();
}
"#,
    )
    .unwrap();

    let output = run_cp_unfold(src_dir.to_str().unwrap(), "main.rs", "library");

    assert!(output.contains("pub fn helper1()"));
    assert!(output.contains("pub fn helper2()"));
    assert!(!output.contains("mod library"));

    fs::remove_dir_all(&test_dir).unwrap();
}

#[test]
fn test_super_import() {
    let test_dir = setup_test_project("super");
    let src_dir = test_dir.join("src");
    let lib_dir = src_dir.join("library");
    fs::create_dir_all(&lib_dir).unwrap();

    // library/common.rs
    fs::write(
        lib_dir.join("common.rs"),
        r#"pub fn common_func() -> i32 { 42 }
"#,
    )
    .unwrap();

    // library/module.rs
    fs::write(
        lib_dir.join("module.rs"),
        r#"use super::common::common_func;

pub fn use_common() -> i32 {
    common_func()
}
"#,
    )
    .unwrap();

    // library/mod.rs
    fs::write(
        lib_dir.join("mod.rs"),
        "pub mod common;\npub mod module;\n",
    )
    .unwrap();

    // main.rs
    fs::write(
        src_dir.join("main.rs"),
        r#"mod library;
use library::module::use_common;

fn main() {
    println!("{}", use_common());
}
"#,
    )
    .unwrap();

    let output = run_cp_unfold(src_dir.to_str().unwrap(), "main.rs", "library");

    assert!(output.contains("pub fn common_func()"));
    assert!(output.contains("pub fn use_common()"));
    assert!(!output.contains("use super::common::common_func"));

    fs::remove_dir_all(&test_dir).unwrap();
}

// ライブラリAPIを直接使用するテスト（lib.rs 作成により可能になった）
#[test]
fn test_library_api_direct_usage() {
    let test_dir = setup_test_project("lib_api");
    let src_dir = test_dir.join("src");
    let lib_dir = src_dir.join("library");
    fs::create_dir_all(&lib_dir).unwrap();

    // library/utils.rs
    fs::write(
        lib_dir.join("utils.rs"),
        r#"pub fn add(a: i32, b: i32) -> i32 { a + b }
"#,
    )
    .unwrap();

    // library/mod.rs
    fs::write(lib_dir.join("mod.rs"), "pub mod utils;\n").unwrap();

    // main.rs
    fs::write(
        src_dir.join("main.rs"),
        r#"mod library;
use library::utils::add;

fn main() {
    println!("{}", add(1, 2));
}
"#,
    )
    .unwrap();

    // ライブラリAPIを直接使用
    let mut unfold = Unfold::from_args(
        "main.rs".to_string(),
        "library".to_string(),
        src_dir.clone(),
        None,
    );

    let result = unfold.unfold().expect("unfold should succeed");

    // 結果を検証
    assert!(result.contains("pub fn add(a: i32, b: i32) -> i32"));
    assert!(result.contains("fn main()"));
    assert!(!result.contains("mod library"));
    assert!(!result.contains("use library::utils::add"));

    fs::remove_dir_all(&test_dir).unwrap();
}
