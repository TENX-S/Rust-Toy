# Generate random password




## Try it!
```shell script
$ cargo run --release --example grp_test
# Default case: password length: 12, amount of symbols: 1, amount of numbers: 3

$ cargo run --release --example grp_test 16 2 3
# Specify the parameter: password length: 16, symbols amount: 2, numbers amount: 3
```



## Installation

```shell script
$ cargo install grp

$ grp 16 4 4 
# Output: 8*_t99m(cCK6[UQi
# Length of password: 16, amount of symbols: 4, amount of numbers: 4

$ grp
# Output: nibIQG2D9D_uDtQ4
# Defaults to: grp 12 1 3
```








