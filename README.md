# cmp_string_match
research: comparing string match of rust

rustc 1.50.0 (cb75ad5db 2021-02-10)

en:
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   26.860 kc |
| match-regex-regex      |   43.896 kc |
| match-regex-fancy      |   50.574 kc |
| match-string-aho       |   74.022 kc |
| match-string-memmem    |  125.493 kc |
| match-regex-pcre       |  166.933 kc |
| match-string-twoway    |  200.824 kc |
| match-string-std       |  217.625 kc |
| match-regex-onig       |  311.354 kc |
| match-glob-globset     |  644.282 kc |
| match-glob-globber     | 2678.772 kc |
| match-glob-glob        | 4737.899 kc |
| match-glob-capturing   | 5029.252 kc |

ja:
|         `name`         |   `bench`   |
|:-----------------------|------------:|
| match-string-memchr    |   26.689 kc |
| match-regex-regex      |   43.936 kc |
| match-regex-fancy      |   50.795 kc |
| match-string-aho       |   81.549 kc |
| match-string-memmem    |   90.230 kc |
| match-string-twoway    |  174.929 kc |
| match-string-std       |  177.071 kc |
| match-regex-onig       |  299.040 kc |
| match-regex-pcre       |  469.573 kc |
| match-glob-globset     |  642.470 kc |
| match-glob-globber     | 1440.961 kc |
| match-glob-glob        | 2063.705 kc |
| match-glob-capturing   | 2143.511 kc |
