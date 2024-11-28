use std::process::Command;

fn main() {
    // Check if `go` binary is available
    if let Err(e) = Command::new("go").arg("version").output() {
        panic!("Go is not installed or not in the PATH: {}", e);
    }

    // Run `go get` to install the required Go package
    let status = Command::new("go")
        .args(["install", "github.com/fr12k/go-mask@latest"])
        .status()
        .expect("Failed to execute `go install`");

    if !status.success() {
        panic!("`go install` command failed. Please check your Go setup.");
    }

    println!("cargo:rerun-if-changed=build.rs"); // Re-run if this file changes
}
