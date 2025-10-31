# CombArc

CombArc stands for "clone-on-mutable-borrow automatic reference counter", but it is essentially
a copy-on-write that transparently wraps [`Arc::as_ref`] and [`Arc::make_mut`] around the
[`Deref`] and [`DerefMut`] traits, respectively.

As such, it has the same effects when borrowed mutably as [`Arc::make_mut`] when called:
* If there are no other references, it borrows normally.
* If there are strong references, it gets cloned.
* If there are no strong references but there are weak references, then all weak references get
  dissociated and can no longer upgrade.

This crate provides both a `CombArc` and `CombRc` type, the difference being that `CombArc` is
atomic and thread-safe, wrapping an [`Arc`], where `CombRc` is not thread-safe, wrapping over a
[`Rc`] and cannot be moved across threads.

When using this, there are a few things to keep in mind:
* Only mutable borrows on the `CombArc` and `CombRc` instances through `DerefMut` will trigger
  clones.
* Interior mutability which uses immutable borrows (e.g. [`Cell::set`]) won't trigger a clone
  even if there are multiple strong references. Use `clone_unique` to force a clone.
* Using `DerefMut` WITHOUT actually mutating the inner object can still trigger a clone. If you
  need manual control over when something is cloned, using the standard library's [`Cow`] type
  might be more what you are looking for.

## Unsafe code

This crate uses no unsafe code directly and only uses safe methods from the `alloc` crate.

## Examples

In this example, `CombArc` is used, but `CombRc` can be used interchangeably here.

```rust
use combarc::CombArc;
use std::cell::Cell;

// Both of these will be the same value.
let mut my_value = CombArc::new(Cell::new(false));
let another_value = my_value.clone();
assert_eq!(my_value, another_value);

// Cell::set uses interior mutability and does not mutably borrow,
// so they still point to the same memory address.
my_value.set(true);
assert_eq!(my_value, another_value);
assert_eq!(my_value.as_ptr(), another_value.as_ptr());

// get_mut does mutably borrow, thus `my_value` is cloned.
*my_value.get_mut() = false;
assert_ne!(my_value, another_value);

// Also, if there is only one reference, `my_value` is not cloned.
let address_before = my_value.as_ptr();
*my_value.get_mut() = true;
let address_after = my_value.as_ptr();
assert_eq!(address_before, address_after);

// Despite not pointing to the same thing, these are equal now.
assert_ne!(my_value.as_ptr(), another_value.as_ptr());
assert_eq!(my_value, another_value);
```

[`Cell::set`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.set
[`Arc::as_ref`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.as_ref
[`Arc::make_mut`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
