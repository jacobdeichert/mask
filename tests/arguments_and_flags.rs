use assert_cmd::prelude::*;
use predicates::str::contains;

mod common;
use common::MaskCommandExt;

#[test]
fn positional_arguments() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## test (file) (test_case)

> Run tests

~~~bash
echo "Testing $test_case in $file"
~~~
"#,
    );

    common::run_mask(maskfile_path)
        .command("test")
        .arg("the_file")
        .arg("some_test_case")
        .assert()
        .stdout(contains("Testing some_test_case in the_file"))
        .success();
}

#[test]
fn missing_positional_arguments() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## test (file) (test_case)

> Run tests

~~~bash
echo "Testing $test_case in $file"
~~~
"#,
    );

    common::run_mask(maskfile_path)
        .command("test")
        .arg("some_test_case")
        .assert()
        .stderr(contains("error: The following required arguments were not provided:
    <test_case>"))
        .failure();
}
