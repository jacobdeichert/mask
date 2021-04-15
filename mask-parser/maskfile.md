# Tasks







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
