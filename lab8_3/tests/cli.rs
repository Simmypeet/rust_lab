use std::path::PathBuf;

use assert_cmd::Command;

#[test]
fn test_valid() {
    let input = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("input.csv");

    let temp_dir = tempfile::tempdir().unwrap();

    let mut cmd = Command::cargo_bin("pointr").unwrap();
    cmd.arg(input).arg(temp_dir.path().join("output.csv"));

    cmd.assert().success();

    let output = std::fs::read_to_string(temp_dir.path().join("output.csv")).unwrap();
    let expected = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("expected.csv"),
    )
    .unwrap();
    assert_eq!(output, expected)
}

#[test]
fn test_invalid() {
    // invalid file case
    {
        let input = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("whatFile.csv");

        let temp_dir = tempfile::tempdir().unwrap();

        let mut cmd = Command::cargo_bin("pointr").unwrap();
        cmd.arg(input).arg(temp_dir.path().join("output.csv"));

        cmd.assert().failure();
    }

    // invalid no-arguments case
    {
        let mut cmd = Command::cargo_bin("pointr").unwrap();
        cmd.assert().failure();
    }

    // non-2 arguments case

    {
        let mut cmd = Command::cargo_bin("pointr").unwrap();
        cmd.arg("first").arg("second").arg("third");
        cmd.assert().failure();
    }
}
