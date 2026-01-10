# Changelog


## UNRELEASED

* ci: Fix tests under windows-latest CI runner [#129](https://github.com/jacobdeichert/mask/pull/129)

* Use triple backticks for markdown snippets in `README.md` [#128](https://github.com/jacobdeichert/mask/pull/128) ([@danielhenrymantilla](https://github.com/danielhenrymantilla))

* Bump actions test runtime to macOS 26 [#135](https://github.com/jacobdeichert/mask/pull/135)

* Remove unnecessary ruby action [#136](https://github.com/jacobdeichert/mask/pull/136)


## v0.11.6 (2024-10-13)

* ci: Fix apple x86_64 release process



## v0.11.5 (2024-10-13)

* ci: Fix php tests on macos actions runner [#110](https://github.com/jacobdeichert/mask/pull/110)

* Add choices list for flags [#111](https://github.com/jacobdeichert/mask/pull/111) ([@lovejia2022](https://github.com/lovejia2022))

* Optional arguments [#109](https://github.com/jacobdeichert/mask/pull/109) ([@jpal91](https://github.com/jpal91))

* Bump dependencies [#116](https://github.com/jacobdeichert/mask/pull/116)

## v0.11.4 (2023-10-15)

### Added

* Improve error message for invalid executors [#99](https://github.com/jacobdeichert/mask/pull/99) ([@brumhard](https://github.com/brumhard))

* Publish sha256 checksums with each release [#103](https://github.com/jacobdeichert/mask/pull/103) ([@simonsan](https://github.com/simonsan))

* ci: Automate publishing of crates

* ci: Add workflow for creating new releases



## v0.11.3 (2023-01-08)

### Added

* Add musl linux binary to releases [#95](https://github.com/jacobdeichert/mask/pull/95) ([@8Mobius8](https://github.com/8Mobius8))



## v0.11.2 (2022-05-25)

### Added

* Add Windows binary to releases [#89](https://github.com/jacobdeichert/mask/pull/89)



## v0.11.1 (2022-02-14)

### Added

* Add Apple Silicon (M1) binary to releases [#88](https://github.com/jacobdeichert/mask/pull/88)



## v0.11.0 (2021-04-15)

### Added

* Split parsing logic into separate crate (`mask-parser`) [#83](https://github.com/jacobdeichert/mask/pull/83)

* Add `--maskfile-introspect` flag to print out the maskfile command structure as json [#85](https://github.com/jacobdeichert/mask/pull/85)

### Fixed

* Rename option_flags to named_flags [#84](https://github.com/jacobdeichert/mask/pull/84)

* Rename `--maskfile-introspect` flag to `--introspect` [#86](https://github.com/jacobdeichert/mask/pull/86)



## v0.10.0 (2020-12-29)

### Added

* Allow marking named flags (previously called optional flags) as required [#74](https://github.com/jacobdeichert/mask/pull/74) ([@cherusk](https://github.com/cherusk))

* Automatically bump Homebrew formula on new release [#74](https://github.com/jacobdeichert/mask/pull/64) ([@vladimyr](https://github.com/vladimyr))

### Fixed

* Fix issue where command before level 1 heading was ignored [#77](https://github.com/jacobdeichert/mask/pull/77)




## v0.9.0 (2020-08-10)

### Added

* Allow absolute naming for headings [#71](https://github.com/jacobdeichert/mask/pull/71) ([@lsampras](https://github.com/lsampras))

### Fixed

* Remove commands without code blocks [#70](https://github.com/jacobdeichert/mask/pull/70) ([@twitu](https://github.com/twitu))




## v0.8.0 (2020-04-26)

### Added

* Windows powershell and batch support [#58](https://github.com/jacobdeichert/mask/pull/58) ([@tensor-programming](https://github.com/tensor-programming))

### Fixed

* Re-add formatting CI [#60](https://github.com/jacobdeichert/mask/pull/60)




## v0.7.1 (2019-12-13)

### Fixed

* Fix typo in README [#39](https://github.com/jacobdeichert/mask/pull/39) ([@nicoder](https://github.com/nicoder))

### Added

* Automated GitHub Releases and attached precompiled binaries for macOS and linux





## v0.7.0 (2019-10-13)

### Breaking Changes

* Allow any shell executor that supports -c evaluation (sh, bash, zsh, fish, dash, etc...) [#37](https://github.com/jacobdeichert/mask/pull/37)
* Error when chosen command doesn't have a script [#37](https://github.com/jacobdeichert/mask/pull/37)
* Error when chosen command script doesn't have a lang code to determine the executor [#37](https://github.com/jacobdeichert/mask/pull/37)
* Remove the `ON::INIT` script idea [#38](https://github.com/jacobdeichert/mask/pull/38)





## v0.6.0 (2019-10-06)

### Breaking Changes

* Add support for an `ON::INIT` script which initializes subshell environments [#36](https://github.com/jacobdeichert/mask/pull/36)





## v0.5.2 (2019-09-26)

### Added

* Add support for type=number in option flags for numerical validation [#35](https://github.com/jacobdeichert/mask/pull/35)

### Fixed

* Allow entering negative numbers as arg values [#34](https://github.com/jacobdeichert/mask/pull/34)





## v0.5.1 (2019-09-24)

### Added

* Colored help output and text wrapping [#30](https://github.com/jacobdeichert/mask/pull/30) ([@DrSensor](https://github.com/DrSensor))

### Fixed

* No need to show mask's author and description in help output [#32](https://github.com/jacobdeichert/mask/pull/32)





## v0.5.0 (2019-07-28)

### Added

* Add `$MASK` and `$MASKFILE_DIR` utility env variables [#26](https://github.com/jacobdeichert/mask/pull/26)

### Fixed

* Error when command has no script and missing subcommand [#27](https://github.com/jacobdeichert/mask/pull/27)
* Remove needless version flag from all subcommands [#27](https://github.com/jacobdeichert/mask/pull/27)





## v0.4.0 (2019-07-26)

### Fixed

* Prevent adding needless verbose flag to commands with no script [#21](https://github.com/jacobdeichert/mask/pull/21)

* Propagate exit status of child process to main process [#22](https://github.com/jacobdeichert/mask/pull/22) ([@atty303](https://github.com/atty303))

* Allow --version and --help to be used even when missing a maskfile [#23](https://github.com/jacobdeichert/mask/pull/23)

* Exit with an error message and status code 1 when subcommand is missing [#23](https://github.com/jacobdeichert/mask/pull/23)

* Always exit with error when custom maskfile is not found [#25](https://github.com/jacobdeichert/mask/pull/25)





## v0.3.1 (2019-07-21)

### Added

* Allow specifying an external maskfile.md to use [#15](https://github.com/jacobdeichert/mask/pull/19) ([@felipesere](https://github.com/felipesere))





## v0.3.0 (2019-07-19)

### Breaking Changes

* Changed required arg syntax from `<arg>` to `(arg)` to prevent markdown renderers from breaking [#16](https://github.com/jacobdeichert/mask/pull/16)

### Fixed

* Using `<>` for required args causes breakage in certain markdown renderers [#15](https://github.com/jacobdeichert/mask/issues/15)
* Using `inline code` in a command description doesn't get output with `-h` [#9](https://github.com/jacobdeichert/mask/issues/9)





## v0.2.1 (2019-07-17)

### Added

* bash, zsh, and fish executors





## v0.2.0 (2019-07-16)

Initial release 🎉
