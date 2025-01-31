#!/bin/bash

# Runs a series of sample programs in QEMU and checks that the standard output
# is as expected.

rustup target add armv7r-none-eabi
rustup target add armv7r-none-eabihf
rustup install nightly # used for Tier 3 armv8r-none-eabihf
rustup component add rust-src --toolchain=nightly

FAILURE=0

fail() {
    echo "***************************************************"
    echo "MISMATCH: Binary $1 for target $2 mismatched"
    echo "***************************************************"
    FAILURE=1
}

# armv7r-none-eabi tests
for binary in hello registers svc; do
    cargo run --target=armv7r-none-eabi --bin $binary | tee ./target/$binary-armv7r-none-eabi.out
    diff ./cortex-r-examples/reference/$binary-armv7r-none-eabi.out ./target/$binary-armv7r-none-eabi.out || fail $binary "armv7r-none-eabi"
done

# armv7r-none-eabihf tests
for binary in hello registers svc; do
    cargo run --target=armv7r-none-eabihf --bin $binary | tee ./target/$binary-armv7r-none-eabihf.out
    diff ./cortex-r-examples/reference/$binary-armv7r-none-eabihf.out ./target/$binary-armv7r-none-eabihf.out || fail $binary "armv7r-none-eabihf"
done

# armv8r-none-eabihf tests
for binary in hello registers svc gic; do
    cargo +nightly run --target=armv8r-none-eabihf --bin $binary --features=gic -Zbuild-std=core | tee ./target/$binary-armv8r-none-eabihf.out
    diff ./cortex-r-examples/reference/$binary-armv8r-none-eabihf.out ./target/$binary-armv8r-none-eabihf.out || fail $binary "armv8r-none-eabihf"
done

if [ "$FAILURE" == "1" ]; then
    echo "Output comparison failed!"
    exit 1
else
    echo "Everything matches :)"
fi
