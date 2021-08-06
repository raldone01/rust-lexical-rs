#!/bin/bash
#
# Comprehensive unittests using both Valgrind and Miri to
# ensure the code doesn't have any obvious memory issues
# in the conversion routines.

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

export RUSTFLAGS="--deny warnings"

cargo +nightly test
cargo +nightly test --all-features
if [ "$SKIP_VALGRIND" == "" ]; then
    cargo +nightly valgrind test
fi
if [ "$SKIP_MIRI" == "" ]; then
    cargo +nightly miri test --tests
fi

# Test various feature combinations.
FEATURES=(
    "compact"
    "format"
    "power-of-two"
    "radix"
    "compact,format"
    "compact,radix"
    "format,power-of-two"
    "format,radix"
)
if [ "$SKIP_FEATURES" == "" ]; then
    for features in "${FEATURES[@]}"; do
        cargo +nightly test --features="$features"
    done
fi

# This is very slow, but uses Valgrind to test all features.
if [ "$SKIP_VALGRIND" == "" ] && [ "$LEXICAL_VALGRIND_TEST_ALL" != "" ]; then
    for features in "${FEATURES[@]}"; do
        cargo +nightly valgrind test --features="$features"
    done
fi

# This is very slow, but uses Miri to test all features.
if [ "$SKIP_MIRI" == "" ] && [ "$LEXICAL_MIRI_TEST_ALL" != "" ]; then
    for features in "${FEATURES[@]}"; do
        cargo +nightly miri test --features="$features" --tests
    done
fi
