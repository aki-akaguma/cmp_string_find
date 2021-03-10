# cmp_string_match
research: comparing string match of rust

rustc 1.50.0 (cb75ad5db 2021-02-10)

ja
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   26.625 kc |
| match-regex-regex      |   43.855 kc |
| match-regex-fancy      |   50.214 kc |
| match-string-aho       |   82.071 kc |
| match-string-memmem    |   89.202 kc |
| match-string-std       |  175.168 kc |
| match-string-twoway    |  176.930 kc |
| match-regex-onig       |  377.151 kc |
| match-regex-pcre       |  438.532 kc |
| match-glob-globset     |  649.236 kc |
| match-glob-globber     | 1447.249 kc |
| match-glob-glob        | 2067.181 kc |
| match-glob-capturing   | 2130.557 kc |

en
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   27.099 kc |
| match-regex-regex      |   43.733 kc |
| match-regex-fancy      |   50.176 kc |
| match-string-aho       |   72.849 kc |
| match-string-memmem    |  119.538 kc |
| match-regex-pcre       |  172.951 kc |
| match-string-twoway    |  209.495 kc |
| match-string-std       |  227.148 kc |
| match-regex-onig       |  317.353 kc |
| match-glob-globset     |  646.480 kc |
| match-glob-globber     | 3598.653 kc |
| match-glob-glob        | 4735.917 kc |
| match-glob-capturing   | 5043.070 kc |
