set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --features unproven
    cargo fmt -- --check

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo test --target $TARGET --features unproven
    fi
}

main
