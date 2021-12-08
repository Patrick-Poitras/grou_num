# grou-num

(Pronounced "groo", from the Chiac meaning "big")

This package is a highly experimental, unstable big integer library.

I would not recommend using this for anything currently. It may become useful at some point if it benchmarks favorably.

Otherwise, this is intended to use the currently feature-gated 'bigint_helper_methods' to implement homebrew methods for bigint types.

Currently the focus is primarily on the `Grou` type, which is an unsigned integer.

# Immediate priorities:
* Multiplication
* Convert to decimal string.
* Modular arithmetic
* ShiftLeft / ShiftRight

# Secondary priorities:
* Implement split_n via macro and refactor.
* Implement `Add<u32>`, `Sub<u32>` for Grou.
