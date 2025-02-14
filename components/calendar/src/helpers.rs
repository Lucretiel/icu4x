// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Calculate `(n / d, n % d)` such that the remainder is always positive.
pub fn div_rem_euclid(n: i32, d: i32) -> (i32, i32) {
    debug_assert!(d > 0);
    let (a, b) = (n / d, n % d);
    if n >= 0 || b == 0 {
        (a, b)
    } else {
        (a - 1, d + b)
    }
}

/// Calculate `n / d` such that the remainder is always positive.
/// This is equivalent to quotient() in the Reingold/Dershowitz Lisp code
pub const fn quotient(n: i32, d: i32) -> i32 {
    debug_assert!(d > 0);
    // Code can use int_roundings once stabilized
    // https://github.com/rust-lang/rust/issues/88581
    let (a, b) = (n / d, n % d);
    if n >= 0 || b == 0 {
        a
    } else {
        a - 1
    }
}

#[test]
fn test_div_rem_euclid() {
    assert_eq!(div_rem_euclid(i32::MIN, 1), (-2147483648, 0));
    assert_eq!(div_rem_euclid(i32::MIN, 2), (-1073741824, 0));
    assert_eq!(div_rem_euclid(i32::MIN, 3), (-715827883, 1));

    assert_eq!(div_rem_euclid(-10, 1), (-10, 0));
    assert_eq!(div_rem_euclid(-10, 2), (-5, 0));
    assert_eq!(div_rem_euclid(-10, 3), (-4, 2));

    assert_eq!(div_rem_euclid(-9, 1), (-9, 0));
    assert_eq!(div_rem_euclid(-9, 2), (-5, 1));
    assert_eq!(div_rem_euclid(-9, 3), (-3, 0));

    assert_eq!(div_rem_euclid(-8, 1), (-8, 0));
    assert_eq!(div_rem_euclid(-8, 2), (-4, 0));
    assert_eq!(div_rem_euclid(-8, 3), (-3, 1));

    assert_eq!(div_rem_euclid(-2, 1), (-2, 0));
    assert_eq!(div_rem_euclid(-2, 2), (-1, 0));
    assert_eq!(div_rem_euclid(-2, 3), (-1, 1));

    assert_eq!(div_rem_euclid(-1, 1), (-1, 0));
    assert_eq!(div_rem_euclid(-1, 2), (-1, 1));
    assert_eq!(div_rem_euclid(-1, 3), (-1, 2));

    assert_eq!(div_rem_euclid(0, 1), (0, 0));
    assert_eq!(div_rem_euclid(0, 2), (0, 0));
    assert_eq!(div_rem_euclid(0, 3), (0, 0));

    assert_eq!(div_rem_euclid(1, 1), (1, 0));
    assert_eq!(div_rem_euclid(1, 2), (0, 1));
    assert_eq!(div_rem_euclid(1, 3), (0, 1));

    assert_eq!(div_rem_euclid(2, 1), (2, 0));
    assert_eq!(div_rem_euclid(2, 2), (1, 0));
    assert_eq!(div_rem_euclid(2, 3), (0, 2));

    assert_eq!(div_rem_euclid(8, 1), (8, 0));
    assert_eq!(div_rem_euclid(8, 2), (4, 0));
    assert_eq!(div_rem_euclid(8, 3), (2, 2));

    assert_eq!(div_rem_euclid(9, 1), (9, 0));
    assert_eq!(div_rem_euclid(9, 2), (4, 1));
    assert_eq!(div_rem_euclid(9, 3), (3, 0));

    assert_eq!(div_rem_euclid(10, 1), (10, 0));
    assert_eq!(div_rem_euclid(10, 2), (5, 0));
    assert_eq!(div_rem_euclid(10, 3), (3, 1));

    assert_eq!(div_rem_euclid(i32::MAX, 1), (2147483647, 0));
    assert_eq!(div_rem_euclid(i32::MAX, 2), (1073741823, 1));
    assert_eq!(div_rem_euclid(i32::MAX, 3), (715827882, 1));
}
