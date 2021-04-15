# Tasks

Development tasks for mask.







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
