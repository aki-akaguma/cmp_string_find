# Changelog: cmp_string_find

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] *
### Changed
* reformat `CHANGELOG.md`


## [0.1.8] (2021-11-04)
### Changed
* update depends: fancy-regex(0.7.1), onig(6.3.1)

## [0.1.7] (2021-06-30)
### Changed
* update depends: fancy-regex(0.6.0), onig(6.2.0)

## [0.1.6] (2021-06-30)
### Added
* memx-cdy(0.1) into `dev-dependencies`

### Changed
* update depends: anyhow(1.0.41), globset(0.4.8)
* update depends: libc(0.2.97), twoway(0.2.2)

## [0.1.5] (2021-05-10)
### Changed
* more short measuring and warming time

## [0.1.4] (2021-05-09)
### Changed
* update depends: regex(1.5.4)

## [0.1.3] (2021-04-25)
### Changed
* update depends: regex(1.4.6)
* rename crate name: cmp_string_match to cmp_string_find
* update depends: anyhow(1.0.40): Reduce memory footprint of errors on Rust versions 1.51+
* update depends: libc(0.2.91) and many

## [0.1.2] (2021-03-13)
### Added
* x86_64-unknown-linux-musl to bench target

### Changed
* remake a report format
* rename do_match_string_std() to do_find_string_std()

## [0.1.1] (2021-03-12)
### Added
* libc::memmem

### Changed
* update crate: regex

## [0.1.0] (2021-03-10)
* first commit

[Unreleased]: https://github.com/aki-akaguma/cmp_string_find/compare/v0.1.4..HEAD
[0.1.4]: https://github.com/aki-akaguma/cmp_string_find/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/cmp_string_find/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/cmp_string_find/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/cmp_string_find/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/cmp_string_find/releases/tag/v0.1.0
