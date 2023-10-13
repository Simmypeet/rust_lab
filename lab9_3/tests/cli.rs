use std::path::PathBuf;

use assert_cmd::Command;

#[test]
fn dies_bad_args() {
    // non-2 arguments case
    {
        let mut cmd = Command::cargo_bin("layer").unwrap();
        cmd.arg("first").arg("second").arg("third");
        cmd.assert().failure();
    }

    // no arguments case
    {
        let mut cmd = Command::cargo_bin("layer").unwrap();
        cmd.assert().failure();
    }

    // non-number argument case
    {
        let mut cmd = Command::cargo_bin("layer").unwrap();
        cmd.arg("first").arg("second");
        cmd.assert().failure();
    }
}

#[test]
fn test_valid() {
    let temp_dir = tempfile::tempdir().unwrap();
    let out_file_path = temp_dir.path().join("output.csv");

    let mut cmd = Command::cargo_bin("layer").unwrap();
    cmd.arg("10").arg(temp_dir.path().join(&out_file_path));

    cmd.assert().success();

    let output = std::fs::read_to_string(&out_file_path).unwrap();
    assert_eq!(output.lines().count(), 10);

    for (index, line) in output.lines().enumerate() {
        // test name part
        let parts = line.split(',').collect::<Vec<_>>();
        assert_eq!(parts[0].trim(), format!("\"Layer {}\"", index));

        // test color part
        assert_eq!(parts[1].trim().len(), 9); // #FFFFFFFF 9 characters in total
        assert_eq!(parts[1].trim().chars().nth(0).unwrap(), '#');

        for point_index in 0..(parts.len() - 2) / 2 {
            let x_index = point_index * 2 + 2;
            let y_index = x_index + 1;

            // should be able to parse into a number
            parts[x_index].trim().parse::<f64>().unwrap();
            parts[y_index].trim().parse::<f64>().unwrap();
        }
    }
}
