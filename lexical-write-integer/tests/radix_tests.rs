#![cfg(not(feature = "compact"))]
#![cfg(feature = "power-of-two")]

mod util;

use core::num::ParseIntError;
#[cfg(feature = "radix")]
use core::str::from_utf8_unchecked;

use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::write::WriteInteger;

use crate::util::from_radix;

pub trait FromRadix: Sized {
    fn from_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
}

macro_rules! impl_from_radix {
    ($($t:ty)*) => ($(impl FromRadix for $t {
        #[inline]
        fn from_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
            <$t>::from_str_radix(src, radix)
        }
    })*)
}

impl_from_radix! { u32 u64 u128 }

#[test]
#[cfg(feature = "radix")]
fn u128toa_test() {
    const FORMAT: u128 = from_radix(12);
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let value = 136551478823710021067381144334863695872u128;
    let count = value.write_mantissa::<FORMAT>(&mut buffer);
    unsafe {
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), 12);
        assert_eq!(y, Ok(value));
    }
}

#[cfg(feature = "power-of-two")]
fn write_integer<T: WriteInteger, const FORMAT: u128>(x: T, actual: &[u8]) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let count = x.write_mantissa::<FORMAT>(&mut buffer);
    assert_eq!(actual.len(), count);
    assert_eq!(actual, &buffer[..count])
}

#[test]
#[cfg(feature = "power-of-two")]
fn binary_test() {
    // Binary
    const BINARY: u128 = from_radix(2);
    write_integer::<_, BINARY>(0u128, b"0");
    write_integer::<_, BINARY>(1u128, b"1");
    write_integer::<_, BINARY>(5u128, b"101");
    write_integer::<_, BINARY>(170141183460469231731687303715884105727u128, b"1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");

    // Hexadecimal
    const HEX: u128 = from_radix(16);
    write_integer::<_, HEX>(
        170141183460469231731687303715884105727u128,
        b"7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
    );
}

#[test]
#[cfg(feature = "radix")]
fn radix_test() {
    write_integer::<u32, { from_radix(2) }>(37u32, b"100101");
    write_integer::<u32, { from_radix(3) }>(37u32, b"1101");
    write_integer::<u32, { from_radix(4) }>(37u32, b"211");
    write_integer::<u32, { from_radix(5) }>(37u32, b"122");
    write_integer::<u32, { from_radix(6) }>(37u32, b"101");
    write_integer::<u32, { from_radix(7) }>(37u32, b"52");
    write_integer::<u32, { from_radix(8) }>(37u32, b"45");
    write_integer::<u32, { from_radix(9) }>(37u32, b"41");
    write_integer::<u32, { from_radix(10) }>(37u32, b"37");
    write_integer::<u32, { from_radix(11) }>(37u32, b"34");
    write_integer::<u32, { from_radix(12) }>(37u32, b"31");
    write_integer::<u32, { from_radix(13) }>(37u32, b"2B");
    write_integer::<u32, { from_radix(14) }>(37u32, b"29");
    write_integer::<u32, { from_radix(15) }>(37u32, b"27");
    write_integer::<u32, { from_radix(16) }>(37u32, b"25");
    write_integer::<u32, { from_radix(17) }>(37u32, b"23");
    write_integer::<u32, { from_radix(18) }>(37u32, b"21");
    write_integer::<u32, { from_radix(19) }>(37u32, b"1I");
    write_integer::<u32, { from_radix(20) }>(37u32, b"1H");
    write_integer::<u32, { from_radix(21) }>(37u32, b"1G");
    write_integer::<u32, { from_radix(22) }>(37u32, b"1F");
    write_integer::<u32, { from_radix(23) }>(37u32, b"1E");
    write_integer::<u32, { from_radix(24) }>(37u32, b"1D");
    write_integer::<u32, { from_radix(25) }>(37u32, b"1C");
    write_integer::<u32, { from_radix(26) }>(37u32, b"1B");
    write_integer::<u32, { from_radix(27) }>(37u32, b"1A");
    write_integer::<u32, { from_radix(28) }>(37u32, b"19");
    write_integer::<u32, { from_radix(29) }>(37u32, b"18");
    write_integer::<u32, { from_radix(30) }>(37u32, b"17");
    write_integer::<u32, { from_radix(31) }>(37u32, b"16");
    write_integer::<u32, { from_radix(32) }>(37u32, b"15");
    write_integer::<u32, { from_radix(33) }>(37u32, b"14");
    write_integer::<u32, { from_radix(34) }>(37u32, b"13");
    write_integer::<u32, { from_radix(35) }>(37u32, b"12");
    write_integer::<u32, { from_radix(36) }>(37u32, b"11");
}

#[test]
#[cfg(feature = "radix")]
fn issue_169_tests() {
    let value = 213850084767170003246100602438595641344u128;
    write_integer::<u128, { from_radix(5) }>(
        value,
        b"3411233210434101044040414300210231141130323220441010334",
    );
}
