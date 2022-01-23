/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::ops::{BitAnd, BitOr, Not, Range, Shl, Shr};

// I thought about using the bit_field crate, but less dependencies are better, plus that crate
// has asserts (not even debug_asserts) in the functions that would really bloat and slow the code.

pub(crate) trait BitField {
    fn get_bit(&self, n: u8) -> bool;

    fn get_bits(&self, range: Range<u8>) -> Self;

    fn set_bit(&self, n: u8, value: bool) -> Self;

    fn set_bits(&self, range: Range<u8>, value: Self) -> Self;
}

impl<T> BitField for T
where
    T: Copy
        + Eq
        + BitAnd<T, Output = T>
        + BitOr<T, Output = T>
        + Not<Output = T>
        + Shl<T, Output = T>
        + Shr<T, Output = T>
        + From<bool>
        + From<u8>,
{
    fn get_bit(&self, n: u8) -> bool {
        let n: Self = n.into();
        let one: Self = 1.into();
        if (*self >> n) & one == one { true } else { false }
    }

    fn get_bits(&self, range: Range<u8>) -> Self {
        let mask: Self = ((1 << range.end - range.start) - 1).into();
        (*self >> range.start.into()) & mask
    }

    #[must_use]
    fn set_bit(&self, n: u8, value: bool) -> Self {
        let value: Self = value.into();
        let n: Self = n.into();
        let one: Self = 1.into();
        (*self & !(one << n)) | value << n
    }

    #[must_use]
    fn set_bits(&self, range: Range<u8>, value: Self) -> Self {
        let mask: Self = ((1 << range.end - range.start) - 1).into();
        (*self & !mask) | (value << range.start.into())
    }
}

#[test]
fn test_bit_field() {
    let x: u16 = 0x3964;

    assert_eq!(x.get_bit(2), true);
    assert_eq!(x.get_bit(3), false);

    assert_eq!(x.get_bits(4..10), 0b010110);

    assert_eq!(x.set_bit(3, true), 0x396C);
    assert_eq!(x.set_bit(3, false), 0x3964);
    assert_eq!(x.set_bit(12, false), 0x2964);

    assert_eq!(x.set_bits(1..4, 0b101), 0x396A);
    assert_eq!(x.set_bits(7..9, 0b11), 0x39E4);
}
