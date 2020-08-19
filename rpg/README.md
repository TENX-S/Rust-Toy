# Random password generator


## Requirement

You must use the **nightly** version to compile this crate

## Try it!
```shell script
# Default case: amount of letters: 10, symbols: 2, numbers: 3
$ cargo run --release --example rpg_test

# Specify the parameter: amount of letters: 16, symbols: 2, numbers: 3
$ cargo run --release --example rpg_test 16 2 3

# Try a larger number!
$ cargo run --release --example rpg_test 200000 200 300
```
