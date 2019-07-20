use assert_cmd::prelude::*;
use predicates::str;

mod common;
use common::MaskCommandExt;

#[test]
fn node() {
    let (_temp, maskfile_path) = common::maskfile(
        "
# Integartion tests

## node

```js
const { name } = process.env;
console.log(`Hello, ${name}!`);
```

",
    );

    common::run_mask(maskfile_path)
        .command("node")
        .env("name", "World")
        .assert()
        .stdout(str::contains("Hello, World!"))
        .success();
}

#[test]
fn python() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Integartion tests

## python

```py
import os

name = os.getenv("name", "WORLD")

print("Hello, " + name + "!")
```

"#,
    );

    common::run_mask(maskfile_path)
        .command("python")
        .env("name", "World")
        .assert()
        .stdout(str::contains("Hello, World!"))
        .success();
}

#[test]
fn ruby() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Integartion tests

## ruby

```ruby
name = ENV["name"] || "WORLD"

puts "Hello, #{name}!"
```

"#,
    );

    common::run_mask(maskfile_path)
        .command("ruby")
        .env("name", "World")
        .assert()
        .stdout(str::contains("Hello, World!"))
        .success();
}

#[test]
fn php() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
# Integartion tests

## php

```php
$name = getenv("name") ?: "WORLD";

echo "Hello, " . $name . "!\n";
```

"#,
    );

    common::run_mask(maskfile_path)
        .command("php")
        .env("name", "World")
        .assert()
        .stdout(str::contains("Hello, World!"))
        .success();
}
