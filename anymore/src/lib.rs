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
//! When you have `dyn AnyDebug` contained in a smart pointer, such as [`Box`] or
//! [`Arc`][alloc::sync::Arc],
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
//! - `alloc` (enabled by default): Implement downcasting from [`Box`]es.
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

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// A trait to implement dynamic typing.
///
/// This trait is the same as the standard library [`Any`] trait,
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

impl dyn AnyDebug {
    /// Returns some shared reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn downcast_ref<T: AnyDebug>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }

    /// Returns some exclusive reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn downcast_mut<T: AnyDebug>(&mut self) -> Option<&mut T> {
        (self as &mut dyn Any).downcast_mut::<T>()
    }

    /// Access the actual type of this [`AnyDebug`].
    ///
    /// Forwards to the method defined on the type [`Box<dyn Any>`].
    ///
    /// ## Errors
    ///
    /// If the message contained within `self` is not of type `T`, returns `self`.
    #[cfg(feature = "alloc")]
    pub fn downcast<T: AnyDebug>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.is::<T>() {
            Ok((self as Box<dyn Any>).downcast::<T>().unwrap())
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the inner type is the same as `T`.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn is<T: AnyDebug>(&self) -> bool {
        let this: &dyn Any = self;
        this.is::<T>()
    }
}

impl dyn AnyDebug + Send {
    /// Returns some shared reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn downcast_ref<T: AnyDebug>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }

    /// Returns some exclusive reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn downcast_mut<T: AnyDebug>(&mut self) -> Option<&mut T> {
        (self as &mut dyn Any).downcast_mut::<T>()
    }

    /// Access the actual type of this [`AnyDebug`].
    ///
    /// Forwards to the method defined on the type [`Box<dyn Any>`].
    ///
    /// ## Errors
    ///
    /// If the message contained within `self` is not of type `T`, returns `self`.
    #[cfg(feature = "alloc")]
    pub fn downcast<T: AnyDebug>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.is::<T>() {
            Ok((self as Box<dyn Any>).downcast::<T>().unwrap())
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the inner type is the same as `T`.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn is<T: AnyDebug>(&self) -> bool {
        let this: &dyn Any = self;
        this.is::<T>()
    }
}

impl dyn AnyDebug + Send + Sync {
    /// Returns some shared reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn downcast_ref<T: AnyDebug>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }

