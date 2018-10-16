set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --features $FEATURES

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo test --target $TARGET --features $FEATURES
    fi
}

main
