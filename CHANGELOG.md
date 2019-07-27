# Changelog


## UNRELEASED

### Fixed

* Prevent adding needless verbose flag to commands with no script [#21](https://github.com/jakedeichert/mask/pull/21)

* Propagate exit status of child process to main process [#22](https://github.com/jakedeichert/mask/pull/22) ([@atty303](https://github.com/atty303))

* Allow --version and --help to be used even when missing a maskfile [#23](https://github.com/jakedeichert/mask/pull/23)

* Exit with an error message and status code 1 when subcommand is missing [#23](https://github.com/jakedeichert/mask/pull/23)

* Always exit with error when custom maskfile is not found [#25](https://github.com/jakedeichert/mask/pull/25)





## v0.3.1 (2019-07-21)

### Added

* Allow specifying an external maskfile.md to use [#15](https://github.com/jakedeichert/mask/pull/19) ([@felipesere](https://github.com/felipesere))





## v0.3.0 (2019-07-19)

### Breaking Changes

* Changed required arg syntax from `<arg>` to `(arg)` to prevent markdown renderers from breaking [#16](https://github.com/jakedeichert/mask/pull/16)

### Fixed

* Using `<>` for required args causes breakage in certain markdown renderers [#15](https://github.com/jakedeichert/mask/issues/15)
* Using `inline code` in a command description doesn't get output with `-h` [#9](https://github.com/jakedeichert/mask/issues/9)





## v0.2.1 (2019-07-17)

### Added

* bash, zsh, and fish executors





## v0.2.0 (2019-07-16)

Initial release 🎉
