macro_rules! make_test {
    ($test_name:tt, $t:tt) => {
        #[test]
        fn $test_name() {
            use core::cell::Cell;
            use crate::$t as ReferenceCounter;

            // Both of these will be the same value.
            let mut my_value = ReferenceCounter::new(Cell::new(false));
            let another_value = my_value.clone();
            assert_eq!(my_value, another_value);

            // Cell::set uses interior mutability and does not mutably borrow, so they still point
            // to the same memory address.
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
        }
    };
}

make_test!(test_arc, CombArc);
make_test!(test_rc, CombRc);
