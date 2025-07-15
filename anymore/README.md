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

<!-- We use cargo-rdme to update the README with the contents of lib.rs.
To edit the following section, update it in lib.rs, then run:
cargo rdme --workspace-project=anymore --heading-base-level=0
Full documentation at https://github.com/orium/cargo-rdme -->

<!-- Intra-doc links used in lib.rs should be evaluated here. 
See https://linebender.org/blog/doc-include/ for related discussion. -->
[`AnyDebug`]: https://docs.rs/anymore/latest/anymore/trait.AnyDebug.html
[core::any]: https://doc.rust-lang.org/stable/core/any/index.html
[core::any#smart-pointers-and-dyn-any]: https://doc.rust-lang.org/stable/core/any/index.html#smart-pointers-and-dyn-any
[AnyDebug::type_name]: https://docs.rs/anymore/latest/anymore/trait.AnyDebug.html#tymethod.type_name
<!-- cargo-rdme start -->

The Anymore crate provides the [`AnyDebug`][] trait, for dynamically typed values which
can be inspected in Rust.

These can be used in any program domain, but have been developed with graphical
user interface libraries as the target consumer.
The `AnyDebug` trait also has the [`type_name`](AnyDebug::type_name) method, which
can be used to access the type name of the contained value.

## Usage

The traits in this crate can be used in the same way that you would use
[`Any`], with the additional capability that the Debug implementation is meaningful.
See the [module-level documentation][core::any] of `Any` for more details of how it
can be used.

## Smart pointers and `dyn AnyDebug`

When you have `dyn AnyDebug` contained in a smart pointer, such as `Box` or `Arc`,
the [`type_name`][AnyDebug::type_name] method will give the type name of the smart
pointer, rather than the type name of the contained value. This can be avoided by
converting the smart pointer into a `&dyn AnyDebug` instead, which will return the
object’s type name. This is the [same caveat][core::any#smart-pointers-and-dyn-any]
seen with the`type_id` method on `Any`.

## Motivation

In user interface contexts, there is often a need for passing dynamically typed values.
In [Xilem](https://docs.rs/xilem/latest/xilem/), for example, one of the key concepts is
dispatching arbitrary messages to the correct component (called `View` in `Xilem`).
Each `View` expects messages of a specific type or types, and knows how to handle these.
However, sometimes, due to bugs elsewhere in the app, a view will receive a message of a
type it didn't expect. If it were using the standard library, `Any` type, there would be no
feasible way for the author of that `View` implementation to learn what the underlying type was.
This can make understanding this failure quite challenging.
Because of this, Xilem uses the `AnyDebug` trait for these messages, so they can be inspected.

A similar need arises in [Masonry](https://docs.rs/masonry/latest/masonry/), which is the widget toolkit
co-developed with Xilem.

## Features

- `std` (enabled by default): Use the Rust standard library.

<!-- cargo-rdme end -->

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

[![Linebender Zulip](https://img.shields.io/badge/Xi%20Zulip-%23general-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/147921-general)

Discussion of Anymore development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically [#rust ui > Anymore](https://xi.zulipchat.com/#narrow/channel/422907-rust-ui/topic/Anymore).
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
