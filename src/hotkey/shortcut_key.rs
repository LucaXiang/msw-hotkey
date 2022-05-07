use std::str::FromStr;

use super::{
    parse_hotkey_error::ParseHotkeyError, special_key::SpecialKey, visible_key::VisibleKey,
};

#[derive(Debug, PartialEq)]
pub enum ShortcutKey {
    VisibleKey(VisibleKey),
    SpecialKey(SpecialKey),
}

impl FromStr for ShortcutKey {
    type Err = ParseHotkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(special_key) = SpecialKey::from_str(s) {
            return Ok(ShortcutKey::SpecialKey(special_key));
        }
        match VisibleKey::from_str(s) {
            Ok(visible_key) => Ok(ShortcutKey::VisibleKey(visible_key)),
            Err(err) => Err(err),
        }
    }
}

impl From<ShortcutKey> for u8 {
    fn from(sk: ShortcutKey) -> Self {
        match sk {
            ShortcutKey::VisibleKey(v) => v.into(),
            ShortcutKey::SpecialKey(s) => s.into(),
        }
    }
}
