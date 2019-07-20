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

    common::run_mask(&maskfile_path)
        .command("test")
        .arg("the_file")
        .arg("some_test_case")
        .assert()
        .stdout(contains("Testing some_test_case in the_file"))
        .success();

    common::run_mask(&maskfile_path)
        .command("test")
        .arg("some_test_case")
        .assert()
        .stderr(contains(
            "error: The following required arguments were not provided:
    <test_case>",
        ))
        .failure();
}

#[test]
fn optional_flags() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## serve

> Serve this directory

<!-- You must define OPTIONS right before your list of flags -->
**OPTIONS**
* port
    * flags: -p --port
    * type: string
    * desc: Which port to serve on

~~~sh
# Set a fallback port
PORT=${port:-8080}

if [[ "$verbose" == "true" ]]; then
    echo "Starting an http server on PORT: $PORT"
else
    echo $PORT
fi
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("serve")
        .arg("--port")
        .arg("1234")
        .assert()
        .stdout(contains("1234"))
        .success();

    // verbose is always available
    common::run_mask(&maskfile_path)
        .command("serve")
        .arg("--port")
        .arg("1234")
        .arg("--verbose")
        .assert()
        .stdout(contains("Starting an http server on PORT: 1234"))
        .success();
}
