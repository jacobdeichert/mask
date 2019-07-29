use assert_cmd::prelude::*;
use predicates::str::{contains, is_empty};

mod common;

use common::MaskCommandExt;

#[test]
fn positional_arguments() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"

## services

> Commands related to starting, stopping, and restarting services

### services start (service_name)

> Start a service.

~~~bash
echo "Starting service $service_name"
~~~

### services stop (service_name)

> Stop a service.

~~~bash
echo "Stopping service $service_name"
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .cli("services start my_fancy_service")
        .assert()
        .stdout(contains("Starting service my_fancy_service"))
        .success();
}

#[test]
fn exits_with_error_when_missing_subcommand() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## foo
"#,
    );

    common::run_mask(&maskfile_path)
        .assert()
        .stderr(contains(
            "error: 'mask' requires a subcommand, but one was not provided",
        ))
        .failure();
}

mod when_command_has_no_source {
    use super::*;

    #[test]
    fn exits_gracefully_when_it_has_no_subcommands() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## system
"#,
        );

        // NOTE: Right now we exit without an error. Perhaps there should at least
        // be a warning logged to the console?
        common::run_mask(&maskfile_path)
            .command("system")
            .assert()
            .stdout(is_empty())
            .success();
    }

    #[test]
    fn exits_with_error_when_it_has_subcommands() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## system

### start

~~~sh
echo "system, online"
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .command("system")
            .assert()
            .stderr(contains(
                "error: 'mask system' requires a subcommand, but one was not provided",
            ))
            .failure();
    }
}
