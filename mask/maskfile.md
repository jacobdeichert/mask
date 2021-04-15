# Tasks

Development tasks for mask.









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

~~~powershell
param (
    $maskfile_command = $env:maskfile_command,
    $watch = $env:watch
)

$cargo_cmd = "cargo run -- $maskfile_command"
$extra_args = "--exts rs --restart $cargo_cmd"

if ($watch) {
    Start-Process watchexec -ArgumentList $extra_args -NoNewWindow -PassThru
} else {
    cargo run -- $maskfile_command
}
~~~













## build
> Build a release version of mask

~~~bash
cargo build --release
~~~

~~~powershell
cargo build --release
~~~









## link
> Build mask and replace your globally installed version with it for testing

~~~bash
cargo install --force --path .
~~~

~~~powershell
[Diagnostics.Process]::Start("cargo", "install --force --path .").WaitForExit()
~~~










## test
> Run tests

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

echo "Running tests..."
if [[ -z "$file" ]]; then
    # Run all tests by default
    cargo test $extra_args
else
    # Tests a specific integration filename
    cargo test --test $file $extra_args
fi
echo "Tests passed!"
~~~

~~~powershell
param (
    $file = $env:file
)

$extra_args = ""
$verbose = $env:verbose

if ($verbose) {
    $extra_args = "-- --nocapture --test-threads=1"
}

Write-Output "Running tests..."
if (!$file) {
    cargo test $extra_args
} else {
    cargo test --test $file $extra_args
}
Write-Output "Tests passed!"
~~~
