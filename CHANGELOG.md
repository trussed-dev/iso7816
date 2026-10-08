# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Fix `Aid::matches` function. Previously, the function accepted all AIDs that start with the truncated prefix of this AID, even if the suffix is different. Now it only accepts AIDs that are a prefix of this AID and have at least the required truncated length.
- Fix panic and formatting inconsistencies in `Debug` implementation for `Aid`.
- Fix `Aid::pix` to return only the PIX. Previously, the function would not check the AID length and always return 11 bytes even if the actual PIX is shorter.
- Make `Aid::as_bytes` and `Aid::truncated` `const`.

## [0.2.1] - 2026-09-21

- Fix `Aid::category` function. Previously, the function checked for the ASCII characters A, D, E and F instead of the byte prefix 0xA, 0xD, 0xE and 0xF, classifying almost all AIDs as `Category::Other`.
- Fix AID validation for national and international AIDs in `Aid::new`, `Aid::new_truncatable`, `Aid::try_new` and `Aid::try_new_truncatable`. Previously, the check enforced a maximum length of 5 instead of a minimum length of 5. Due to the aforementioned bug in `Aid::category`, it did not affect national and international AIDs but only AIDs with a custom prefix.

## [0.2.0] - 2025-09-25

- Update `heapless` to 0.9.1 ([#30][])
- Update `heapless-bytes` to 0.5.0 ([#30][])

[#30]: https://github.com/trussed-dev/iso7816/pull/30

## [0.1.4] - 2025-03-04

- Add support for `defmt` ([#25][])
- Fix heapless-byte feature flag ([#26][])
- Improve compiled binary size ([#23][])

[#26]: https://github.com/trussed-dev/iso7816/pull/26
[#25]: https://github.com/trussed-dev/iso7816/pull/25
[#23]: https://github.com/trussed-dev/iso7816/pull/23

## [0.1.3] - 2024-10-18

- CommandView: Precise lifetime of data ([#22][])

[#22]: https://github.com/trussed-dev/iso7816/pull/22

## [0.1.2]

- Improve `CommandView` API ([#12][])
- Add a command builder API ([#13][])
- Add support for all status bytes ([#17][])
- Improve stack usage in `CommandView::to_owned` ([#20][])

[#12]: https://github.com/trussed-dev/iso7816/pull/12
[#13]: https://github.com/trussed-dev/iso7816/pull/13
[#17]: https://github.com/trussed-dev/iso7816/pull/17
[#20]: https://github.com/trussed-dev/iso7816/pull/20

## [0.1.1] - 2022-08-22
- various fixes @robin-nitrokey @sosthene-nitrokey

## [0.1.0] - 2022-03-05

- use 2021 edition
- non-alpha release to bump dependees
- add an experimental CommandView

[Unreleased]: https://github.com/trussed-dev/iso7816/compare/0.2.1...HEAD
[0.2.1]: https://github.com/trussed-dev/iso7816/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/trussed-dev/iso7816/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/trussed-dev/iso7816/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/trussed-dev/iso7816/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/trussed-dev/iso7816/releases/tag/0.1.0
