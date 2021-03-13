# cmp_string_match
research: comparing string match of rust

rustc 1.50.0 (cb75ad5db 2021-02-10)

|         `name`         | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:-----------------------|------------:|------------:|------------:|------------:|
| cmp-string-memchr      |   12.477 kc |   12.587 kc |   13.940 kc |   14.050 kc |
| cmp-regex-regex        |   19.005 kc |   18.713 kc |   20.888 kc |   20.931 kc |
| cmp-regex-fancy        |   22.957 kc |   22.652 kc |   24.851 kc |   24.705 kc |
| cmp-string-aho         |   36.059 kc |   36.275 kc |   34.365 kc |   38.160 kc |
| cmp-string-libc        |   37.132 kc |   46.151 kc |   45.409 kc |   51.458 kc |
| cmp-string-memmem      |   52.831 kc |   41.331 kc |   57.372 kc |   39.599 kc |
| cmp-regex-pcre         |   78.281 kc |  204.005 kc |   92.071 kc |  262.495 kc |
| cmp-string-twoway      |   97.555 kc |   82.479 kc |   95.033 kc |   80.995 kc |
| cmp-string-std         |  103.919 kc |   89.543 kc |  101.030 kc |   84.890 kc |
| cmp-regex-onig         |  144.206 kc |  146.922 kc |  492.660 kc |  496.659 kc |
| cmp-glob-globset       |  285.891 kc |  287.925 kc |  284.374 kc |  289.259 kc |
| cmp-glob-globber       | 1240.940 kc |  673.019 kc | 1299.597 kc |  684.208 kc |
| cmp-glob-glob          | 2208.728 kc |  979.100 kc | 2212.058 kc |  968.245 kc |
| cmp-glob-capturing     | 2322.110 kc | 1000.022 kc | 2288.584 kc |  999.854 kc |


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

