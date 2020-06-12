# Arithmetic Parser Exercise

This is a parser that compute simple integer arithmetic based on an input string. Operations are performed from left to right with no order-of-operations, excepting brackets. Integers are parsed as written, with the following operators accepted on them in infix notation:

* `a`: Addition
* `b`: Subtraction
* `c`: Multiplication
* `d`: Division (only integer division supported)
* `e` and `f`: Open and close bracket, respectively

## Usage

`cargo run <expr>` where `<expr>` is a string expression

`cargo test` to run tests

## Known Bugs

`e`/`f` expressions collapse into concatenation when no outer operator is provided.
