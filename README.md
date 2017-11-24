# haraka-rs [![Build Status](https://travis-ci.org/gendx/haraka-rs.svg?branch=master)](https://travis-ci.org/gendx/haraka-rs)

A Rust implementation of the [Haraka](https://github.com/kste/haraka) (v2) short-input hash function.

## Implementation

As for the original Haraka implementation in C, this project relies on AES-NI instructions.
Therefore, a nightly Rust compiler is required, due to `#![feature(asm)]`.

Besides the original 5-round Haraka functions (with 256 and 512 bits of input), extensions to 6 rounds are provided.
This is to target collision resistance, contrary to the 5-round versions that only provide preimage resistance.

## Testing

Unit tests are implemented to check the logic of Haraka's building blocks.
High-level test vectors were generated from the [Python implementation](https://github.com/kste/haraka/blob/master/code/python/ref.py) of Haraka (for the 5-round versions).

## License

MIT

