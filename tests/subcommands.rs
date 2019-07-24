use assert_cmd::prelude::*;
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
