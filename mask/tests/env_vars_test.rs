mod common;
use assert_cmd::prelude::*;
use common::MaskCommandExt;
use predicates::str::contains;

// NOTE: This test suite depends on the mask binary being available in the current shell

// Using current_dir("tests") to make sure the default maskfile.md can't be found
mod env_var_mask {
    use super::*;

    #[test]
    fn works_from_any_dir() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## ci

~~~bash
$MASK test
~~~

~~~powershell
$path = $env:MASK.replace("\\?\", "")
$pos = $path.IndexOf(" ");
$arglist = $path.Substring($pos + 1);

Start-Process mask.exe -ArgumentList "$arglist test" -wait -NoNewWindow -PassThru
~~~

## test

~~~bash
echo "tests passed"
~~~

~~~powershell
Write-Output "tests passed"
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .current_dir("tests")
            .command("ci")
            .assert()
            .stdout(contains("tests passed"))
            .success();
    }

    #[test]
    fn set_to_the_correct_value() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## run

~~~bash
echo "mask = $MASK"
~~~

~~~powershell
param (
    $var = "$env:mask /"
)

Write-Output "mask = $var"
~~~

"#,
        );

        #[cfg(windows)]
        let predicate = contains("mask = mask --maskfile \\");
        #[cfg(not(windows))]
        let predicate = contains("mask = mask --maskfile /");

        common::run_mask(&maskfile_path)
            .current_dir("tests")
            .command("run")
            .assert()
            // Absolute maskfile path starts with /
            .stdout(predicate)
            // And ends with maskfile.md
            .stdout(contains("maskfile.md"))
            .success();
    }
}

// Using current_dir("tests) to make sure the default maskfile.md can't be found
mod env_var_maskfile_dir {
    use super::*;

    #[test]
    fn set_to_the_correct_value() {
        let (_temp, maskfile_path) = common::maskfile(
            r#"
## run

~~~bash
echo "maskfile_dir = $MASKFILE_DIR"
~~~

~~~powershell
param (
    $var = $env:maskfile_dir
)

Write-Output "maskfile_dir = /$var"
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .current_dir("tests")
            .command("run")
            .assert()
            // Absolute maskfile path starts with /
            .stdout(contains("maskfile_dir = /"))
            .success();
    }
}
