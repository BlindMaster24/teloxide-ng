set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Show available recipes
help:
    just --list

# Format code
fmt:
    cargo fmt --all

# Check formatting and run clippy
lint:
    cargo fmt --all --check
    cargo clippy --all-targets --features "full nightly"

# Run tests
test:
    cargo test --features "full nightly"

# Test documentation
docs:
    cargo docs

# Ensure repository does not contain email addresses
check-no-email:
    python -c "import subprocess,sys; cmd=['rg','-n','--hidden','-g','!.git','-g','!target','-e',r'[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}','.']; r=subprocess.run(cmd); sys.exit(1 if r.returncode==0 else 0 if r.returncode==1 else r.returncode)"

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
