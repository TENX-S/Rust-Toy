# Generate random password




## Try it!
```shell script
$ cargo run --release --example grp_test
# Default case: password length: 10, amount of symbols: 2, amount of numbers: 3

$ cargo run --release --example grp_test 16 2 3
# Specify the parameter: password length: 16, symbols amount: 2, numbers amount: 3
# The result will be saved in random_password.txt on the desktop

$ cargo run --release --example grp_test 200000 200 300
# Try a larger number!
```










