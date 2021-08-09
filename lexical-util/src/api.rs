//! Implement string conversion routines in a single trait.

// NOTE:
//  We use macros to define the traits, rather than implement here
//  since we can't define traits for types when both are defined outside
//  the current crate, including in workspaces.

// FROM LEXICAL

/// Define FromLexical trait.
#[macro_export]
#[cfg(feature = "parse")]
macro_rules! from_lexical {
    () => {
        /// Trait for numerical types that can be parsed from bytes.
        pub trait FromLexical: lexical_util::num::Number {
            /// Checked parser for a string-to-number conversion.
            ///
            /// This method parses the entire string, returning an error if
            /// any invalid digits are found during parsing. Returns a `Result`
            /// containing either the parsed value, or an error containing
            /// any errors that occurred during parsing.
            ///
            /// * `bytes`   - Slice containing a numeric string.
            fn from_lexical(bytes: &[u8]) -> lexical_util::result::Result<Self>;

            /// Checked parser for a string-to-number conversion.
            ///
            /// This method parses until an invalid digit is found (or the end
            /// of the string), returning the number of processed digits
            /// and the parsed value until that point. Returns a `Result`
            /// containing either the parsed value and the number of processed
            /// digits, or an error containing any errors that occurred during
            /// parsing.
            ///
            /// * `bytes`   - Slice containing a numeric string.
            fn from_lexical_partial(bytes: &[u8]) -> lexical_util::result::Result<(Self, usize)>;
        }
    };
}

/// Define FromLexicalWithOptions trait.
#[macro_export]
#[cfg(feature = "parse")]
macro_rules! from_lexical_with_options {
    () => {
        /// Trait for numerical types that can be parsed from bytes with custom options.
        ///
        /// The `Options` type specifies the configurable options to provide.
        pub trait FromLexicalWithOptions: lexical_util::num::Number {
            /// Custom formatting options for parsing a number.
            type Options: Default;

            /// Checked parser for a string-to-number conversion.
            ///
            /// This method parses the entire string, returning an error if
            /// any invalid digits are found during parsing. The parsing
            /// is dictated by the options, which specifies special
            /// float strings, required float components, digit separators,
            /// exponent characters, and more. Returns a `Result` containing
            /// either the parsed value, or an error containing any errors
            /// that occurred during parsing.
            ///
            /// * `FORMAT`  - Flags and characters designating the number grammar.
            /// * `bytes`   - Slice containing a numeric string.
            /// * `options` - Options to dictate number parsing.
            ///
            /// The `FORMAT` packed struct is built using [`NumberFormatBuilder`].
            /// Any invalid number format will prevent parsing, returning
            /// the appropriate format error. If you are unsure which format
            /// to use, use [`STANDARD`].
            ///
            /// [`NumberFormatBuilder`]: lexical_util::format::NumberFormatBuilder
            /// [`STANDARD`]: lexical_util::format::STANDARD
            fn from_lexical_with_options<const FORMAT: u128>(
                bytes: &[u8],
                options: &Self::Options,
            ) -> lexical_util::result::Result<Self>;

            /// Checked parser for a string-to-number conversion.
            ///
            /// This method parses until an invalid digit is found (or the end
            /// of the string), returning the number of processed digits
            /// and the parsed value until that point. Returns a `Result`
            /// containing either the parsed value and the number of
            /// processed digits, or an error containing any errors that
            /// occurred during parsing.
            ///
            /// * `FORMAT`  - Flags and characters designating the number grammar.
            /// * `bytes`   - Slice containing a numeric string.
            /// * `options` - Options to dictate number parsing.
            ///
            /// The `FORMAT` packed struct is built using [`NumberFormatBuilder`].
            /// Any invalid number format will prevent parsing, returning
            /// the appropriate format error. If you are unsure which format
            /// to use, use [`STANDARD`].
            ///
            /// [`NumberFormatBuilder`]: lexical_util::format::NumberFormatBuilder
            /// [`STANDARD`]: lexical_util::format::STANDARD
            fn from_lexical_partial_with_options<const FORMAT: u128>(
                bytes: &[u8],
                options: &Self::Options,
            ) -> lexical_util::result::Result<(Self, usize)>;
        }
    };
}

