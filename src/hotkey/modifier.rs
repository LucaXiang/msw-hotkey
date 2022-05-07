use std::str::FromStr;

use super::parse_hotkey_error::ParseHotkeyError;

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Alt = 1,
    Ctrl = 2,
    Shift = 4,
    Win = 8,
}

impl From<Modifier> for u8 {
    fn from(m: Modifier) -> Self {
        m as u8
    }
}

impl FromStr for Modifier {
    type Err = ParseHotkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_case_s = s.to_ascii_lowercase();
        if lower_case_s.is_empty() {
            return Err(ParseHotkeyError::MissingModifier);
        }
        match lower_case_s.as_str() {
            "ctrl" => Ok(Modifier::Ctrl),
            "alt" => Ok(Modifier::Alt),
            "shift" => Ok(Modifier::Shift),
            "win" => Ok(Modifier::Win),
            _ => Err(ParseHotkeyError::UnexpectedModifier),
        }
    }
}
