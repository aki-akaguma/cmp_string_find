# cmp_string_match
research: comparing string match of rust

rustc 1.51.0 (2fd73fabe 2021-03-23)

|         `name`         | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:-----------------------|------------:|------------:|------------:|------------:|
| cmp-string-memchr      |   12.483 kc |   12.481 kc |   13.900 kc |   13.856 kc |
| cmp-regex-regex        |   19.119 kc |   19.097 kc |   20.664 kc |   20.759 kc |
| cmp-regex-fancy        |   22.620 kc |   22.704 kc |   24.353 kc |   24.327 kc |
| cmp-string-aho         |   34.282 kc |   36.355 kc |   34.343 kc |   38.300 kc |
| cmp-string-libc        |   37.042 kc |   46.203 kc |   42.719 kc |   52.180 kc |
| cmp-string-memmem      |   57.622 kc |   40.067 kc |   55.967 kc |   41.256 kc |
| cmp-regex-pcre         |   79.521 kc |  207.884 kc |   91.492 kc |  252.451 kc |
| cmp-string-twoway      |   98.094 kc |   80.943 kc |   91.845 kc |   80.231 kc |
| cmp-string-std         |  102.515 kc |   82.253 kc |  103.957 kc |   81.925 kc |
| cmp-regex-onig         |  143.011 kc |  143.545 kc |  505.089 kc |  501.320 kc |
| cmp-glob-globset       |  288.332 kc |  285.554 kc |  285.973 kc |  286.569 kc |
| cmp-glob-globber       | 1295.129 kc |  678.078 kc | 1238.279 kc |  675.573 kc |
| cmp-glob-glob          | 2217.110 kc |  964.833 kc | 2217.373 kc |  966.302 kc |
| cmp-glob-capturing     | 2285.809 kc | 1001.333 kc | 2291.530 kc | 1000.428 kc |


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

