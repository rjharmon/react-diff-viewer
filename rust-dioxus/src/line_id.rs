//! Line ids: which side a line belongs to and its line number, read as `L-20`
//! or `R-3`.

use std::fmt;
use std::str::FromStr;

/// A line on one side of the diff.
///
/// REQT-zaens35zqy (Highlighted lines): its text form joins the side (`L` for
/// old, `R` for new) and the line number with a hyphen, as in `L-20`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LineId {
    /// A line of the old text, by its line number.
    Old(usize),
    /// A line of the new text, by its line number.
    New(usize),
}

impl fmt::Display for LineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineId::Old(number) => write!(f, "L-{number}"),
            LineId::New(number) => write!(f, "R-{number}"),
        }
    }
}

/// Text that does not read as a line id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineIdParseError {
    text: String,
}

impl fmt::Display for LineIdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not a line id such as L-20 or R-3", self.text)
    }
}

impl std::error::Error for LineIdParseError {}

impl FromStr for LineId {
    type Err = LineIdParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parse_error = || LineIdParseError {
            text: text.to_owned(),
        };
        let (side, number) = text.split_once('-').ok_or_else(parse_error)?;
        let number: usize = number.parse().map_err(|_| parse_error())?;
        match side {
            "L" => Ok(LineId::Old(number)),
            "R" => Ok(LineId::New(number)),
            _ => Err(parse_error()),
        }
    }
}
