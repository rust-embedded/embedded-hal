set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --features unproven

    if [ "$TARGET" = "x86_64-unknown-linux-gnu" ]; then
        cargo test --target $TARGET --features unproven
    fi

    cargo fmt -- --check
}

main