# MSW-HOTKEY

A hotkey library for Microsoft Windows

# How to use

```rust
use msw-hotkey::Hotkey
// https://github.com/microsoft/windows-rs
// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/fn.RegisterHotKey.html
use windows::Win32::UI::Input::KeyboardAndMouse::RegisterHotKey
// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/struct.HOT_KEY_MODIFIERS.html
use windows::Win32::UI::Input::KeyboardAndMouse::HOT_KEY_MODIFIERS;


fn main()
{
  let hotkey = Hotkey::from_str("ctrl + a");
  unsafe {
    RegisterHotkey(Win32::Foundation::HWND(0) , 1 ,HOT_KEY_MODIFIERS(hotkey.modifier()), hotkey.key() as u32);
  }

}
```
