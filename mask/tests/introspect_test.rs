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
        "choices": [],
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
                "optional_args": [],
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

#[test]
fn includes_body_content_between_description_and_script() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Document Title

## greet (name)
> Say hello

This is some **extra** documentation.

It can span multiple lines.

```bash
echo "Hello $name"
```
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
        "choices": [],
    });

    let expected_json = json!({
        "title": "Document Title",
        "description": "",
        "commands": [
            {
                "level": 2,
                "name": "greet",
                "description": "Say hello",
                "body": "This is some **extra** documentation.\n\nIt can span multiple lines.",
                "script": {
                    "executor": "bash",
                    "source": "echo \"Hello $name\"\n",
                },
                "subcommands": [],
                "required_args": [
                    {
                        "name": "name"
                    }
                ],
                "optional_args": [],
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

#[test]
fn body_excludes_options_section() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Document Title

## deploy (env)
> Deploy the app

Some notes about deployment.

**OPTIONS**
* verbose
    * flags: -v --verbose
    * type: string
    * desc: Verbosity level

```bash
echo "Deploying to $env"
```
"#,
    );

    common::run_mask(&maskfile_path)
        .arg("--introspect")
        .assert()
        .code(0)
        .stdout(contains("\"body\": \"Some notes about deployment.\""))
        .success();
}
