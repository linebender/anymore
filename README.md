<div align="center">

# Anymore

`Any` extended with `Debug` support, developed for user interfaces

[![Latest published version.](https://img.shields.io/crates/v/anymore.svg)](https://crates.io/crates/anymore)
[![Documentation build status.](https://docs.rs/anymore/badge.svg)](https://docs.rs/anymore)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
\
[![Linebender Zulip, #anymore topic.](https://img.shields.io/badge/Linebender-%23rust--ui-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/422907-rust-ui/topic/Anymore/with/528830762)
[![Dependency staleness status.](https://deps.rs/repo/github/linebender/anymore/status.svg)](https://deps.rs/repo/github/linebender/anymore)
[![GitHub Actions CI status.](https://github.com/linebender/anymore/workflows/CI/badge.svg)](https://github.com/linebender/anymore/actions)

</div>

The Anymore library provides the `AnyDebug` trait, for dynamically typed values which can be inspected in Rust.

Full details, including usage instructions and motivations can be found in the [Anymore package's README](./anymore/README.md).

## Minimum supported Rust Version (MSRV)

This version of Anymore has been verified to compile with **Rust 1.86** and later.

Future versions of Anymore might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of Anymore's dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```

</details>

## Community

[![Linebender Zulip, #anymore topic.](https://img.shields.io/badge/Linebender-%23rust--ui-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/422907-rust-ui/topic/Anymore/with/528830762)

Discussion of Anymore development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically in [#rust ui > Anymore](https://xi.zulipchat.com/#narrow/channel/422907-rust-ui/topic/Anymore).
All public content can be read without logging in.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.
Please feel free to add your name to the [AUTHORS] file in any substantive pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
[AUTHORS]: ./AUTHORS
