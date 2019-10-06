# Tasks

> Development tasks for mask





## run (maskfile_command)

> Build and run mask in development mode

**NOTE:** This uses `cargo run` to build and run `mask` in development mode. You must have a `maskfile` in the current directory (this file) and must supply a valid command for that `maskfile` (`maskfile_command`) in order to test the changes you've made to `mask`. Since you can only test against this `maskfile` for now, you can add subcommands to the bottom and run against those instead of running one of the existing commands.

**EXAMPLE:** `mask run "test -h"` - outputs the help info of this `test` command

**OPTIONS**
* watch
    * flags: -w --watch
    * desc: Rebuild on file change

~~~bash
if [[ $watch == "true" ]]; then
    watchexec --exts rs --restart "cargo run -- $maskfile_command"
else
    cargo run -- $maskfile_command
fi
~~~



## build

> Build a release version of mask

~~~bash
cargo build --release
~~~



## link

> Build mask and replace your globally installed version with it for testing

~~~bash
cargo install --force --path .
~~~



## test

> Run all tests

**OPTIONS**
* file
    * flags: -f --file
    * type: string
    * desc: Only run tests from a specific filename

~~~bash
extra_args=""

if [[ "$verbose" == "true" ]]; then
    # Run tests linearly and make logs visible in output
    extra_args="-- --nocapture --test-threads=1"
fi

log_info "Running tests..."
if [[ -z "$file" ]]; then
    # Run all tests by default
    cargo test $extra_args
else
    # Tests a specific integration filename
    cargo test --test $file $extra_args
fi
log_success "Tests passed!"
~~~



## deps

> Commands related to cargo dependencies

### deps upgrade

> Update the cargo dependencies

~~~bash
cargo update
~~~



## format

> Format all source files

**OPTIONS**
* check
    * flags: -c --check
    * desc: Show which files are not formatted correctly

~~~bash
if [[ $check == "true" ]]; then
    cargo fmt --all -- --check
else
    cargo fmt
fi
~~~



## lint

> Lint the project with clippy

~~~bash
cargo clippy
~~~







**ON::INIT**

This special script sets up the subshell environment before a command is executed. This is useful for global utilities and helpers.

~~~bash
set -a # Export everything so subprocesses have access
color_reset=$(tput sgr0)
color_blue=$(tput setaf 4)
color_green=$(tput setaf 2)
color_yellow=$(tput setaf 3)
color_red=$(tput setaf 1)
log_info() { echo "$color_blue$1$color_reset"; }
log_success() { echo "$color_green$1$color_reset"; }
log_error() { echo "$color_red$1$color_reset"; }
log_warn() { echo "$color_yellow$1$color_reset"; }
set +a
set -e # Exit on error
# Export this so bash subshells inherit "set -e"
export SHELLOPTS
~~~
