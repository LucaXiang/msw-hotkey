use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ParseHotkeyError {
    KeyNotEnough,
    MissingModifier,
    DuplicateModifier,
    MissingKey,
    TooManyKey,
    UnexpectedModifier,
    UnexpectedKey,
}

impl Display for ParseHotkeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseHotkeyError::MissingModifier => "missing modifier",
                ParseHotkeyError::DuplicateModifier => "duplicate modifier",
                ParseHotkeyError::MissingKey => "missing key",
                ParseHotkeyError::TooManyKey => "to many key",
                ParseHotkeyError::UnexpectedKey => "unexpected key",
                ParseHotkeyError::UnexpectedModifier => "unexpected modifier",
                ParseHotkeyError::KeyNotEnough => "key not enough",
            }
        )
    }
}
