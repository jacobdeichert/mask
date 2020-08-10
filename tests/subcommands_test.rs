use assert_cmd::prelude::*;
use colored::*;
use predicates::str::contains;

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

~~~powershell
param(
    $service_name = $env:service_name
)

Write-Output "Starting service $service_name"
~~~

### services stop (service_name)

> Stop a service.

~~~bash
echo "Stopping service $service_name"
~~~

~~~powershell
param(
    $service_name = $service_name
)

Write-Output "Stopping service $service_name"
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
## service
### service start

~~~bash
echo "subcommand should exist"
~~~
"#,
    );

    #[cfg(not(windows))]
    let predicate =
        contains("error: 'mask service' requires a subcommand, but one was not provided");
    #[cfg(windows)]
    let predicate =
        contains("error: 'mask.exe service' requires a subcommand, but one was not provided");
    common::run_mask(&maskfile_path)
        .command("service")
        .assert()
        .code(1)
        .stderr(predicate)
        .failure();
}

mod when_command_has_no_source {
    use super::*;

    #[test]
    fn exits_with_error_when_it_has_no_script_lang_code() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## start
~~~
echo "system, online"
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .command("start")
            .assert()
            .code(1)
            .stderr(contains(format!(
                "{} Command script requires a lang code which determines which executor to use.",
                "ERROR:".red()
            )))
            .failure();
    }
}
