use assert_cmd::prelude::*;
use clap::{crate_name, crate_version};
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

```bash
# Set a fallback port
PORT=${port:-8080}

if [[ "$verbose" == "true" ]]; then
    echo "Starting an http server on PORT: $PORT"
else
    echo $PORT
fi
```
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

mod version_flag {
    use super::*;

    #[test]
    fn shows_the_correct_version_for_the_root_command() {
        let (_temp, maskfile_path) = common::maskfile("## foo");

        common::run_mask(&maskfile_path)
            .command("--version")
            .assert()
            .stdout(contains(format!("{} {}", crate_name!(), crate_version!())))
            .success();
    }

    #[test]
    fn exits_with_error_when_subcommand_has_version_flag() {
        let (_temp, maskfile_path) = common::maskfile("## foo");

        // The setting "VersionlessSubcommands" removes the version flags (-V, --version)
        // from subcommands. Only the root command has a version flag.

        common::run_mask(&maskfile_path)
            .command("foo")
            .arg("--version")
            .assert()
            .stderr(contains(
                "error: Found argument '--version' which wasn't expected, or isn't valid in this context",
            ))
            .failure();
    }
}
