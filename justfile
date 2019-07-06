@run:
	cargo run


# Rebuild on file change
@watch:
	watchexec --exts rs --restart "just run"


# Update the cargo dependencies
@upgrade-deps:
	cargo update


# Run the test suites
test which_files="" is_verbose="":
	#!/usr/bin/env sh
	echo "----------------------------------------------------------"
	echo "🔬 RUNNING TESTS"
	echo "----------------------------------------------------------"
	extra_args=""

	if [[ "{{is_verbose}}" == "-v" ]]; then
	 	# Run tests linearly and make logs visible in output
		extra_args="-- --nocapture --test-threads=1"
	fi

	echo "Start tests..."
	if [[ "{{which_files}}" == "-a" ]]; then
		cargo test $extra_args
	else
		# Tests a specific integration filename
		cargo test --test {{which_files}} $extra_args
	fi

###########################################################################################
# FORMATTING
###########################################################################################

# Format the project
@format:
	cargo fmt


# Show which files need to be formatted
@format-check:
	cargo fmt --all -- --check


# Lint the project
@lint:
	cargo clippy
