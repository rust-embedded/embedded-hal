set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo fmt -- --check

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo test --target $TARGET
    fi
}

main
