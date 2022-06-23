//! The `fmt` module contains [`SizeFormatter`] and other types pertaining to formatting a size as
//! human-readable text.

use core::fmt;
use super::*;

/// An enumeration of supported bases to use for generating textual descriptions of sizes.
///
/// [`Base::Base10`] is the "usual" units like "kilobyte" and "exabyte", while [`Base::Base2`] is
/// the SI/memory units like "mebibyte" and "tebibyte", (more often referred to as "MiB" and "TiB",
/// respectively).
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Base {
    /// Base-2 units like "kibibyte" and "mebibyte", more often referred to via their abbreviations
    /// ("KiB" and "MiB", respectively). Each unit is 1024 times greater than the preceding one.
    Base2,
    /// Base-10 units like "kilobyte" and "megabyte". Each unit is 1000 times greater than the
    /// preceding one.
    Base10,
}

/// A collection of units used to refer to sizes, for all supported bases.
enum Unit {
    /// The basic "byte" unit, used by both base-2 and base-10 styles.
    Byte,
    /// The base-2 "kibibyte" unit, equal to 1024 bytes.
    Kibibyte,
    /// The base-10 "kilobyte" unit, equal to 1000 bytes.
    Kilobyte,
    /// The base-2 "mebibyte" unit, equal to 1024 kibibytes.
    Mebibyte,
    /// The base-10 "megabyte" unit, equal to 1000 kilobytes.
    Megabyte,
    /// The base-2 "gibibyte" unit, equal to 1024 mebibytes.
    Gibibyte,
    /// The base-10 "gigabyte" unit, equal to 1000 megabytes.
    Gigabyte,
    /// The base-2 "tebibyte" unit, equal to 1024 gibibytes.
    Tebibyte,
    /// The base-10 "terabyte" unit, equal to 1000 gigabytes.
    Terabyte,
    /// The base-2 "pebibyte" unit, equal to 1024 tebibytes.
    Pebibyte,
    /// The base-10 "petabyte" unit, equal to 1000 terabytes.
    Petabyte,
    /// The base-2 "exbibyte" unit, equal to 1024 pebibytes.
    Exbibyte,
    /// The base-10 "exabyte" unit, equal to 1000 petabytes.
    Exabyte,
}

impl Unit {
    const fn text(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        use self::Unit::*;

        match &self {
            &Byte => ("byte", "Byte", "b", "B"),

            &Kilobyte => ("kilobyte", "Kilobyte", "kb", "KB"),
            &Megabyte => ("megabyte", "Megabyte", "mb", "MB"),
            &Gigabyte => ("gigabyte", "Gigabyte", "gb", "GB"),
            &Terabyte => ("terabyte", "Terabyte", "tb", "TB"),
            &Petabyte => ("petabyte", "Petabyte", "pb", "PB"),
            &Exabyte  => ("exabyte",  "Exabyte",  "eb", "EB"),

            &Kibibyte => ("kibibyte", "Kibibyte", "kib", "KiB"),
            &Mebibyte => ("mebibyte", "Mebibyte", "mib", "MiB"),
            &Gibibyte => ("gibibyte", "Gibibyte", "gib", "GiB"),
            &Pebibyte => ("pebibyte", "Pebibyte", "pib", "PiB"),
            &Tebibyte => ("tebibyte", "Tebibyte", "tib", "TiB"),
            &Exbibyte => ("exbibyte", "Exbibyte", "eib", "EiB"),
        }
    }

    fn format(&self, mut fmt: &mut fmt::Formatter, bytes: u64, style: &Style) -> fmt::Result {
        match (&style, bytes) {
            (&Style::Default, _) => match &self {
                &Unit::Byte => self.format(&mut fmt, bytes, &Style::FullLowercase),
                _ => self.format(&mut fmt, bytes, &Style::Abbreviated),
            },

            (&Style::FullLowercase, 1) => write!(fmt, " {}", self.text().0),
            (&Style::Full, 1) => write!(fmt, " {}", self.text().1),
            (&Style::AbbreviatedLowercase, 1) => write!(fmt, " {}", self.text().2),
            (&Style::Abbreviated, 1) => write!(fmt, " {}", self.text().3),

            (&Style::FullLowercase, _) => write!(fmt, " {}s", self.text().0),
            (&Style::Full, _) => write!(fmt, " {}s", self.text().1),
            (&Style::AbbreviatedLowercase, _) => write!(fmt, " {}", self.text().2),
            (&Style::Abbreviated, _) => write!(fmt, " {}", self.text().3),
        }
    }
}

