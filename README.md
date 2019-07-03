# lotsa

Lotsa is a simple Rust library for executing lots of operations spread over any number of threads.

This is port of the Go library https://github.com/tidwall/lotsa.

## Example 

```
fn main() {
    // The `i` and `thread` params correspond to the op index and thread
    // number, respectively.
    lotsa::ops(100000, 1, |i, thread| {
        fibonacci(10);
    });
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

Outputs something like:

```
100000 ops in 0.061 secs (1,639,344 ops/sec)
```

## Contact

Josh Baker [@tidwall](http://twitter.com/tidwall)

## License

Source code is available under the MIT [License](/LICENSE).
