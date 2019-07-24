use assert_cmd::prelude::*;
use predicates::str::contains;

mod common;
use common::MaskCommandExt;

#[test]
fn bash() {
    let (_temp, maskfile_path) = common::maskfile(
        "
# Integration tests

## bash

```bash
echo Hello, $name!
```

",
    );

    common::run_mask(&maskfile_path)
        .command("bash")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn node() {
    let (_temp, maskfile_path) = common::maskfile(
        "
# Integration tests

## node

```js
const { name } = process.env;
console.log(`Hello, ${name}!`);
```

",
    );

    common::run_mask(&maskfile_path)
        .command("node")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn python() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Integration tests

## python

```py
import os

name = os.getenv("name", "WORLD")

print("Hello, " + name + "!")
```

"#,
    );

    common::run_mask(&maskfile_path)
        .command("python")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn ruby() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Integration tests

## ruby

```ruby
name = ENV["name"] || "WORLD"

puts "Hello, #{name}!"
```

"#,
    );

    common::run_mask(&maskfile_path)
        .command("ruby")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn php() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Integration tests

## php

```php
$name = getenv("name") ?: "WORLD";

echo "Hello, " . $name . "!\n";
```

"#,
    );

    common::run_mask(&maskfile_path)
        .command("php")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}
