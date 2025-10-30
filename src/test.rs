use alloc::rc::Rc;
use alloc::sync::Arc;

macro_rules! make_test {
    ($test_name:tt, $t:tt, $get_strong:tt, $strong:tt) => {
        #[test]
        fn $test_name() {
            use core::cell::Cell;
            use crate::$t as ReferenceCounter;

            // Both of these will be the same value.
            let mut my_value = ReferenceCounter::new(Cell::new(false));
            let another_value = my_value.clone();
            assert_eq!(my_value, another_value, "cloning a reference should yield equal values");

            // Cell::set uses interior mutability and does not mutably borrow, so they still point
            // to the same memory address.
            my_value.set(true);
            assert_eq!(my_value, another_value, "reference to same value");
            assert_eq!(my_value.as_ptr(), another_value.as_ptr(), "cloning a reference should yield equal addresses");

            // get_mut does mutably borrow, thus `my_value` is cloned.
            *my_value.get_mut() = false;
            assert_ne!(my_value, another_value, "mutable borrow should break the connection from the old value if NOT unique");

            // Also, if there is only one reference, `my_value` is not cloned.
            let address_before = my_value.as_ptr();
            *my_value.get_mut() = true;
            let address_after = my_value.as_ptr();
            assert_eq!(address_before, address_after, "mutable borrow should NOT break the connection from the old value IF unique");

            // Despite not pointing to the same thing, these are equal now.
            assert_ne!(my_value.as_ptr(), another_value.as_ptr(), "mutable borrow when NOT unique makes a unique one with different address");
            assert_eq!(my_value, another_value, "changing the value back failed");

            // Let's make a weak reference. It should break if we try to make_inner.
            let downgrade_disassociated = $strong::downgrade(ReferenceCounter::$get_strong(&my_value));
            let _ = ReferenceCounter::make_inner(my_value);
            assert!(downgrade_disassociated.upgrade().is_none());

            // But if there is another strong reference, the weak reference won't break.
            let _ = another_value.clone();
            let downgrade_still_works = $strong::downgrade(ReferenceCounter::$get_strong(&another_value));
            let _ = ReferenceCounter::make_inner(another_value);
            assert!(downgrade_still_works.upgrade().is_none());
        }
    };
}

make_test!(test_arc, CombArc, get_arc, Arc);
make_test!(test_rc, CombRc, get_rc, Rc);
