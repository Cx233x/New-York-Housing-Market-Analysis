use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ny-housing-graph"))
        .args(args)
        .output()
        .expect("run CLI")
}

fn fixture() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/categories.csv")
}

#[test]
fn categories_are_sorted_including_isolated_nodes() {
    let result = run(&["--data", fixture(), "--list-types"]);
    assert!(result.status.success());
    assert_eq!(String::from_utf8(result.stdout).unwrap(), "A\nB\nC\n");
}

#[test]
fn both_algorithms_print_routes_and_agree() {
    let result = run(&[
        "--data",
        fixture(),
        "--start",
        "A",
        "--end",
        "B",
        "--algorithm",
        "both",
        "--repeat",
        "3",
    ]);
    assert!(result.status.success());
    let output = String::from_utf8(result.stdout).unwrap();
    assert!(output.contains("heap route: A -> B"));
    assert!(output.contains("scan route: A -> B"));
    assert!(output.contains("Cost/reachability agreement: yes"));
    for name in ["heap", "scan"] {
        let row = output
            .lines()
            .find(|line| line.split_whitespace().next() == Some(name))
            .unwrap();
        assert_eq!(
            &row.split_whitespace().collect::<Vec<_>>()[1..3],
            &["2", "3"]
        );
    }
}

#[test]
fn unreachable_and_unknown_categories_are_distinct() {
    let result = run(&["--data", fixture(), "--start", "A", "--end", "C"]);
    assert!(result.status.success());
    assert!(String::from_utf8(result.stdout)
        .unwrap()
        .contains("unreachable"));
    let result = run(&["--data", fixture(), "--start", "missing", "--end", "C"]);
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("unknown category: missing"));
}

#[test]
fn help_does_not_load_data_and_invalid_arguments_fail() {
    let result = run(&["--data", "/nonexistent/ny-housing.csv", "--help"]);
    assert!(result.status.success());
    assert!(String::from_utf8(result.stdout).unwrap().contains("Usage:"));
    assert!(!run(&["--repeat", "0"]).status.success());
    assert!(!run(&["--data", "/nonexistent/ny-housing.csv"])
        .status
        .success());
}
