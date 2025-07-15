// Copyright 2025 the Anymore Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The Anymore crate provides the [`AnyDebug`][] trait, for dynamically typed values which
//! can be inspected in Rust.
//!
//! These can be used in any program domain, but have been developed with graphical
//! user interface libraries as the target consumer.
//! The `AnyDebug` trait also has the [`type_name`](AnyDebug::type_name) method, which
//! can be used to access the type name of the contained value.
//!
//! ## Usage
//!
//! The traits in this crate can be used in the same way that you would use
//! [`Any`], with the additional capability that the Debug implementation is meaningful.
//! See the [module-level documentation][core::any] of `Any` for more details of how it
//! can be used.
//!
//! ## Smart pointers and `dyn AnyDebug`
//!
//! When you have `dyn AnyDebug` contained in a smart pointer, such as `Box` or `Arc`,
//! the [`type_name`][AnyDebug::type_name] method will give the type name of the smart
//! pointer, rather than the type name of the contained value. This can be avoided by
//! converting the smart pointer into a `&dyn AnyDebug` instead, which will return the
//! object’s type name. This is the [same caveat][core::any#smart-pointers-and-dyn-any]
//! seen with the`type_id` method on `Any`.
//!
//! ## Motivation
//!
//! In user interface contexts, there is often a need for passing dynamically typed values.
//! In [Xilem](https://docs.rs/xilem/latest/xilem/), for example, one of the key concepts is
//! dispatching arbitrary messages to the correct component (called `View` in `Xilem`).
//! Each `View` expects messages of a specific type or types, and knows how to handle these.
//! However, sometimes, due to bugs elsewhere in the app, a view will receive a message of a
//! type it didn't expect. If it were using the standard library, `Any` type, there would be no
//! feasible way for the author of that `View` implementation to learn what the underlying type was.
//! This can make understanding this failure quite challenging.
//! Because of this, Xilem uses the `AnyDebug` trait for these messages, so they can be inspected.
//!
//! A similar need arises in [Masonry](https://docs.rs/masonry/latest/masonry/), which is the widget toolkit
//! co-developed with Xilem.
//!
//! ## Features
//!
//! - `std` (enabled by default): Use the Rust standard library.
// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

use core::any::Any;
use core::fmt::Debug;

#[cfg(feature = "std")]
extern crate std;

/// A trait to implement dynamic typing.
///
/// This type is the same as the standard library [`Any`] trait,
/// except that it can be debug printed.
///
/// See also the [crate level documentation](crate) for more details.
pub trait AnyDebug: Debug + Any {
    /// Returns the [`type_name`](core::any::type_name) of this value's concrete type.
    ///
    /// This is useful for debugging downcasting failures.
    /// That is, this method being present does likely increase binary size,
    /// but we believe that tradeoff is worthwhile to make debugging easier
    /// for our users.
    // TODO: Do we want to feature gate this?
    // TODO: Maybe only implement if debug_assertions are enabled?
    fn type_name(&self) -> &'static str;
}
impl<T: Any + Debug> AnyDebug for T {
    fn type_name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
}

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
