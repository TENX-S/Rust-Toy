# Decrypt ncm file


## Try it!

```shell script
$ cargo run --release --example ncm_test
```

## Use the multi-processing feature (Recommended!)

```shell script
$ cargo build --release
$ cargo run --release /absolute/path/to/directory/where/ncm/files/in
```



## Tips:

As far as we know from test, it's not quite stable right at the time after the commit of "73a4300", some converted files(.flac/.mp3)
may have a very short interruption (< 1 sec).

But it's hard to happen:)
