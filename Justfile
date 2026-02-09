# Show available recipes
help:
    just --list

# Format code
fmt:
    cargo fmt --all

# Check formatting and run clippy
lint:
    cargo fmt --all --check || (echo "Run 'just fmt' to fix formatting!" && exit 1)
    cargo clippy --all-targets --features "full nightly"

# Run tests
test:
    cargo test --features "full nightly"

# Test documentation
docs:
    cargo docs

# Ensure repository does not contain email addresses
check-no-email:
    if rg -n --hidden -g '!.git' -g '!target' -e "[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}" .; then echo "Email addresses found in repository files. Remove them before commit."; exit 1; else echo "No email addresses found."; fi

# Run all checks we do in CI (lint + test + docs)
ci:
    just check-no-email
    just lint
    just test
    just docs

# Same as 'just ci' but in clean environment
clean_ci:
    cargo clean
    just ci
