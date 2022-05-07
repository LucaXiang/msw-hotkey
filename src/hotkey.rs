use std::str::FromStr;

mod modifier;
mod parse_hotkey_error;
mod shortcut_key;
mod special_key;
mod visible_key;

pub use self::modifier::Modifier;
pub use self::parse_hotkey_error::ParseHotkeyError;
pub use self::shortcut_key::ShortcutKey;
pub use self::special_key::SpecialKey;
pub use self::visible_key::VisibleKey;

/// parse hotkey from &str vector \
/// # Accept syntax like
/// 1. modifier + key (ctrl + a),(ctrl + delete)
/// 2. modifier + modifier + key (ctrl + shift + a) (ctrl + shift + delete)
/// # Example
/// ```
/// let syntax1 = vec!["ctrl","a"];
/// let syntax2 = vec!["ctrl","shift","b"];
/// assert!(parse_hotkey(&syntax1).is_ok());
/// assert!(parse_hotkey(&syntax2).is_ok());
/// ```
pub fn parse_hotkey(shortcut_keys: &[&str]) -> Result<Hotkey, ParseHotkeyError> {
    let mut shortcut_keys_iter = shortcut_keys.iter().peekable();
    let mut hotkey_modifier = 0_u8;
    let mut hotkey_key = 0_u8;
    while let Some(&shortcut_key) = shortcut_keys_iter.next() {
        // if iter is on last element
        if shortcut_keys_iter.peek() == None {
            let key = ShortcutKey::from_str(shortcut_key)?.into();
            hotkey_key = key;
        } else {
            let modifier: u8 = Modifier::from_str(shortcut_key)?.into();
            if hotkey_modifier & modifier != 0 {
                return Err(ParseHotkeyError::DuplicateModifier);
            }
            hotkey_modifier |= modifier;
        }
    }
    Ok(Hotkey {
        modifier: hotkey_modifier,
        key: hotkey_key,
    })
}

#[derive(Debug, PartialEq)]
pub struct Hotkey {
    modifier: u8,
    key: u8,
}

impl Hotkey {
    pub fn modifier(&self) -> u8 {
        self.modifier
    }
    pub fn key(&self) -> u8 {
        self.key
    }
}

impl FromStr for Hotkey {
    type Err = ParseHotkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let part_of_s: Vec<&str> = s.split('+').map(|s| s.trim()).collect();
        match part_of_s.len() {
            2 | 3 => parse_hotkey(&part_of_s),
            0 | 1 => Err(ParseHotkeyError::KeyNotEnough),
            _ => Err(ParseHotkeyError::TooManyKey),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    #[test]
    fn parse_from_str_ok() {
        let actual = Hotkey::from_str("ctrl + a");
        let expect: Result<Hotkey, ParseHotkeyError> = Ok(Hotkey {
            key: 'a' as u8,
            modifier: Modifier::Ctrl as u8,
        });
        assert_eq!(expect, actual);

        let actual = Hotkey::from_str("ctrl + shift + a");
        let expect: Result<Hotkey, ParseHotkeyError> = Ok(Hotkey {
            key: 'a' as u8,
            modifier: (Modifier::Ctrl as u8 | Modifier::Shift as u8),
        });
        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_modifier_ok() {
        let test = "ctrl";
        let expect = Ok(Modifier::Ctrl);
        let actual = Modifier::from_str(test);
        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_modifier_fail() {
        let test = "abc";
        let expect = Err(ParseHotkeyError::UnexpectedModifier);
        let actual = Modifier::from_str(test);
        assert_eq!(expect, actual);
    }

    #[test]
    fn hotket_from_str_key_not_enough() {
        let actual = Hotkey::from_str("ctrl");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::KeyNotEnough);
        assert_eq!(expect, actual);

        let actual = Hotkey::from_str("");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::KeyNotEnough);
        assert_eq!(expect, actual);
    }

    #[test]
    fn hotket_from_str_key_too_many_key() {
        let actual = Hotkey::from_str("ctrl+shift+a+b+c");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::TooManyKey);
        assert_eq!(expect, actual);

        let actual = Hotkey::from_str("a+b+c+d");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::TooManyKey);
        assert_eq!(expect, actual);
    }

    #[test]
    fn hotket_from_str_key_missing_key() {
        let actual = Hotkey::from_str("ctrl+shift+");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::MissingKey);
        assert_eq!(expect, actual);

        let actual = Hotkey::from_str("ctrl+");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::MissingKey);
        assert_eq!(expect, actual);
    }

    #[test]
    fn hotket_from_str_key_missing_modifier() {
        let actual = Hotkey::from_str("ctrl+ +a");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::MissingModifier);
        assert_eq!(expect, actual);

        let actual = Hotkey::from_str(" +a");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::MissingModifier);
        assert_eq!(expect, actual);
    }

    #[test]
    fn hotket_from_str_key_unexpected_modifier() {
        let actual = Hotkey::from_str("ctl+a");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::UnexpectedModifier);
        assert_eq!(expect, actual);

        let actual = Hotkey::from_str("ctrl+sheft+a");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::UnexpectedModifier);
        assert_eq!(expect, actual);
    }

    #[test]
    fn hotket_from_str_key_unexpected_key() {
        let actual = Hotkey::from_str("ctrl+shift");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::UnexpectedKey);
        assert_eq!(expect, actual);

        let actual = Hotkey::from_str("ctrl+shift+bb");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::UnexpectedKey);
        assert_eq!(expect, actual);
    }

    #[test]
    fn hotket_from_str_key_duplicate_modifier() {
        let actual = Hotkey::from_str("ctrl+ctrl+a");
        let expect: Result<Hotkey, ParseHotkeyError> = Err(ParseHotkeyError::DuplicateModifier);
        assert_eq!(expect, actual);
    }
}
