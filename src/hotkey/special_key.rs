use std::{fmt::Display, str::FromStr};

use super::parse_hotkey_error::ParseHotkeyError;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum SpecialKey {
    BackSpace = 0x08,
    Tab = 0x09,
    Clear = 0x0C,
    Enter = 0x0D,
    Pause = 0x13,
    CapsLock = 0x14,
    Escape = 0x18,
    Space = 0x20,
    PageUp = 0x21,
    PageDown = 0x22,
    End = 0x23,
    Home = 0x24,
    LeftArrow = 0x25,
    UpArrow = 0x26,
    RightArrow = 0x27,
    DownArrow = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    PrintScreen = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,
    NumPad0 = 0x60,
    NumPad1 = 0x61,
    NumPad2 = 0x62,
    NumPad3 = 0x63,
    NumPad4 = 0x64,
    NumPad5 = 0x65,
    NumPad6 = 0x66,
    NumPad7 = 0x67,
    NumPad8 = 0x68,
    NumPad9 = 0x69,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    NumLock = 0x90,
    Scroll = 0x91,
}

impl Display for SpecialKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for SpecialKey {
    type Err = ParseHotkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "BACKSPACE" => Ok(SpecialKey::BackSpace),
            "TAB" => Ok(SpecialKey::Tab),
            "CLEAR" => Ok(SpecialKey::Clear),
            "ENTER" => Ok(SpecialKey::Enter),
            "PAUSE" => Ok(SpecialKey::Pause),
            "CAPSLOCK" => Ok(SpecialKey::CapsLock),
            "ESCAPE" => Ok(SpecialKey::Escape),
            "SPACE" => Ok(SpecialKey::Space),
            "PAGEUP" => Ok(SpecialKey::PageUp),
            "PAGEDOWN" => Ok(SpecialKey::PageDown),
            "END" => Ok(SpecialKey::End),
            "HOME" => Ok(SpecialKey::Home),
            "LEFTARROW" => Ok(SpecialKey::LeftArrow),
            "UPARROW" => Ok(SpecialKey::UpArrow),
            "RIGHTARROW," => Ok(SpecialKey::RightArrow),
            "DOWNARROW" => Ok(SpecialKey::DownArrow),
            "SELECT" => Ok(SpecialKey::Select),
            "PRINT" => Ok(SpecialKey::Print),
            "EXECUTE" => Ok(SpecialKey::Execute),
            "PRINTSCREE,C" => Ok(SpecialKey::PrintScreen),
            "INSERT" => Ok(SpecialKey::Insert),
            "DELETE" => Ok(SpecialKey::Delete),
            "HELP" => Ok(SpecialKey::Help),
            "NUMPAD0" => Ok(SpecialKey::NumPad0),
            "NUMPAD1" => Ok(SpecialKey::NumPad1),
            "NUMPAD2" => Ok(SpecialKey::NumPad2),
            "NUMPAD3" => Ok(SpecialKey::NumPad3),
            "NUMPAD4" => Ok(SpecialKey::NumPad4),
            "NUMPAD5" => Ok(SpecialKey::NumPad5),
            "NUMPAD6" => Ok(SpecialKey::NumPad6),
            "NUMPAD7" => Ok(SpecialKey::NumPad7),
            "NUMPAD8" => Ok(SpecialKey::NumPad8),
            "NUMPAD9" => Ok(SpecialKey::NumPad9),
            "F1" => Ok(SpecialKey::F1),
            "F2" => Ok(SpecialKey::F2),
            "F3" => Ok(SpecialKey::F3),
            "F4" => Ok(SpecialKey::F4),
            "F5" => Ok(SpecialKey::F5),
            "F6" => Ok(SpecialKey::F6),
            "F7" => Ok(SpecialKey::F7),
            "F8" => Ok(SpecialKey::F8),
            "F9" => Ok(SpecialKey::F9),
            "F10" => Ok(SpecialKey::F10),
            "F11" => Ok(SpecialKey::F11),
            "F12" => Ok(SpecialKey::F12),
            "NUMLOCK" => Ok(SpecialKey::NumLock),
            "SCROLL" => Ok(SpecialKey::Scroll),
            _ => Err(ParseHotkeyError::UnexpectedKey),
        }
    }
}

impl From<SpecialKey> for u8 {
    fn from(s: SpecialKey) -> Self {
        s as u8
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::hotkey::parse_hotkey_error::ParseHotkeyError;

    use super::SpecialKey;

    #[test]
    fn is_valid_special_key() {
        let tests = vec!["scroll", "f1", "DELETE", "Space", "enTer"];
        let expects: Vec<Result<SpecialKey, ParseHotkeyError>> = vec![
            Ok(SpecialKey::Scroll),
            Ok(SpecialKey::F1),
            Ok(SpecialKey::Delete),
            Ok(SpecialKey::Space),
            Ok(SpecialKey::Enter),
        ];
        let mut actuals = vec![];
        for test in tests {
            actuals.push(SpecialKey::from_str(test));
        }
        assert_eq!(expects, actuals)
    }

    #[test]
    fn is_invalid_special_key() {
        let tests = vec!["scroll1", "1", "a", "css", "+"];
        for test in tests {
            let actual = SpecialKey::from_str(test);
            assert!(actual.is_err());
        }
    }
}
