# Tasks

> Development tasks for mask





## run <maskfile_command>

> Build and run mask in development mode

**NOTE:** This uses `cargo run` to build and run `mask` in development mode. You must have a `maskfile` in the current directory (this file) and must supply a valid command for that `maskfile` (`maskfile_command`) in order to test the changes you've made to `mask`. Since you can only test against this `maskfile` (for now), you can add subcommands to the bottom and run against those instead of running one of the existing commands.

**EXAMPLE:** `mask run "test -h"` - outputs the help info of this `test` command

**OPTIONS**
* watch
    * flags: -w --watch
    * desc: Rebuild on file change

~~~sh
if [[ $watch == "true" ]]; then
    watchexec --exts rs --restart "cargo run -- $maskfile_command"
else
    cargo run -- $maskfile_command
fi
~~~



## build

> Build a release version of mask

~~~sh
cargo build --release
~~~



## test

> Run all tests

**OPTIONS**
* pattern
    * flags: -p --pattern
    * type: string
    * desc: Test only a specific file pattern

~~~sh
extra_args=""

if [[ "$verbose" == "true" ]]; then
    # Run tests linearly and make logs visible in output
    extra_args="-- --nocapture --test-threads=1"
fi

echo "Start tests..."
# Run all tests by default
if [[ "$pattern" == "" ]]; then
    cargo test $extra_args
else
    # Tests a specific integration filename pattern
    cargo test --test $pattern $extra_args
fi
~~~



## deps

> Commands related to cargo dependencies

### upgrade

> Update the cargo dependencies

~~~sh
cargo update
~~~



## format

> Format all source files

**OPTIONS**
* check
    * flags: -c --check
    * desc: Show which files are not formatted correctly

~~~sh
if [[ $check == "true" ]]; then
    cargo fmt --all -- --check
else
    cargo fmt
fi
~~~



## lint

> Lint the project with clippy

~~~sh
cargo clippy
~~~
