mod common;
use assert_cmd::prelude::*;
use predicates::str::contains;
use serde_json::json;

#[test]
fn outputs_the_maskfile_structure_as_json() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Document Title

## somecommand
> The command description

~~~bash
echo something
~~~
"#,
    );

    let verbose_flag = json!({
        "name": "verbose",
        "description": "Sets the level of verbosity",
        "short": "v",
        "long": "verbose",
        "multiple": false,
        "takes_value": false,
        "required": false,
        "validate_as_number": false,
    });

    let expected_json = json!({
        "title": "Document Title",
        "description": "",
        "commands": [
            {
                "level": 2,
                "name": "somecommand",
                "description": "The command description",
                "script": {
                    "executor": "bash",
                    "source": "echo something\n",
                },
                "subcommands": [],
                "required_args": [],
                "named_flags": [verbose_flag],
            }
        ]
    });

    common::run_mask(&maskfile_path)
        .arg("--introspect")
        .assert()
        .code(0)
        .stdout(contains(
            serde_json::to_string_pretty(&expected_json).unwrap(),
        ))
        .success();
}
