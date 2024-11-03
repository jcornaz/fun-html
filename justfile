set dotenv-load

# Perform all verifications (compile, test, lint, etc.)
verify: test lint doc check-msrv
	cargo semver-checks
	cargo deny check licenses

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch --delay 0.1 --clear --why -- just verify

# Run the tests
test:
	cargo hack test --tests --feature-powerset --locked
	cargo test --examples --all-features --locked
	cargo test --doc --all-features --locked

# Run the static code analysis
lint:
	cargo fmt -- --check
	cargo hack clippy --each-feature --all-targets --locked

# Build the documentation
doc *args:
	cargo doc --all-features --no-deps {{args}}

# Open the documentation page
doc-open: (doc "--open")

# Make sure the MSRV is satisfiable
check-msrv:
	cargo msrv verify

release *args: verify
    test $GITHUB_TOKEN
    test $CARGO_REGISTRY_TOKEN
    cargo release {{args}}
