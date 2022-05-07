use std::str::FromStr;

use super::parse_hotkey_error::ParseHotkeyError;
#[derive(Debug, PartialEq)]
pub struct VisibleKey(u8);

impl VisibleKey {
    pub fn is_valid(value: char) -> bool {
        const VALID_ASTERISM: &str = "*+-.;=,-./`[]\\";
        let mut result = false;
        loop {
            if value.is_ascii_alphanumeric() {
                result = true;
                break;
            }
            if VALID_ASTERISM.find(value).is_some() {
                result = true;
                break;
            }
            break;
        }
        result
    }
}

impl FromStr for VisibleKey {
    type Err = ParseHotkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err;
        loop {
            if s.is_empty() {
                err = ParseHotkeyError::MissingKey;
                break;
            }

            let c = s.chars().next().unwrap();

            if s.len() != 1 || !VisibleKey::is_valid(c) {
                err = ParseHotkeyError::UnexpectedKey;
                break;
            }

            return Ok(VisibleKey(c as u8));
        }
        Err(err)
    }
}

impl From<VisibleKey> for u8 {
    fn from(v: VisibleKey) -> Self {
        v.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::VisibleKey;

    #[test]
    fn is_valid_visible_char() {
        let tests = ["a", "b", "c", "1", "2", "3", "`", "\\", ".", "["];
        for test in tests.iter() {
            let actual = VisibleKey::from_str(test);
            assert!(actual.is_ok(), "{}", test);
        }
    }

    #[test]
    fn is_invalid_visible_char() {
        let tests = ["!", "@", "aa", "", " ", "?", ">", "delete"];
        for test in tests.iter() {
            let actual = VisibleKey::from_str(test);
            assert!(actual.is_err(), "{}", test);
        }
    }
}
