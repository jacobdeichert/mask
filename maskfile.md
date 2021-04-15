# Tasks

Development tasks for mask.








## test
> Run all tests in the workspace

~~~bash
echo "Running tests..."
cargo test
echo "Tests passed!"
~~~

**Note:** On Windows platforms, `mask` falls back to running `powershell` code blocks.

~~~powershell
Write-Output "Running tests..."
cargo test
Write-Output "Tests passed!"
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

~~~powershell
param (
    $check = $env:check
)

if ($check) {
    cargo fmt --all -- --check
} else {
    cargo fmt
}
~~~










## lint
> Lint the project with clippy

~~~bash
cargo clippy
~~~

~~~powershell
cargo clippy
~~~
