# grou-num

(Pronounced "groo", from the Chiac meaning "big")

This package is a highly experimental, unstable big integer library.

I would not recommend using this for anything currently. It is missing critical features.
I'm using this primarily as a hobby project to learn the faster implementations of base arithmetic operations on big integer types. 
It may become useful at some point if it benchmarks favorably. See the development plan below for more details.

Otherwise, this is also intended to use the currently feature-gated 'bigint_helper_methods' to implement homebrew methods for bigint types.

Currently the focus is primarily on the `Grou` type, which is an unsigned integer.

# Development plan
The current plan is going to focus first on adding features that may not be optimized. After a somewhat complete set of features is complete,
then, optimization work will begin in order to minimize runtime. This may require rewriting a substantial amount of code. The purpose
of this "two-pass" approach is firstly to get familiar with the base concepts that govern big integer libraries. (This is in fact my
main reason in developing this entire library.) Then once I am more familiar with the base algorithms, I can look at reducing the time footprint
of each of the components, where necessary.

Once that is complete, the code is going to be refactored, documented, and then published to crates for everyone to use. I don't intend on
publishing there until the process is complete.

# Immediate priorities:
* Refactor radix conversion, add binary, hexadecimal.
* Multiplication benchmarking
* Do testing of small addition, small multiplication for Grou.
* Modular arithmetic
* ShiftLeft / ShiftRight

# Secondary priorities:
* Add some more tests for the multiplication individual methods.
* Implement split_n via macro and refactor.
