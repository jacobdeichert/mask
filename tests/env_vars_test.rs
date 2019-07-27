use assert_cmd::prelude::*;
use predicates::str::contains;

mod common;
use common::MaskCommandExt;

// Using current_dir(".github") to make sure the default maskfile.md can't be found
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

## test

~~~bash
echo "tests passed"
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .current_dir(".github")
            .command("ci")
            .assert()
            .stdout(contains("tests passed"))
            .success();
    }

    #[test]
    fn set_to_the_correct_value() {
        let (temp, maskfile_path) = common::maskfile(
            r#"
## run

~~~bash
echo "mask = $MASK"
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .current_dir(".github")
            .command("run")
            .assert()
            // Needs "/private" because the temp dir is "/var" which is a symlink to "/private/var".
            .stdout(contains(format!(
                "mask = mask --maskfile /private{}/maskfile.md",
                temp.path().to_str().unwrap().to_string()
            )))
            .success();
    }
}

// Using current_dir(".github") to make sure the default maskfile.md can't be found
mod env_var_maskfile_dir {
    use super::*;

    #[test]
    fn set_to_the_correct_value() {
        let (temp, maskfile_path) = common::maskfile(
            r#"
## run

~~~bash
echo "maskfile_dir = $MASKFILE_DIR"
~~~
"#,
        );

        common::run_mask(&maskfile_path)
            .current_dir(".github")
            .command("run")
            .assert()
            // Needs "/private" because the temp dir is "/var" which is a symlink to "/private/var".
            .stdout(contains(format!(
                "maskfile_dir = /private{}",
                temp.path().to_str().unwrap().to_string()
            )))
            .success();
    }
}
