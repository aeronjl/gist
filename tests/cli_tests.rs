use assert_cmd::Command;

#[test]
fn test_cli_fetch_abstract() {
    let mut cmd = Command::cargo_bin("gist").unwrap();
    let assert = cmd
        .arg("--url")
        .arg("https://pubmed.ncbi.nlm.nih.gov/21150120/")
        .assert();
    
    assert
        .success()
        .stdout(predicates::str::contains("peroxisome proliferator-activated receptor signaling"));
}

#[test]
fn test_cli_summarize_abstract() {
    let mut cmd = Command::cargo_bin("gist").unwrap();
    let assert = cmd
        .arg("--url")
        .arg("https://pubmed.ncbi.nlm.nih.gov/21150120/")
        .arg("--short")
        .assert();
    
    assert
        .success()
        .stdout(predicates::str::contains("Summary:"));
}