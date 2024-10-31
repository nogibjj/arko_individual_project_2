use std::process::Command;

#[test]
fn test_no_args() {
    let output = Command::new("cargo")
        .args(&["run", "--release"])
        .output()
        .expect("Failed to execute process");

    assert!(
        output.status.success(),
        "Running without arguments should not fail"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Please provide a command"),
        "Expected error message for missing command"
    );
}

#[test]
fn test_invalid_command() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "invalid"])
        .output()
        .expect("Failed to execute process");

    assert!(
        output.status.success(),
        "Running with an invalid command should not fail"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown command"),
        "Expected error message for unknown command"
    );
}

#[test]
fn test_extract_command() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "extract"])
        .output()
        .expect("Failed to execute extract command");

    assert!(output.status.success(), "Extract command failed");

    // Add any specific assertions for expected output here, if applicable
}

#[test]
fn test_query_command() {
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "query"])
        .output()
        .expect("Failed to execute query command");

    assert!(output.status.success(), "Query command failed");

    // Add any specific assertions for expected output here, if applicable
}
