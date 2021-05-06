//! Wrapper around David Tolnay's ryu.

use crate::util::*;
use ryu::raw;

// F32

/// Wrapper for ryu.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline]
pub(crate) fn float_decimal<'a>(f: f32, bytes: &'a mut [u8], _: NumberFormat) -> usize {
    unsafe { raw::format32(f, bytes.as_mut_ptr()) }
}

// F64

/// Wrapper for ryu.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline]
pub(crate) fn double_decimal<'a>(d: f64, bytes: &'a mut [u8], _: NumberFormat) -> usize {
    unsafe { raw::format64(d, bytes.as_mut_ptr()) }
}
