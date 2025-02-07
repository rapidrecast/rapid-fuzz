# Rapid-Fuzz
A crate that can be used to build fuzzing binaries

## What is fuzzing

Fuzzing is a way of testing software by giving it random data and conditions.
While the data is random, fuzz tests tend to have a way of displaying the steps that were taken and providing a way of reproducing the test.

## Why a new crate for fuzzing in Rust?

While there are already existing crates for fuzzing, they have limitations.
For one, the fuzz tests have limited runtime duration (usually under 20 seconds).
And second, you don't get much control.
You can't easily control memory, for example.

## What's special about Rapid-Fuzz

Rapid-Fuzz provides a way to conveniently build executable binaries that include everything you would need to make your deterministic tests.

To build a fuzz test with Rapid-Fuzz, include the crate as a dependency and launch it via the Builder.