/// An enumeration of supported styles to be used when formatting/printing a [`Size`] type,
/// specifying how the unit should be spelled out.
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Style {
    /// The default "smart" style, currently equal to [`Style::FullLowercase`] when the final unit is
    /// in bytes or [`Style::Abbreviated`] otherwise, e.g. "1024 bytes" and "1.29 GiB"
    Default,
    /// Abbreviated style, e.g. "1024 KB" and "1.29 GiB"
    Abbreviated,
    /// Abbreviated, lowercase style, e.g. "1024 kb" and "1.29 gib"
    AbbreviatedLowercase,
    /// Full unit name style, e.g. "1024 Kilobytes" and "1.29 Gibibytes"
    Full,
    /// Full, lowercase unit name style, e.g. "1024 kilobytes" and "1.29 gibibytes"
    FullLowercase,
}

// Backwards-compatibility associated constants to mimic `Style` variants to enable compilation of
// older code. They are all hidden from the docs.
impl Style {
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.3", note = "Use Style::Default instead")]
    /// A backwards-compatible alias for [`Style::Default`]
    pub const Smart: Style = Style::Default;

    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.3", note = "Use Style::AbbreviatedLowercase instead")]
    /// A backwards-compatible alias for [`Style::AbbreviatedLowercase`]
    pub const AbbreviatedLowerCase: Style = Style::AbbreviatedLowercase;

    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[deprecated(since = "0.3", note = "Use Style::FullLowercase instead")]
    /// A backwards-compatible alias for [`Style::FullLowercase`]
    pub const FullLowerCase: Style = Style::FullLowercase;
}

impl std::fmt::Display for Size {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.to_string())
    }
}

/// A struct that can be used to achieve greater control over how a [`Size`] is formatted as
/// human-readable text, created by calling [`Size::format()`]. The `SizeFormatter` follows the
/// builder model and exposes a chaining API for configuration.
///
/// After configuration, a `SizeFormatter` may be written directly (via `write!()`, `format!()`,
/// `println!()`, etc.) because it implements [`Display`](std::fmt::Display), or
/// [`SizeFormatter::to_string()`] can be used to retrieve a `String` containing the formatted size.
pub struct SizeFormatter<'a> {
    size: &'a Size,
    base: Base,
    style: Style,
}

impl<'a> SizeFormatter<'a> {
    /// Specify the base of the units to be used when generating the textual description of the
    /// `Size`.
    ///
    /// This lets users choose between "standard" base-10 units like "KB" and "MB" or the improved
    /// SI base-2 units like "KiB" and "MiB". See [`Base`] for more information.
    pub fn with_base(self, base: Base) -> Self {
        Self {
            base,
            .. self
        }
    }

    /// Specify the style used to write the accompanying unit for a formatted file size.
    ///
    /// See [`Style`] for more information.
    pub fn with_style(self, style: Style) -> Self {
        Self {
            style,
            .. self
        }
    }

    /// Returns the formatted `Size` as a `String`, formatted according to the current state of the
    /// `SizeFormatter` instance as modified via [`with_style()`](Self::with_style),
    /// [`with_base()`](Self::with_base), and co.
    pub fn to_string(&self) -> String {
        format!("{}", &self)
    }
}

impl<'a> std::fmt::Display for SizeFormatter<'a> {
    fn fmt(&self, mut fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = match self.size.bytes() {
            x@ 0.. => x as u64,
            y => {
                write!(fmt, "-")?;

                // The absolute magnitude of T::min_value() for a signed number is one more than
                // that of T::max_value(), meaning T::min_value().abs() will panic.
                match y.checked_abs() {
                    Some(abs) => abs as u64,
                    None => i64::max_value() as u64,
                }
            }
        };

        let rule = match self.base {
            Base::Base2 => match BASE2_RULES.binary_search_by_key(&bytes, |rule| rule.less_than) {
                Ok(index) => &BASE2_RULES[index + 1],
                Err(index) => &BASE2_RULES[index],
            },
            Base::Base10 => {
                match BASE10_RULES.binary_search_by_key(&bytes, |rule| rule.less_than) {
                    Ok(index) => &BASE10_RULES[index + 1],
                    Err(index) => &BASE10_RULES[index],
                }
            }
        };

        (rule.formatter)(&mut fmt, bytes)?;
        rule.unit.format(&mut fmt, bytes, &self.style)?;

        return Ok(());
    }
}

impl Size {
    /// Returns a textual representation of the [`Size`] for display purposes. This is a `String`
    /// equivalent to what `Size`'s `std::fmt::Display` would return.
    pub fn to_string(&self) -> String {
        return format!("{}", self.format());
    }