// TO LEXICAL

/// Define ToLexical trait.
#[macro_export]
#[cfg(feature = "write")]
macro_rules! to_lexical {
    () => {
        /// Trait for numerical types that can be serialized to bytes.
        ///
        /// To determine the number of bytes required to serialize a value to
        /// string, check the associated constants from a required trait:
        /// - [`FORMATTED_SIZE`]
        /// - [`FORMATTED_SIZE_DECIMAL`]
        ///
        /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
        /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
        pub trait ToLexical:
            lexical_util::constants::FormattedSize + lexical_util::num::Number
        {
            /// Serializer for a number-to-string conversion.
            ///
            /// Returns a subslice of the input buffer containing the written bytes,
            /// starting from the same address in memory as the input slice.
            ///
            /// * `value`   - Number to serialize.
            /// * `bytes`   - Buffer to write number to.
            ///
            /// # Safety
            ///
            /// Safe as long as the caller has provided a buffer of at least
            /// [`FORMATTED_SIZE_DECIMAL`] elements. If a smaller buffer is
            /// provided, a buffer overflow is very likely.
            ///
            /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
            unsafe fn to_lexical_unchecked<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8];

            /// Serializer for a number-to-string conversion.
            ///
            /// Returns a subslice of the input buffer containing the written bytes,
            /// starting from the same address in memory as the input slice.
            ///
            /// * `value`   - Number to serialize.
            /// * `bytes`   - Buffer to write number to.
            ///
            /// # Panics
            ///
            /// Panics if the buffer is not of sufficient size. The caller
            /// must provide a slice of sufficient size. In order to ensure
            /// the function will not panic, ensure the buffer has at least
            /// [`FORMATTED_SIZE_DECIMAL`] elements.
            ///
            /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
            fn to_lexical<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8];
        }
    };
}

/// Define ToLexicalWithOptions trait.
#[macro_export]
#[cfg(feature = "write")]
macro_rules! to_lexical_with_options {
    () => {
        /// Trait for numerical types that can be serialized to bytes with custom options.
        ///
        /// To determine the number of bytes required to serialize a value to
        /// string, check the associated constants from a required trait:
        /// - [`FORMATTED_SIZE`]
        /// - [`FORMATTED_SIZE_DECIMAL`]
        ///
        /// The `Options` type specifies the configurable options to provide.
        ///
        /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
        /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
        pub trait ToLexicalWithOptions:
            lexical_util::constants::FormattedSize + lexical_util::num::Number
        {
            /// Custom formatting options for writing a number.
            type Options: Default;

            /// Serializer for a number-to-string conversion.
            ///
            /// Returns a subslice of the input buffer containing the written bytes,
            /// starting from the same address in memory as the input slice.
            ///
            /// * `FORMAT`  - Flags and characters designating the number grammar.
            /// * `value`   - Number to serialize.
            /// * `bytes`   - Buffer to write number to.
            /// * `options` - Options for number formatting.
            ///
            /// # Safety
            ///
            /// Safe as long as the caller has provided a buffer of at least
            /// [`FORMATTED_SIZE`] elements. If a smaller buffer is
            /// provided, a buffer overflow is very likely.
            ///
            /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
            unsafe fn to_lexical_with_options_unchecked<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8];

            /// Serializer for a number-to-string conversion.
            ///
            /// Returns a subslice of the input buffer containing the written bytes,
            /// starting from the same address in memory as the input slice.
            ///
            /// * `FORMAT`  - Flags and characters designating the number grammar.
            /// * `value`   - Number to serialize.
            /// * `bytes`   - Buffer to write number to.
            /// * `options` - Options for number formatting.
            ///
            /// # Panics
            ///
            /// Panics if the buffer is not of sufficient size. The caller
            /// must provide a slice of sufficient size. In order to ensure
            /// the function will not panic, ensure the buffer has at least
            /// [`FORMATTED_SIZE`] elements.
            ///
            /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
            fn to_lexical_with_options<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8];
        }
    };
}