    /// Returns some exclusive reference to the inner value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn downcast_mut<T: AnyDebug>(&mut self) -> Option<&mut T> {
        (self as &mut dyn Any).downcast_mut::<T>()
    }

    /// Access the actual type of this [`AnyDebug`].
    ///
    /// Forwards to the method defined on the type [`Box<dyn Any>`].
    ///
    /// ## Errors
    ///
    /// If the message contained within `self` is not of type `T`, returns `self`.
    #[cfg(feature = "alloc")]
    pub fn downcast<T: AnyDebug>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.is::<T>() {
            Ok((self as Box<dyn Any>).downcast::<T>().unwrap())
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the inner type is the same as `T`.
    ///
    /// Forwards to the method defined on the type `dyn Any`.
    pub fn is<T: AnyDebug>(&self) -> bool {
        let this: &dyn Any = self;
        this.is::<T>()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "alloc"))]
    compile_error!("Anymore's tests need the `alloc` crate feature to be enabled.");

    use crate::AnyDebug;
    use alloc::{boxed::Box, format};

    #[derive(Debug)]
    struct SomeMessage(u32);

    #[test]
    fn any_debug_correct_typename() {
        let val = SomeMessage(4);
        let val: &dyn AnyDebug = &val;
        assert!(val.type_name().contains("SomeMessage"));
    }

    #[test]
    fn any_debug_shared_correct_debug() {
        let val = SomeMessage(5);
        let val: &dyn AnyDebug = &val;
        let format_result = format!("{val:?}");
        assert!(format_result.contains("SomeMessage"));
        assert!(format_result.contains("5"));
    }
    #[test]
    fn any_debug_excl_correct_debug() {
        let mut val = SomeMessage(6);
        let val: &mut dyn AnyDebug = &mut val;
        let format_result = format!("{val:?}");
        assert!(format_result.contains("SomeMessage"));
        assert!(format_result.contains("6"));
    }
    #[test]
    fn any_debug_box_correct_debug() {
        let val = SomeMessage(7);
        let val: Box<dyn AnyDebug> = Box::new(val);
        let format_result = format!("{val:?}");
        assert!(format_result.contains("SomeMessage"));
        assert!(format_result.contains("7"));
    }

    #[test]
    fn any_debug_normal_is() {
        let val = SomeMessage(10);
        let val: &dyn AnyDebug = &val;
        assert!(val.is::<SomeMessage>());
        assert!(!val.is::<u32>());
    }
    #[test]
    fn any_debug_normal_downcast_ref() {
        let val = SomeMessage(11);
        let val: &dyn AnyDebug = &val;
        assert_eq!(val.downcast_ref::<SomeMessage>().unwrap().0, 11);
    }
    #[test]
    fn any_debug_normal_downcast_mut() {
        let mut val = SomeMessage(12);
        let val_mut: &mut dyn AnyDebug = &mut val;
        val_mut.downcast_mut::<SomeMessage>().unwrap().0 = 13;
        assert!(val_mut.downcast_mut::<u32>().is_none());
        assert_eq!(val.0, 13);
    }

    #[test]
    fn any_debug_normal_downcast() {
        let val = SomeMessage(14);
        let val: Box<dyn AnyDebug> = Box::new(val);
        let val = val.downcast::<u32>().unwrap_err();
        let val = val.downcast::<SomeMessage>().unwrap();
        assert_eq!(val.0, 14);
    }

    #[test]
    fn any_debug_send_is() {
        let val = SomeMessage(20);
        let val: &(dyn AnyDebug + Send) = &val;
        assert!(val.is::<SomeMessage>());
        assert!(!val.is::<u32>());
    }
    #[test]
    fn any_debug_send_downcast_ref() {
        let val = SomeMessage(21);
        let val: &(dyn AnyDebug + Send) = &val;
        assert_eq!(val.downcast_ref::<SomeMessage>().unwrap().0, 21);
    }
    #[test]
    fn any_debug_send_downcast_mut() {
        let mut val = SomeMessage(22);
        let val_mut: &mut (dyn AnyDebug + Send) = &mut val;
        val_mut.downcast_mut::<SomeMessage>().unwrap().0 = 23;
        assert!(val_mut.downcast_mut::<u32>().is_none());
        assert_eq!(val.0, 23);
    }
    #[test]
    fn any_debug_send_downcast() {
        let val = SomeMessage(24);
        let val: Box<(dyn AnyDebug + Send)> = Box::new(val);
        let val = val.downcast::<u32>().unwrap_err();
        let val = val.downcast::<SomeMessage>().unwrap();
        assert_eq!(val.0, 24);
    }

    #[test]
    fn any_debug_send_sync_is() {
        let val = SomeMessage(30);
        let val: &(dyn AnyDebug + Send + Sync) = &val;
        assert!(val.is::<SomeMessage>());
        assert!(!val.is::<u32>());
    }
    #[test]
    fn any_debug_send_sync_downcast_ref() {
        let val = SomeMessage(31);
        let val: &(dyn AnyDebug + Send + Sync) = &val;
        assert_eq!(val.downcast_ref::<SomeMessage>().unwrap().0, 31);
    }
    #[test]
    fn any_debug_send_sync_downcast_mut() {
        let mut val = SomeMessage(32);
        let val_mut: &mut (dyn AnyDebug + Send + Sync) = &mut val;
        val_mut.downcast_mut::<SomeMessage>().unwrap().0 = 33;
        assert!(val_mut.downcast_mut::<u32>().is_none());
        assert_eq!(val.0, 33);
    }
    #[test]
    fn any_debug_send_sync_downcast() {
        let val = SomeMessage(34);
        let val: Box<(dyn AnyDebug + Send + Sync)> = Box::new(val);
        let val = val.downcast::<u32>().unwrap_err();
        let val = val.downcast::<SomeMessage>().unwrap();
        assert_eq!(val.0, 34);
    }
}
