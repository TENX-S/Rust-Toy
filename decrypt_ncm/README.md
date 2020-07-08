# Decrypt ncm file


## Try it!

```shell script
$ cargo run --release --example ncm_test
```

## Use the multi-processing feature (Recommend!)

```shell script
$ cargo build --release
$ cargo run --release /absolute/path/to/directory/where/ncm/files/in
```



## API:

```rust
pub fn decrypt_ncm(_path: &str) -> Result<(), Box<dyn error::Error>> {}
```