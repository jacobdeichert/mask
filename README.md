# mask 🎭

`mask` is a CLI task runner defined by a simple markdown file. `mask` searches for a `maskfile.md` in the current directory which it then parses for commands and arguments.

A `maskfile.md` is both a **human-readable document** and a **command definition**! Being documentation focused allows others to easily get started with your project's toolchain by simply reading your `maskfile.md` which can be rendered using a variety of markdown previewers or just GitHub itself.

Here's `mask`'s own [maskfile.md](/maskfile.md) as an example!

Follow the getting started guide below or check out the other [features](#features) `mask` has.



## Getting started

First, install `mask` with `cargo`. You'll need the [rust toolchain][1] installed.

~~~sh
cargo install mask
~~~

Next, define a simple `maskfile.md` in your project.

```md
# My Project CLI

## build

<!-- This is the commands description which is printed with -h/--help -->
> Builds my project

<!-- This is the script that will be executed -->
~~~sh
echo "building project..."
~~~

## test

> Tests my project

<!-- You can choose between a few scripting languages too! (node, python, ruby, php) -->
~~~js
console.log("running project's all tests")
~~~
```

And finally, try running one of your commands!

~~~sh
mask build
mask test
~~~




## Features

### Positional arguments

These are defined beside the command name within `<angle_brackets>`. They are required arguments that must be supplied for the command to run. [Optional args][2] are coming soon. The argument name is injected into the script's scope as an environment variable.

```md
## test <file> <test_case>

> Run tests

~~~bash
echo "Testing '$test_case' in '$file'"
~~~
```

### Optional flags

You can define a list of optional flags for your commands. The flag name is injected into the script's scope as an environment variable.

Important to note that `mask` auto injects a very common `boolean` flag called `verbose` into every single command even if it's not used. This saves a bit of typing for you! This means every command implictly has a `-v` and `--verbose` flag already. The value of the `$verbose` environment variable is either `"true"` or simply unset/non-existent.

```md
## serve

> Serve this directory

<!-- You must define OPTIONS right before your list of flags -->
**OPTIONS**
* port
    * flags: -p --port
    * type: string
    * desc: Which port to serve on

~~~sh
# Set a fallback port
PORT=${port:-8080}

if [[ "$verbose" == "true" ]]; then
    echo "Starting an http server on PORT: $PORT"
fi
python -m SimpleHTTPServer $PORT
~~~

```


[1]: https://github.com/rust-lang/rustup.rs
[2]: https://github.com/jakedeichert/mask/issues/5
