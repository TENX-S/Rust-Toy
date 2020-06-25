# Generate random password


## How to use?

```shell script
$ cargo install grp

$ grp 16 4 4 
# Output: 8*_t99m(cCK6[UQi
# Length of password: 16, amount of symbols: 4, amount of numbers: 4

$ grp
# Output: nibIQG2D9D_uDtQ4
# Defaults to: grp 12 1 3
```




## Build yourself
```shell script
$ cargo run length symbols_count numbers_count --release

$ cargo run --release
#Defaults to: cargo run 12 1 3 --release
```


