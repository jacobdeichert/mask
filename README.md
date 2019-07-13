# mask 🎭

`mask` is a CLI task runner defined by a simple markdown file. `mask` searches for a `maskfile.md` in the current directory which it then parses for commands and arguments.

A `maskfile.md` is both a **human-readable document** and a **command definition**! Being documentation focused allows others to easily get started with your project's toolchain by simply reading your `maskfile.md` which can be rendered using a variety of markdown previewers or just GitHub itself.

Here's `mask`'s own [maskfile.md](/maskfile.md) as an example!



## Getting started

First, install `mask` with `cargo`. You'll need the [rust toolchain][1] installed.

~~~sh
cargo install mask
~~~

Next, define a simple `maskfile.md` in your project.

```md
# My Project CLI


## build

> Builds my project

~~~sh
echo "building project..."
~~~


## test

> Tests my project

~~~sh
echo "testing project..."
~~~
```




[1]: https://github.com/rust-lang/rustup.rs
