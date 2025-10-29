//! # CombArc
//!
//! CombArc stands for "clone-on-mutable-borrow automatic reference counter", but it is essentially
//! a copy-on-write that transparently wraps [`Arc::as_ref`] and [`Arc::make_mut`] around the
//! [`Deref`] and [`DerefMut`] traits, respectively.
//!
//! As such, it has the same effects when borrowed mutably as [`Arc::make_mut`] when called:
//! * If there are no other references, it borrows normally.
//! * If there are strong references, it gets cloned.
//! * If there are no strong references but there are weak references, then all weak references get
//!   dissociated and can no longer upgrade.
//!
//! Note that interior mutability (e.g. [`Cell::set`]) won't trigger a clone.
//!
//! This crate uses no unsafe code directly and only uses the `alloc` crate. It provides both a
//! `CombArc` and `CombRc`, the difference being that `CombArc` is atomic and thread-safe,
//! where `CombRc` is not thread-safe and cannot be moved across threads.
//!
//! [`Arc::as_ref`]: alloc::sync::Arc::as_ref
//! [`Arc::make_mut`]: alloc::sync::Arc::make_mut
//! [`Deref`]: core::ops::Deref
//! [`DerefMut`]: core::ops::DerefMut
//! [`Cell::set`]: core::cell::Cell::set
//!
//! ## Examples
//!
//! In this example, `CombArc` is used, but `CombRc` can be used interchangeably here.
//!
//! ```rust
//! use combarc::CombArc;
//! use std::cell::Cell;
//!
//! // Both of these will be the same value.
//! let mut my_value = CombArc::new(Cell::new(false));
//! let another_value = my_value.clone();
//! assert_eq!(my_value, another_value);
//!
//! // Cell::set uses interior mutability and does not mutably borrow, so
//! // they still point to the same memory address.
//! my_value.set(true);
//! assert_eq!(my_value, another_value);
//! assert_eq!(my_value.as_ptr(), another_value.as_ptr());
//!
//! // get_mut does mutably borrow, thus `my_value` is cloned.
//! *my_value.get_mut() = false;
//! assert_ne!(my_value, another_value);
//!
//! // Also, if there is only one reference, `my_value` is not cloned.
//! let address_before = my_value.as_ptr();
//! *my_value.get_mut() = true;
//! let address_after = my_value.as_ptr();
//! assert_eq!(address_before, address_after);
//!
//! // Despite not pointing to the same thing, these are equal now.
//! assert_ne!(my_value.as_ptr(), another_value.as_ptr());
//! assert_eq!(my_value, another_value);
//! ```
#![no_std]
#![forbid(unsafe_code)]
#![forbid(dead_code)]
#![forbid(missing_docs)]

extern crate alloc;

mod arc;
mod rc;

#[cfg(test)]
mod test;

pub use arc::CombArc;
pub use rc::CombRc;