    /// Returns a textual representation of the [`Size`] for display purposes, giving control over
    /// the returned representation's base (see [`Base::Base2`] and [`Base::Base10`]) and the style
    /// used to express the determined unit (see [`Style`]).
    pub fn format(& self) -> SizeFormatter<'_> {
        return SizeFormatter {
            size: &self,
            base: DEFAULT_BASE,
            style: DEFAULT_STYLE,
        }
    }
}

struct FormatRule {
    less_than: u64,
    formatter: fn(&mut fmt::Formatter, bytes: u64) -> fmt::Result,
    unit: Unit,
}

const BASE10_RULES: [FormatRule; 17] = [
    FormatRule {
        less_than: 1 * KILOBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Unit::Byte,
    },
    FormatRule {
        less_than: 10 * KILOBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Unit::Kilobyte,
    },
    FormatRule {
        less_than: 100 * KILOBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Unit::Kilobyte,
    },
    FormatRule {
        less_than: 1 * MEGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * KILOBYTE) as f64)),
        unit: Unit::Kilobyte,
    },
    FormatRule {
        less_than: 10 * MEGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Unit::Megabyte,
    },
    FormatRule {
        less_than: 100 * MEGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Unit::Megabyte,
    },
    FormatRule {
        less_than: 1 * GIGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * MEGABYTE) as f64)),
        unit: Unit::Megabyte,
    },
    FormatRule {
        less_than: 10 * GIGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Unit::Gigabyte,
    },
    FormatRule {
        less_than: 100 * GIGABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Unit::Gigabyte,
    },
    FormatRule {
        less_than: 1 * TERABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * GIGABYTE) as f64)),
        unit: Unit::Gigabyte,
    },
    FormatRule {
        less_than: 10 * TERABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Unit::Terabyte,
    },
    FormatRule {
        less_than: 100 * TERABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Unit::Terabyte,
    },
    FormatRule {
        less_than: 1 * PETABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * TERABYTE) as f64)),
        unit: Unit::Terabyte,
    },
    FormatRule {
        less_than: 10 * PETABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Unit::Petabyte,
    },
    FormatRule {
        less_than: 100 * PETABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Unit::Petabyte,
    },
    FormatRule {
        less_than: 1 * EXABYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * PETABYTE) as f64)),
        unit: Unit::Petabyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1i64 * EXABYTE) as f64)),
        unit: Unit::Exabyte,
    },
];

const BASE2_RULES: [FormatRule; 17] = [
    FormatRule {
        less_than: 1 * KIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes),
        unit: Unit::Byte,
    },
    FormatRule {
        less_than: 10 * KIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Unit::Kibibyte,
    },
    FormatRule {
        less_than: 100 * KIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Unit::Kibibyte,
    },
    FormatRule {
        less_than: 1 * MEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * KIBIBYTE) as f64)),
        unit: Unit::Kibibyte,
    },
    FormatRule {
        less_than: 10 * MEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Unit::Mebibyte,
    },
    FormatRule {
        less_than: 100 * MEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Unit::Mebibyte,
    },
    FormatRule {
        less_than: 1 * GIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * MEBIBYTE) as f64)),
        unit: Unit::Mebibyte,
    },
    FormatRule {
        less_than: 10 * GIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Unit::Gibibyte,
    },
    FormatRule {
        less_than: 100 * GIBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Unit::Gibibyte,
    },
    FormatRule {
        less_than: 1 * TEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * GIBIBYTE) as f64)),
        unit: Unit::Gibibyte,
    },
    FormatRule {
        less_than: 10 * TEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Unit::Tebibyte,
    },
    FormatRule {
        less_than: 100 * TEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Unit::Tebibyte,
    },
    FormatRule {
        less_than: 1 * PEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * TEBIBYTE) as f64)),
        unit: Unit::Tebibyte,
    },
    FormatRule {
        less_than: 10 * PEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.2}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Unit::Pebibyte,
    },
    FormatRule {
        less_than: 100 * PEBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.1}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Unit::Pebibyte,
    },
    FormatRule {
        less_than: 1 * EXBIBYTE as u64,
        formatter: |fmt, bytes| write!(fmt, "{:.0}", bytes as f64 / ((1i64 * PEBIBYTE) as f64)),
        unit: Unit::Pebibyte,
    },
    FormatRule {
        less_than: u64::max_value(),
        formatter: |fmt, bytes| write!(fmt, "{:0}", bytes as f64 / ((1i64 * EXBIBYTE) as f64)),
        unit: Unit::Exbibyte,
    },
];
