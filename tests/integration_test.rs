use std::process::Command;
use std::path::PathBuf;

#[test]
fn test_super_import() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let test_dir = PathBuf::from(manifest_dir).join("tests/fixtures/test_super");
    
    let output = Command::new("cargo")
        .arg("run")
        .env("CP_UNFOLD_FILE_DIR", test_dir.join("src").to_str().unwrap())
        .env("CP_UNFOLD_LIBRARY_PATH", test_dir.join("library").to_str().unwrap())
        .env("CP_UNFOLD_SRC", "main.rs")
        .env("CP_UNFOLD_LIBRARY_NAME", "library")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // 展開されたコードに base_func と nested_func が含まれているか確認
    assert!(stdout.contains("pub fn base_func()"), "Should contain base_func definition");
    assert!(stdout.contains("pub fn nested_func()"), "Should contain nested_func definition");
    
    // use super:: が展開されていることを確認（展開後は存在しないはず）
    assert!(!stdout.contains("use super::"), "Should not contain 'use super::' after unfolding");
    
    // main関数が含まれていることを確認
    assert!(stdout.contains("fn main()"), "Should contain main function");
}

#[test]
fn test_basic_import() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    
    // 基本的なテストケース用のディレクトリを作成
    let test_dir = PathBuf::from(manifest_dir).join("tests/fixtures/test_basic");
    std::fs::create_dir_all(test_dir.join("library")).ok();
    
    // テストファイルを作成
    std::fs::write(
        test_dir.join("library/math.rs"),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }"
    ).ok();
    
    std::fs::write(
        test_dir.join("main.rs"),
        "use library::math::*;\nfn main() { println!(\"{}\", add(1, 2)); }"
    ).ok();
    
    let output = Command::new("cargo")
        .arg("run")
        .env("CP_UNFOLD_FILE_DIR", test_dir.to_str().unwrap())
        .env("CP_UNFOLD_SRC", "main.rs")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("pub fn add"), "Should contain add function");
    assert!(stdout.contains("fn main()"), "Should contain main function");
}

#[test]
fn test_curly_bracket_import() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let test_dir = PathBuf::from(manifest_dir).join("tests/fixtures/test_curly");
    std::fs::create_dir_all(test_dir.join("library")).ok();
    
    std::fs::write(
        test_dir.join("library/utils.rs"),
        "pub fn foo() {}\npub fn bar() {}"
    ).ok();
    
    std::fs::write(
        test_dir.join("main.rs"),
        "use library::utils::{foo, bar};\nfn main() {}"
    ).ok();
    
    let output = Command::new("cargo")
        .arg("run")
        .env("CP_UNFOLD_FILE_DIR", test_dir.to_str().unwrap())
        .env("CP_UNFOLD_SRC", "main.rs")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("pub fn foo"), "Should contain foo function");
    assert!(stdout.contains("pub fn bar"), "Should contain bar function");
}
