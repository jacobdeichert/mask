use assert_cmd::prelude::*;
use predicates::str::contains;
use colored::*;

mod common;
use common::MaskCommandExt;

#[test]
fn prepares_the_subshell_with_the_init_script() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
**ON::INIT**
~~~sh
set -a # Export everything so subprocesses have access
TEST_VAR=123
~~~

## run
~~~bash
echo "The test var is $TEST_VAR"
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .cli("run")
        .assert()
        .stdout(contains("The test var is 123"))
        .success();
}

#[test]
fn exits_with_error_status_when_init_script_fails() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
**ON::INIT**
~~~sh
exit 1
~~~

## run
~~~bash
echo "This shouldn't echo"
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .cli("run")
        .assert()
        .code(1)
        .failure();
}

#[test]
fn exits_with_error_when_not_a_shell_based_executor() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
**ON::INIT**
~~~js
console.log("nope");
~~~

## run
~~~bash
echo "This shouldn't echo"
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .cli("run")
        .assert()
        .code(1)
        .stderr(contains(format!(
            "{} ON::INIT must be a shell-based script executor.",
            "ERROR:".red()
        )))
        .failure();
}

