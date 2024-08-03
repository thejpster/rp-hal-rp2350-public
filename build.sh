#!/bin/sh
# curl -sSL https://github.com/est31/cargo-udeps/releases/download/v0.1.49/cargo-udeps-v0.1.49-x86_64-unknown-linux-gnu.tar.gz | tar xvzf - --strip-components=2 -C ~/.cargo/bin ./cargo-udeps-v0.1.49-x86_64-unknown-linux-gnu/cargo-udeps
# curl -sSL https://github.com/taiki-e/cargo-hack/releases/download/v0.5.28/cargo-hack-x86_64-unknown-linux-gnu.tar.gz | tar xvzf - -C ~/.cargo/bin
cargo clippy --workspace --examples -- -Dwarnings --target=thumbv8m.main-none-eabihf
cargo clippy --workspace --examples --all-features -- -Dwarnings --target=thumbv8m.main-none-eabihf
cargo fmt -- --check
cargo hack build --optional-deps --each-feature --target=thumbv8m.main-none-eabihf
cargo hack build --examples --optional-deps --each-feature --target=thumbv8m.main-none-eabihf
cargo hack test -p rp235x-hal --doc --optional-deps --each-feature --features critical-section-impl
cargo hack test -p rp235x-hal --tests --optional-deps --each-feature --features critical-section-impl
cargo hack test -p rp235x-hal-macros --doc --optional-deps --each-feature
cargo hack test -p rp235x-hal-macros --tests --optional-deps --each-feature
cargo +nightly hack udeps --optional-deps --each-feature --target=thumbv8m.main-none-eabihf

