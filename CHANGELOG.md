# Changelog


## UNRELEASED




## v0.10.0 (2020-12-29)

### Added

* Allow marking named flags (previously called optional flags) as required [#74](https://github.com/jakedeichert/mask/pull/74) ([@cherusk](https://github.com/cherusk))

* Automatically bump Homebrew formula on new release [#74](https://github.com/jakedeichert/mask/pull/64) ([@vladimyr](https://github.com/vladimyr))

### Fixed

* Fix issue where command before level 1 heading was ignored [#77](https://github.com/jakedeichert/mask/pull/77)




## v0.9.0 (2020-08-10)

### Added

* Allow absolute naming for headings [#71](https://github.com/jakedeichert/mask/pull/71) ([@lsampras](https://github.com/lsampras))

### Fixed

* Remove commands without code blocks [#70](https://github.com/jakedeichert/mask/pull/70) ([@twitu](https://github.com/twitu))




## v0.8.0 (2020-04-26)

### Added

* Windows powershell and batch support [#58](https://github.com/jakedeichert/mask/pull/58) ([@tensor-programming](https://github.com/tensor-programming))

### Fixed

* Re-add formatting CI [#60](https://github.com/jakedeichert/mask/pull/60)




## v0.7.1 (2019-12-13)

### Fixed

* Fix typo in README [#39](https://github.com/jakedeichert/mask/pull/39) ([@nicoder](https://github.com/nicoder))

### Added

* Automated GitHub Releases and attached precompiled binaries for macOS and linux





## v0.7.0 (2019-10-13)

### Breaking Changes

* Allow any shell executor that supports -c evaluation (sh, bash, zsh, fish, dash, etc...) [#37](https://github.com/jakedeichert/mask/pull/37)
* Error when chosen command doesn't have a script [#37](https://github.com/jakedeichert/mask/pull/37)
* Error when chosen command script doesn't have a lang code to determine the executor [#37](https://github.com/jakedeichert/mask/pull/37)
* Remove the `ON::INIT` script idea [#38](https://github.com/jakedeichert/mask/pull/38)





## v0.6.0 (2019-10-06)

### Breaking Changes

* Add support for an `ON::INIT` script which initializes subshell environments [#36](https://github.com/jakedeichert/mask/pull/36)





## v0.5.2 (2019-09-26)

### Added

* Add support for type=number in option flags for numerical validation [#35](https://github.com/jakedeichert/mask/pull/35)

### Fixed

* Allow entering negative numbers as arg values [#34](https://github.com/jakedeichert/mask/pull/34)





## v0.5.1 (2019-09-24)

### Added

* Colored help output and text wrapping [#30](https://github.com/jakedeichert/mask/pull/30) ([@DrSensor](https://github.com/DrSensor))

### Fixed

* No need to show mask's author and description in help output [#32](https://github.com/jakedeichert/mask/pull/32)





## v0.5.0 (2019-07-28)

### Added

* Add `$MASK` and `$MASKFILE_DIR` utility env variables [#26](https://github.com/jakedeichert/mask/pull/26)

### Fixed

* Error when command has no script and missing subcommand [#27](https://github.com/jakedeichert/mask/pull/27)
* Remove needless version flag from all subcommands [#27](https://github.com/jakedeichert/mask/pull/27)





## v0.4.0 (2019-07-26)

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
