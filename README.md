# cmp_string_match
research: comparing string match of rust

rustc 1.50.0 (cb75ad5db 2021-02-10)

en:
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   12.539 kc |
| match-regex-regex      |   18.928 kc |
| match-regex-fancy      |   22.688 kc |
| match-string-aho       |   34.394 kc |
| match-string-libc      |   37.376 kc |
| match-string-memmem    |   53.627 kc |
| match-regex-pcre       |   81.956 kc |
| match-string-twoway    |   97.743 kc |
| match-string-std       |  105.379 kc |
| match-regex-onig       |  146.449 kc |
| match-glob-globset     |  286.613 kc |
| match-glob-globber     | 1245.398 kc |
| match-glob-glob        | 2235.350 kc |
| match-glob-capturing   | 2318.177 kc |

ja:
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   12.470 kc |
| match-regex-regex      |   19.077 kc |
| match-regex-fancy      |   22.827 kc |
| match-string-aho       |   36.416 kc |
| match-string-memmem    |   41.918 kc |
| match-string-libc      |   46.832 kc |
| match-string-twoway    |   83.241 kc |
| match-string-std       |   89.070 kc |
| match-regex-onig       |  140.230 kc |
| match-regex-pcre       |  215.313 kc |
| match-glob-globset     |  286.490 kc |
| match-glob-globber     |  674.985 kc |
| match-glob-glob        |  981.140 kc |
| match-glob-capturing   |  999.353 kc |
