# cmp_string_match
research: comparing string match of rust

rustc 1.50.0 (cb75ad5db 2021-02-10)

|         `name`         | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:-----------------------|------------:|------------:|------------:|------------:|
| cmp-string-memchr      |   12.814 kc |   12.813 kc |   14.261 kc |   14.209 kc |
| cmp-regex-regex        |   19.837 kc |   19.331 kc |   21.503 kc |   20.755 kc |
| cmp-regex-fancy        |   23.520 kc |   23.174 kc |   25.632 kc |   24.253 kc |
| cmp-string-aho         |   35.130 kc |   36.672 kc |   34.760 kc |   38.471 kc |
| cmp-string-libc        |   38.148 kc |   47.292 kc |   45.845 kc |   51.273 kc |
| cmp-string-memmem      |   55.527 kc |   41.929 kc |   57.889 kc |   40.501 kc |
| cmp-regex-pcre         |   78.957 kc |  214.195 kc |   98.580 kc |  264.603 kc |
| cmp-string-twoway      |  100.485 kc |   82.323 kc |   95.763 kc |   80.605 kc |
| cmp-string-std         |  113.199 kc |   87.319 kc |  103.869 kc |   83.799 kc |
| cmp-regex-onig         |  148.976 kc |  144.010 kc |  545.470 kc |  496.881 kc |
| cmp-glob-globset       |  289.234 kc |  296.790 kc |  285.509 kc |  284.907 kc |
| cmp-glob-globber       | 1252.944 kc |  681.064 kc | 1299.456 kc |  673.239 kc |
| cmp-glob-glob          | 2242.488 kc | 1002.933 kc | 2283.155 kc |  967.394 kc |
| cmp-glob-capturing     | 2588.365 kc | 1005.336 kc | 2296.839 kc | 1003.254 kc |


This benchmark measures string search.
It is based on `std::str::find()`, which has the same functionality
as the `strstr()` and `memmem()` functions in C language.

Here is the code using `std::str::find()`:

```rust
pub fn do_find_string_std(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = line.find(pattern) {
            found += 1;
        }
    }
    Ok(found)
}
```

Receives multiple strings in `texts` and checks if each string contains
a `pattern` string. It then returns the count of strings it contained.

Measure the benchmark with what I wrote this part in other ways such as `regex`.

