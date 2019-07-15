# mask 🎭

`mask` is a CLI task runner which is defined by a simple markdown file. It searches for a `maskfile.md` in the current directory which it then parses for commands and arguments.

A `maskfile.md` is both a **human-readable document** and a **command definition**! Being documentation focused allows others to easily get started with your project's development setup by simply reading your `maskfile.md`. A nice advantage of using markdown is that syntax highlighting for code blocks is built-in to many editors and renderers like GitHub itself.

Here's `mask`'s own [maskfile.md](/maskfile.md) as an example!

To get started, follow the guide below or check out the more [advanced features](#features) `mask` has.





## Getting started

First, install `mask` with `cargo`. You'll need the [rust toolchain][1] installed if you don't have it already.

~~~sh
cargo install mask
~~~

Next, define a simple `maskfile.md` in your project.

```md
# My Project CLI


<!-- A heading defines the command's name -->
## build

<!-- A blockquote defines the command's description -->
> Builds my project

<!-- A code block defines the script to be executed -->
~~~sh
echo "building project..."
~~~


## test

> Tests my project

You can also write documentation anywhere you want. Only certain types of markdown patterns
are parsed to determine the command structure.

Note this code block below is defined as js. So far, mask supports node,
python, ruby and php as scripting languages!

~~~js
console.log("running project's tests")
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

**Example:**

```md
## test <file> <test_case>

> Run tests

~~~bash
echo "Testing $test_case in $file"
~~~
```

### Optional flags

You can define a list of optional flags for your commands. The flag name is injected into the script's scope as an environment variable.

Important to note that `mask` auto injects a very common `boolean` flag called `verbose` into every single command even if it's not used. This saves a bit of typing for you! This means every command implictly has a `-v` and `--verbose` flag already. The value of the `$verbose` environment variable is either `"true"` or simply unset/non-existent.

**Example:**

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

### Subcommands

Nested command structures can easily be created since they are simply defined by the level of markdown heading. H2 (`##`) is where you define your top-level commands. Every level after that is a subcommand. The only requirement is that subcommands must have all ancestor commands present in their heading.

**Example:**
```md
## services

> Commands related to starting, stopping, and restarting services

### services start <service_name>

> Start a service.

~~~bash
echo "Starting service $service_name"
~~~

### services stop <service_name>

> Stop a service.

~~~bash
echo "Stopping service $service_name"
~~~

#### services stop all

> Stop everything.

~~~bash
echo "Stopping everything"
~~~
```

### Automatic help and usage output

You don't have to spend time writing out help info manually. `mask` uses your command descriptions and options to automatically generate help output. It adds `-h --help` flags and a `help <name>` command to all commands (thanks to [clap][3]).

**Example:**
~~~sh
mask services start -h
mask services start --help
mask services help start
mask help services start
~~~

All output the same help info:

~~~txt
mask-services-start
Start or restart a service.

USAGE:
    mask services start [FLAGS] <service_name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity
    -r, --restart    Restart this service if it's already running
    -w, --watch      Restart a service on file change

ARGS:
    <service_name>
~~~

### Support for other scripting languages

On top of shell/bash scripts, `mask` also supports using node, python, ruby and php as scripting languages. This gives you the freedom to choose the right tool for the specific task at hand.

For example, let's say you have a `serve` command and a `snapshot` command. You could choose python to `serve` a simple directory and maybe node to run a puppeteer script that generates a png `snapshot` of each page.

**Example:**

```md
## node <name>

> An example node script

"js" or "javascript" can be used as the lang code

~~~js
const { name } = process.env;
console.log(`Hello, ${name}!`)
~~~


## python <name>

> An example python script

"py" or "python" can be used as the lang code

~~~python
import os

name = os.getenv("name", "WORLD")

print("Hello, " + name + "!")
~~~


## ruby <name>

> An example ruby script

"rb" or "ruby" can be used as the lang code

~~~ruby
name = ENV["name"] || "WORLD"

puts "Hello, #{name}!"
~~~


## php <name>

> An example php script

~~~php
$name = getenv("name") ?: "WORLD";

echo "Hello, " . $name . "!\n";
~~~
```





[1]: https://github.com/rust-lang/rustup.rs
[2]: https://github.com/jakedeichert/mask/issues/5
[3]: https://github.com/clap-rs/clap
