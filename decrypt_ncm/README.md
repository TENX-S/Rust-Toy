# Decrypt ncm file


## Try it!

```shell script
cargo run --release --example ncm_test
```



## API:

```rust
pub fn decrypt_ncm(_path: &str) -> Result<(), Box<dyn error::Error>> {}
```