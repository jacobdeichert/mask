<p align="center">
  <img height="180" width="210" src="https://user-images.githubusercontent.com/1631044/103310063-b1d79980-49e4-11eb-86e4-242c3691b0b4.png">
</p>


[![build status](https://github.com/jacobdeichert/mask/actions/workflows/ci.yml/badge.svg?branch=master)][github_ci]
[![mask version](https://img.shields.io/crates/v/mask.svg)][crate]
[![mask crate downloads](https://img.shields.io/crates/d/mask.svg)][crate]

`mask` is a CLI task runner which is defined by a simple markdown file. It searches for a `maskfile.md` in the current directory which it then parses for commands and arguments.

A `maskfile.md` is both a **human-readable document** and a **command definition**! Being documentation focused allows others to easily get started with your project's development setup by simply reading your `maskfile.md`. A nice advantage of using markdown is that syntax highlighting for code blocks is built-in to many editors and renderers like GitHub itself.

Here's the [maskfile.md](/maskfile.md) that `mask` itself uses as an example!

To get started, follow the guide below or check out the more [advanced features](#features) `mask` has like **positional args**, **optional flags**, **subcommands**, other **scripting runtimes** and more!





## Installation

### Precompiled binaries

Head to the [Releases page][releases] and look for the latest published version. Under **Assets** you'll see zips available for download for linux, macOS and Windows. Once downloaded, you can unzip them and then move the `mask` binary to somewhere accessible in your `$PATH` like `mv mask /usr/local/bin`.

### Homebrew

`mask` is available in [Homebrew][homebrew] which allows you to install it via `brew install mask`.

### Cargo

`mask` is published to [crates.io][crate] which allows you to install it via `cargo install mask`.

### From source

If you prefer to build from source, clone this repo and then run `cargo build --release`





## Getting started

First, define a simple `maskfile.md` in your project.

````markdown
# Tasks For My Project


<!-- A heading defines the command's name -->
## build

<!-- A blockquote defines the command's description -->
> Builds my project

<!-- A code block defines the script to be executed -->
```sh
echo "building project..."
```


## test

> Tests my project

You can also write documentation anywhere you want. Only certain types of markdown patterns
are parsed to determine the command structure.

This code block below is defined as js which means it will be ran with node. Mask also
supports other scripting runtimes including python, ruby and php!

```js
console.log("running tests...")
```
````

Then, try running one of your commands!

```sh
mask build
mask test
```





## Features

### Positional arguments

These are defined beside the command name within `(round_brackets)`. They are required arguments that must be supplied for the command to run. The argument name is injected into the script's scope as an environment variable.

**Example:**

````markdown
## test (file) (test_case)

> Run tests

```bash
echo "Testing $test_case in $file"
```
````

Optional arguments are defined within `[square_brackets]`.

**Example:**

````markdown
## test [test_file]

> Run tests

```bash
if [[ -n "$test_file" ]]; then
    echo "Run tests in $test_file..."
else
    echo "Running all tests...."
fi
```
````

### Named flags

You can define a list of named flags for your commands. The flag name is injected into the script's scope as an environment variable.

**Example:**

````markdown
## serve

> Serve this directory

<!-- You must define OPTIONS right before your list of flags -->
**OPTIONS**
* port
    * flags: -p --port
    * type: string
    * desc: Which port to serve on

```sh
PORT=${port:-8080} # Set a fallback port if not supplied

if [[ "$verbose" == "true" ]]; then
    echo "Starting an http server on PORT: $PORT"
fi
python -m SimpleHTTPServer $PORT
```
````

You can also make your flag expect a numerical value by setting its `type` to `number`. This means `mask` will automatically validate it as a number for you. If it fails to validate, `mask` will exit with a helpful error message.

**Example:**

````markdown
## purchase (price)

> Calculate the total price of something.

**OPTIONS**
* tax
    * flags: -t --tax
    * type: number
    * desc: What's the tax?

```sh
TAX=${tax:-1} # Fallback to 1 if not supplied
echo "Total: $(($price * $TAX))"
```
````

If you add a `choices` list, `mask` will validate if the flag value is one of them.

**Example:**

````markdown
## color

**OPTIONS**
* color
    * flags: -c --color
    * type: string
    * choices: RED, BLUE, GREEN

```bash
COLOR=${color:-RED} # Fallback to RED if not supplied
echo "Color selected = '$COLOR'"
```
````

If you exclude the `type` field, `mask` will treat it as a `boolean` flag. If the flag is passed, its environment variable will be `"true"`, otherwise it will be unset/non-existent.

Important to note that `mask` auto injects a very common `boolean` flag called `verbose` into every single command even if it's not used, which saves a bit of typing for you. This means every command implicitly has a `-v` and `--verbose` flag already.

**Example:**

````markdown
## test

> Run the test suite

**OPTIONS**
* watch
    * flags: -w --watch
    * desc: Run tests on file change

```bash
[[ "$watch" == "true" ]] && echo "Starting in watch mode..."
[[ "$verbose" == "true" ]] && echo "Running with extra logs..."
```
````

Flags are optional by default. If you add `required` to your flag definition, `mask` will error if it isn't supplied by the user.

**Example:**

````markdown
## ping

**OPTIONS**
* domain
    * flags: -d --domain
    * type: string
    * desc: Which domain to ping
    * required

```sh
ping $domain
```
````

### Subcommands

Nested command structures can easily be created since they are simply defined by the level of markdown heading. H2 (`##`) is where you define your top-level commands. Every level after that is a subcommand.

**Example:**
````markdown
## services

> Commands related to starting and stopping services

### services start (service_name)

> Start a service.

```bash
echo "Starting service $service_name"
```

### services stop (service_name)

> Stop a service.

```bash
echo "Stopping service $service_name"
```
````

You may notice above that the `start` and `stop` commands are prefixed with their parent command `services`. Prefixing subcommands with their ancestor commands may help readability in some cases, however, it is completely optional. The example below is the same as above, but without prefixing.

**Example:**
````markdown
## services

> Commands related to starting and stopping services

### start (service_name)

> Start a service.

```bash
echo "Starting service $service_name"
```

### stop (service_name)

> Stop a service.

```bash
echo "Stopping service $service_name"
```
````

### Support for other scripting runtimes

On top of shell/bash scripts, `mask` also supports using node, python, ruby and php as scripting runtimes. This gives you the freedom to choose the right tool for the specific task at hand. For example, let's say you have a `serve` command and a `snapshot` command. You could choose python to `serve` a simple directory and maybe node to run a puppeteer script that generates a png `snapshot` of each page.

**Example:**

````markdown
## shell (name)

> An example shell script

Valid lang codes: sh, bash, zsh, fish... any shell that supports -c

```zsh
echo "Hello, $name!"
```

## node (name)

> An example node script

Valid lang codes: js, javascript

```js
const { name } = process.env;
console.log(`Hello, ${name}!`);
```

## python (name)

> An example python script

Valid lang codes: py, python

```python
import os
name = os.getenv("name", "WORLD")
print("Hello, " + name + "!")
```

## lua (name)

> An example lua script

```lua
name = os.getenv("name")
print("Hello, " .. name .. "!")
```

## ruby (name)

> An example ruby script

Valid lang codes: rb, ruby

```ruby
name = ENV["name"] || "WORLD"
puts "Hello, #{name}!"
```

## php (name)

> An example php script

```php
$name = getenv("name") ?: "WORLD";
echo "Hello, " . $name . "!\n";
```

## swift (name)

> An example swift script

```swift
import Foundation
let name = ProcessInfo.processInfo.environment["name"] ?? "WORLD"
print("Hello, \(name)!")
```
````

#### Windows support

You can even add powershell or batch code blocks alongside linux/macOS ones. Depending on which platform this runs on, the correct code block will be executed.

**Example:**

````markdown
## link

> Build and link the binary globally

```bash
cargo install --force --path .
```

```powershell
[Diagnostics.Process]::Start("cargo", "install --force --path .").WaitForExit()
```
````

### Automatic help and usage output

You don't have to spend time writing out help info manually. `mask` uses your command descriptions and options to automatically generate help output. For every command, it adds `-h, --help` flags and an alternative `help <name>` command.

**Example:**
```sh
mask services start -h
mask services start --help
mask services help start
mask help services start
```

All output the same help info:

```txt
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
```

### Running mask from within a script

You can easily call `mask` within scripts if you need to chain commands together. However, if you plan on [running mask with a different maskfile](#running-mask-with-a-different-maskfile), you should consider using the `$MASK` utility instead which allows your scripts to be location-agnostic.

**Example:**

````markdown
## bootstrap

> Installs deps, builds, links, migrates the db and then starts the app

```sh
mask install
mask build
mask link
# $MASK also works. It's an alias variable for `mask --maskfile <path_to_maskfile>`
# which guarantees your scripts will still work even if they are called from
# another directory.
$MASK db migrate
$MASK start
```
````

### Inherits the script's exit code

If your command exits with an error, `mask` will exit with its status code. This allows you to chain commands which will exit on the first error.

**Example:**

````markdown
## ci

> Runs tests and checks for lint and formatting errors

```sh
mask test \
    && mask lint \
    && mask format --check
```
````

### Running mask with a different maskfile

If you're in a directory that doesn't have a `maskfile.md` but you want to reference one somewhere else, you can with the `--maskfile <path_to_maskfile>` option.

**Example:**

```sh
mask --maskfile ~/maskfile.md <subcommand>
```

**Tip:** Make a bash alias for this so you can call it anywhere easily

```bash
# Call it something fun
alias wask="mask --maskfile ~/maskfile.md"

# You can run this from anywhere
wask <subcommand>
```

### Environment variable utilities

Inside of each script's execution environment, `mask` injects a few environment variable helpers that might come in handy.

**`$MASK`**

This is useful when [running mask within a script](#running-mask-from-within-a-script). This variable allows us to call `$MASK command` instead of `mask --maskfile <path> command` inside scripts so that they can be location-agnostic (not care where they are called from). This is especially handy for global maskfiles which you may call from anywhere.

**`$MASKFILE_DIR`**

This variable is an absolute path to the maskfile's parent directory. Having the parent directory available allows us to load files relative to the maskfile itself which can be useful when you have commands that depend on other external files.

### Documentation sections

If a heading doesn't have a code block, it will be treated as documentation and completely ignored.

**Example:**

```markdown
## This is a heading with no script

It's useful as a place to document things like a setup guide or required dependencies
or tools that your commands may rely on.
```





## Use cases

Here's some example scenarios where `mask` might be handy.

### Project specific tasks

You have a project with a bunch of random build and development scripts or an unwieldy `Makefile`. You want to simplify by having a single, readable file for your team members to add and modify existing tasks.


### Global system utility

You want a global utility CLI for a variety of system tasks such as backing up directories or renaming a bunch of files. This is easily possible by making a bash alias for `mask --maskfile ~/my-global-maskfile.md`.





## FAQ

### Is `mask` available as a lib?

The [mask-parser][mask_parser] crate is available. However, it's not yet documented nor considered stable.

### Where did the inspiration come from?

I'm definitely not the first to come up with this idea of using markdown as a CLI structure definition.

My frustrations with `make`'s syntax is what led me to search for other options. I landed on [just][just] for awhile which was a pretty nice improvement. My favourite feature of `just` is its support for other language runtimes, which is why `mask` also has this ability! However, it still didn't have some features I wanted like nested subcommands and multiple optional flags.

At some point in my searching, I came across [maid][maid] which is where most of the inspiration for `mask` comes from. I thought it was brilliant that markdown could be used as a command definition format while still being so readable.

So why did I choose to rebuild the wheel instead of using `maid`? For one, I preferred installing a single binary, like `just` is, rather than installing an npm package with hundreds of deps. I also had a few ideas on how I could improve upon `maid` which is why `mask` supports multiple levels of nested subcommands as well as optional flags and positional args. Also... I just really wanted to build another thing with Rust :)

I also need to mention [clap][clap] and [pulldown-cmark][cmark] which are really the core parts of `mask` that made it so easy to create.





## Contributing

Check out our [Contribution Guidelines](CONTRIBUTING.md) before creating an issue or submitting a PR 🙌

Also, please review and follow the rules within our [Code of Conduct](CODE_OF_CONDUCT.md) 🙂





## Author

Jacob Deichert with the help of contributors.





[github_ci]: https://github.com/jacobdeichert/mask/actions?query=workflow%3ACI
[mask_parser]: https://crates.io/crates/mask-parser
[homebrew]: https://formulae.brew.sh/formula/mask
[crate]: https://crates.io/crates/mask
[releases]: https://github.com/jacobdeichert/mask/releases
[2]: https://github.com/jacobdeichert/mask/issues/5
[maid]: https://github.com/egoist/maid
[just]: https://github.com/casey/just
[clap]: https://github.com/clap-rs/clap
[cmark]: https://github.com/raphlinus/pulldown-cmark
