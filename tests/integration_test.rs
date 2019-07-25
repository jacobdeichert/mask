use assert_cmd::prelude::*;
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
        .arg("--maskfile")
        .arg(maskfile_path)
        .arg("--help")
        .assert()
        .stdout(contains("USAGE:"))
        .success();
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
