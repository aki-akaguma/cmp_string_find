# cmp_string_match
research: comparing string match of rust

rustc 1.50.0 (cb75ad5db 2021-02-10)

en:
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   12.519 kc |
| match-regex-regex      |   20.611 kc |
| match-regex-fancy      |   23.646 kc |
| match-string-aho       |   34.361 kc |
| match-string-memmem    |   58.536 kc |
| match-regex-pcre       |   78.797 kc |
| match-string-twoway    |   92.087 kc |
| match-string-std       |  101.663 kc |
| match-regex-onig       |  143.147 kc |
| match-glob-globset     |  302.927 kc |
| match-glob-globber     | 1240.582 kc |
| match-glob-glob        | 2216.223 kc |
| match-glob-capturing   | 2347.241 kc |

ja:
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   12.533 kc |
| match-regex-regex      |   20.644 kc |
| match-regex-fancy      |   23.549 kc |
| match-string-aho       |   37.857 kc |
| match-string-memmem    |   42.407 kc |
| match-string-twoway    |   80.828 kc |
| match-string-std       |   80.931 kc |
| match-regex-onig       |  140.793 kc |
| match-regex-pcre       |  208.224 kc |
| match-glob-globset     |  302.382 kc |
| match-glob-globber     |  682.928 kc |
| match-glob-glob        |  963.326 kc |
| match-glob-capturing   | 1007.421 kc |
