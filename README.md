# cmp_string_find
research: comparing string match of rust

The regex(1.5) is slower than regex(1.4)

- rustc 1.53.0 (53cb7b09b 2021-06-17): regex(1.5)

|         `name`         | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:-----------------------|------------:|------------:|------------:|------------:|
| cmp-string-memchr      |   13.065 kc |   13.027 kc |   12.980 kc |   13.182 kc |
| cmp-string-libc        |   15.862 kc |   16.049 kc |   15.877 kc |   16.018 kc |
| cmp-string-aho         |   34.133 kc |   36.552 kc |   34.046 kc |   36.295 kc |
| cmp-regex-regex        |   47.032 kc |   57.893 kc |   47.409 kc |   58.085 kc |
| cmp-regex-fancy        |   49.731 kc |   60.348 kc |   49.884 kc |   62.002 kc |
| cmp-string-memmem      |   56.198 kc |   42.263 kc |   55.297 kc |   42.067 kc |
| cmp-regex-pcre         |   78.703 kc |  220.783 kc |   76.418 kc |  213.986 kc |
| cmp-string-twoway      |   98.689 kc |   80.366 kc |   95.423 kc |   81.659 kc |
| cmp-string-std         |  101.563 kc |   83.196 kc |  102.052 kc |   83.447 kc |
| cmp-regex-onig         |  166.274 kc |  166.873 kc |  514.217 kc |  508.249 kc |
| cmp-glob-globset       |  299.172 kc |  300.670 kc |  298.139 kc |  299.406 kc |
| cmp-glob-globber       | 1275.348 kc |  673.941 kc | 1290.863 kc |  673.000 kc |
| cmp-glob-glob          | 2152.369 kc |  968.428 kc | 2160.090 kc |  972.271 kc |
| cmp-glob-capturing     | 2458.600 kc | 1061.923 kc | 2467.678 kc | 1056.847 kc |


- rustc 1.52.0 (88f19c6da 2021-05-03): regex(1.5)

|         `name`         | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:-----------------------|------------:|------------:|------------:|------------:|
| cmp-string-memchr      |   12.791 kc |   12.827 kc |   14.275 kc |   14.236 kc |
| cmp-string-aho         |   34.146 kc |   36.585 kc |   34.246 kc |   36.487 kc |
| cmp-string-libc        |   37.105 kc |   46.236 kc |   45.364 kc |   51.340 kc |
| cmp-regex-regex        |   48.399 kc |   57.970 kc |   48.578 kc |   59.285 kc |
| cmp-regex-fancy        |   51.678 kc |   61.076 kc |   51.690 kc |   62.205 kc |
| cmp-string-memmem      |   56.153 kc |   41.380 kc |   54.482 kc |   41.553 kc |
| cmp-regex-pcre         |   79.352 kc |  203.931 kc |   91.500 kc |  264.839 kc |
| cmp-string-twoway      |   96.632 kc |   80.222 kc |   93.587 kc |   78.698 kc |
| cmp-string-std         |  101.449 kc |   80.869 kc |   98.608 kc |   82.995 kc |
| cmp-regex-onig         |  144.101 kc |  143.322 kc |  450.817 kc |  514.653 kc |
| cmp-glob-globset       |  298.435 kc |  303.670 kc |  298.675 kc |  299.082 kc |
| cmp-glob-globber       | 1297.217 kc |  716.429 kc | 1295.699 kc |  718.851 kc |
| cmp-glob-glob          | 2159.356 kc |  960.595 kc | 2158.646 kc |  961.765 kc |
| cmp-glob-capturing     | 2317.993 kc |  995.677 kc | 2387.127 kc |  996.094 kc |


- rustc 1.51.0 (2fd73fabe 2021-03-23): regex(1.4)

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


- rustc 1.41.1 (f3e1a954d 2020-02-24): regex(1.4)

|         `name`         | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:-----------------------|------------:|------------:|------------:|------------:|
| cmp-string-memchr      |   12.564 kc |   12.585 kc |   14.011 kc |   14.091 kc |
| cmp-regex-regex        |   19.266 kc |   19.244 kc |   20.873 kc |   20.923 kc |
| cmp-regex-fancy        |   23.569 kc |   23.676 kc |   24.577 kc |   24.628 kc |
| cmp-string-libc        |   37.006 kc |   46.429 kc |   43.978 kc |   51.068 kc |
| cmp-string-aho         |   38.413 kc |   37.728 kc |   38.449 kc |   38.054 kc |
| cmp-string-memmem      |   52.460 kc |   40.355 kc |   55.807 kc |   41.184 kc |
| cmp-regex-pcre         |   77.730 kc |  204.434 kc |   90.461 kc |  251.549 kc |
| cmp-string-twoway      |   90.486 kc |   72.062 kc |   88.144 kc |   70.514 kc |
| cmp-string-std         |   97.001 kc |   80.515 kc |   94.441 kc |   82.067 kc |
| cmp-regex-onig         |  151.627 kc |  141.215 kc |  447.086 kc |  445.707 kc |
| cmp-glob-globset       |  300.542 kc |  304.205 kc |  299.778 kc |  300.281 kc |
| cmp-glob-globber       | 1237.060 kc |  630.658 kc | 1241.291 kc |  631.961 kc |
| cmp-glob-glob          | 2163.196 kc |  973.118 kc | 2163.146 kc |  974.601 kc |
| cmp-glob-capturing     | 2341.567 kc | 1001.635 kc | 2301.930 kc | 1001.509 kc |

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

