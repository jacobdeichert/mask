use std::path::PathBuf;

use assert_cmd::prelude::*;
use clap::{crate_name, crate_version};
use colored::*;
use predicates::str::contains;

mod common;
use common::MaskCommandExt;

#[test]
fn specifying_a_maskfile_in_a_different_dir() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## foo

<!-- a few more details -->
"#,
    );

    common::run_mask(&maskfile_path)
        .arg("--help")
        .assert()
        .stdout(contains("USAGE:"))
        .success();
}

// Using current_dir(".github") to make sure the default maskfile.md can't be found
mod when_no_maskfile_found_in_current_directory {
    use super::*;

    #[test]
    fn logs_warning_about_missing_maskfile() {
        common::run_mask(&PathBuf::from("./maskfile.md"))
            .current_dir(".github")
            .command("-V")
            .assert()
            .stdout(contains(format!(
                "{} no maskfile.md found",
                "WARNING:".yellow()
            )));
    }

    #[test]
    fn exits_without_error_for_help() {
        common::run_mask(&PathBuf::from("./maskfile.md"))
            .current_dir(".github")
            .command("--help")
            .assert()
            .stdout(contains("USAGE:"))
            .success();
    }

    #[test]
    fn exits_without_error_for_version() {
        common::run_mask(&PathBuf::from("./maskfile.md"))
            .current_dir(".github")
            .command("--version")
            .assert()
            .stdout(contains(format!("{} {}", crate_name!(), crate_version!())))
            .success();
    }

    #[test]
    fn exits_with_error_for_any_other_command() {
        common::run_mask(&PathBuf::from("./maskfile.md"))
            .current_dir(".github")
            .command("nothing")
            .assert()
            .stderr(contains("error: Found argument 'nothing' which wasn't expected, or isn't valid in this context"))
            .failure();
    }
}

mod when_custom_specified_maskfile_not_found {
    use super::*;

    #[test]
    fn exits_with_error_for_help() {
        common::run_mask(&PathBuf::from("./nonexistent.md"))
            .command("--help")
            .assert()
            .stderr(contains(format!(
                "{} specified maskfile not found",
                "ERROR:".red()
            )))
            .failure();
    }

    #[test]
    fn exits_with_error_for_version() {
        common::run_mask(&PathBuf::from("./nonexistent.md"))
            .command("--version")
            .assert()
            .stderr(contains(format!(
                "{} specified maskfile not found",
                "ERROR:".red()
            )))
            .failure();
    }

    #[test]
    fn exits_with_error_for_any_other_command() {
        common::run_mask(&PathBuf::from("./nonexistent.md"))
            .command("what")
            .assert()
            .stderr(contains(format!(
                "{} specified maskfile not found",
                "ERROR:".red()
            )))
            .failure();
    }
}

mod exits_with_the_child_process_status_code {
    use super::*;

    #[test]
    fn exits_with_success() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## success

~~~sh
exit 0
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .command("success")
            .assert()
            .code(0)
            .success();
    }

    #[test]
    fn exits_with_error1() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## failure

~~~sh
exit 1
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .command("failure")
            .assert()
            .code(1)
            .failure();
    }

    #[test]
    fn exits_with_error2() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## failure

~~~sh
exit 2
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .command("failure")
            .assert()
            .code(2)
            .failure();
    }
}
