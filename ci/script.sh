set -euxo pipefail

main() {
    cargo check --target $TARGET
}

main
