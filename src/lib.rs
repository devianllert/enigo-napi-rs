#![deny(clippy::all)]

use enigo::{
  Axis::{Horizontal, Vertical},
  Button,
  Coordinate::{Abs, Rel},
  Direction::{Click, Press, Release},
  Enigo, Key, Keyboard, Mouse, Settings,
};
use napi_derive::napi;
use std::cell::RefCell;
#[cfg(target_os = "windows")]
use std::sync::Once;

thread_local! {
  static ENIGO: RefCell<Option<Enigo>> = const { RefCell::new(None) };
}

#[cfg(target_os = "windows")]
static SET_DPI_AWARENESS: Once = Once::new();

enum ParsedKey {
  Key(Key),
  #[cfg(target_os = "macos")]
  Raw(u16),
}

#[cfg(target_os = "windows")]
fn init_dpi_awareness() {
  SET_DPI_AWARENESS.call_once(|| {
    if enigo::set_dpi_awareness().is_err() {
      eprintln!("Failed to set DPI awareness (non-fatal)");
    }
  });
}

fn with_enigo(mut f: impl FnMut(&mut Enigo)) {
  ENIGO.with(|cell| {
    let mut enigo = cell.borrow_mut();
    if enigo.is_none() {
      #[cfg(target_os = "windows")]
      init_dpi_awareness();

      match Enigo::new(&Settings::default()) {
        Ok(instance) => *enigo = Some(instance),
        Err(e) => {
          eprintln!("Failed to create Enigo: {:?}", e);
          return;
        }
      }
    }

    if let Some(instance) = enigo.as_mut() {
      f(instance);
    }
  });
}

#[cfg(target_os = "windows")]
fn parse_letter_key(c: char) -> Option<Key> {
  match c.to_ascii_uppercase() {
    'A' => Some(Key::A),
    'B' => Some(Key::B),
    'C' => Some(Key::C),
    'D' => Some(Key::D),
    'E' => Some(Key::E),
    'F' => Some(Key::F),
    'G' => Some(Key::G),
    'H' => Some(Key::H),
    'I' => Some(Key::I),
    'J' => Some(Key::J),
    'K' => Some(Key::K),
    'L' => Some(Key::L),
    'M' => Some(Key::M),
    'N' => Some(Key::N),
    'O' => Some(Key::O),
    'P' => Some(Key::P),
    'Q' => Some(Key::Q),
    'R' => Some(Key::R),
    'S' => Some(Key::S),
    'T' => Some(Key::T),
    'U' => Some(Key::U),
    'V' => Some(Key::V),
    'W' => Some(Key::W),
    'X' => Some(Key::X),
    'Y' => Some(Key::Y),
    'Z' => Some(Key::Z),
    _ => None,
  }
}

#[cfg(target_os = "windows")]
fn parse_digit_key(c: char) -> Option<Key> {
  match c {
    '0' => Some(Key::Num0),
    '1' => Some(Key::Num1),
    '2' => Some(Key::Num2),
    '3' => Some(Key::Num3),
    '4' => Some(Key::Num4),
    '5' => Some(Key::Num5),
    '6' => Some(Key::Num6),
    '7' => Some(Key::Num7),
    '8' => Some(Key::Num8),
    '9' => Some(Key::Num9),
    _ => None,
  }
}

/// US QWERTY physical key positions — layout-independent on macOS.
#[cfg(target_os = "macos")]
fn macos_physical_keycode(c: char) -> Option<u16> {
  match c.to_ascii_lowercase() {
    'a' => Some(0x00),
    's' => Some(0x01),
    'd' => Some(0x02),
    'f' => Some(0x03),
    'h' => Some(0x04),
    'g' => Some(0x05),
    'z' => Some(0x06),
    'x' => Some(0x07),
    'c' => Some(0x08),
    'v' => Some(0x09),
    'b' => Some(0x0B),
    'q' => Some(0x0C),
    'w' => Some(0x0D),
    'e' => Some(0x0E),
    'r' => Some(0x0F),
    'y' => Some(0x10),
    't' => Some(0x11),
    '1' => Some(0x12),
    '2' => Some(0x13),
    '3' => Some(0x14),
    '4' => Some(0x15),
    '6' => Some(0x16),
    '5' => Some(0x17),
    '9' => Some(0x19),
    '7' => Some(0x1A),
    '8' => Some(0x1C),
    '0' => Some(0x1D),
    'o' => Some(0x1F),
    'u' => Some(0x20),
    'i' => Some(0x22),
    'p' => Some(0x23),
    'l' => Some(0x25),
    'j' => Some(0x26),
    'k' => Some(0x28),
    'n' => Some(0x2D),
    'm' => Some(0x2E),
    _ => None,
  }
}

fn parse_function_key(name: &str) -> Option<Key> {
  let lower = name.to_lowercase();
  if lower.len() < 2 || !lower.starts_with('f') {
    return None;
  }

  match lower[1..].parse::<u8>() {
    Ok(1) => Some(Key::F1),
    Ok(2) => Some(Key::F2),
    Ok(3) => Some(Key::F3),
    Ok(4) => Some(Key::F4),
    Ok(5) => Some(Key::F5),
    Ok(6) => Some(Key::F6),
    Ok(7) => Some(Key::F7),
    Ok(8) => Some(Key::F8),
    Ok(9) => Some(Key::F9),
    Ok(10) => Some(Key::F10),
    Ok(11) => Some(Key::F11),
    Ok(12) => Some(Key::F12),
    Ok(13) => Some(Key::F13),
    Ok(14) => Some(Key::F14),
    Ok(15) => Some(Key::F15),
    Ok(16) => Some(Key::F16),
    Ok(17) => Some(Key::F17),
    Ok(18) => Some(Key::F18),
    Ok(19) => Some(Key::F19),
    Ok(20) => Some(Key::F20),
    Ok(21) => Some(Key::F21),
    Ok(22) => Some(Key::F22),
    Ok(23) => Some(Key::F23),
    Ok(24) => Some(Key::F24),
    _ => None,
  }
}

#[cfg(target_os = "macos")]
fn parse_right_alt() -> Key {
  Key::ROption
}

#[cfg(target_os = "windows")]
fn parse_right_alt() -> Key {
  Key::RMenu
}

#[cfg(target_os = "macos")]
fn parse_right_meta() -> Key {
  Key::RCommand
}

#[cfg(target_os = "windows")]
fn parse_right_meta() -> Key {
  Key::RWin
}

fn parse_key(name: &str) -> Option<ParsedKey> {
  let name = name.trim();
  if name.is_empty() {
    return None;
  }

  if name.len() == 1 {
    let c = name.chars().next()?;
    if c.is_ascii_alphabetic() {
      #[cfg(target_os = "windows")]
      return parse_letter_key(c).map(ParsedKey::Key);
      #[cfg(target_os = "macos")]
      return macos_physical_keycode(c).map(ParsedKey::Raw);
    }
    if c.is_ascii_digit() {
      #[cfg(target_os = "windows")]
      return parse_digit_key(c).map(ParsedKey::Key);
      #[cfg(target_os = "macos")]
      return macos_physical_keycode(c).map(ParsedKey::Raw);
    }
  }

  if let Some(key) = parse_function_key(name) {
    return Some(ParsedKey::Key(key));
  }

  match name.to_lowercase().as_str() {
    "return" | "enter" => Some(ParsedKey::Key(Key::Return)),
    "escape" | "esc" => Some(ParsedKey::Key(Key::Escape)),
    "backspace" => Some(ParsedKey::Key(Key::Backspace)),
    "delete" | "del" => Some(ParsedKey::Key(Key::Delete)),
    "tab" => Some(ParsedKey::Key(Key::Tab)),
    "space" | "spacebar" => Some(ParsedKey::Key(Key::Space)),
    "up" | "uparrow" | "arrowup" => Some(ParsedKey::Key(Key::UpArrow)),
    "down" | "downarrow" | "arrowdown" => Some(ParsedKey::Key(Key::DownArrow)),
    "left" | "leftarrow" | "arrowleft" => Some(ParsedKey::Key(Key::LeftArrow)),
    "right" | "rightarrow" | "arrowright" => Some(ParsedKey::Key(Key::RightArrow)),
    "home" => Some(ParsedKey::Key(Key::Home)),
    "end" => Some(ParsedKey::Key(Key::End)),
    "pageup" | "page_up" => Some(ParsedKey::Key(Key::PageUp)),
    "pagedown" | "page_down" => Some(ParsedKey::Key(Key::PageDown)),
    "capslock" | "caps_lock" => Some(ParsedKey::Key(Key::CapsLock)),
    "control" | "ctrl" | "lcontrol" | "leftcontrol" => Some(ParsedKey::Key(Key::Control)),
    "shift" | "lshift" | "leftshift" => Some(ParsedKey::Key(Key::Shift)),
    "alt" | "option" | "lalt" | "leftalt" => Some(ParsedKey::Key(Key::Alt)),
    "meta" | "command" | "cmd" | "super" | "win" | "lwin" | "leftmeta" => {
      Some(ParsedKey::Key(Key::Meta))
    }
    "rcontrol" | "rightcontrol" => Some(ParsedKey::Key(Key::RControl)),
    "rshift" | "rightshift" => Some(ParsedKey::Key(Key::RShift)),
    "ralt" | "roption" | "rightalt" => Some(ParsedKey::Key(parse_right_alt())),
    "rmeta" | "rcommand" | "rcmd" | "rwin" | "rightmeta" => {
      Some(ParsedKey::Key(parse_right_meta()))
    }
    "numpad0" => Some(ParsedKey::Key(Key::Numpad0)),
    "numpad1" => Some(ParsedKey::Key(Key::Numpad1)),
    "numpad2" => Some(ParsedKey::Key(Key::Numpad2)),
    "numpad3" => Some(ParsedKey::Key(Key::Numpad3)),
    "numpad4" => Some(ParsedKey::Key(Key::Numpad4)),
    "numpad5" => Some(ParsedKey::Key(Key::Numpad5)),
    "numpad6" => Some(ParsedKey::Key(Key::Numpad6)),
    "numpad7" => Some(ParsedKey::Key(Key::Numpad7)),
    "numpad8" => Some(ParsedKey::Key(Key::Numpad8)),
    "numpad9" => Some(ParsedKey::Key(Key::Numpad9)),
    _ => None,
  }
}

fn invalid_key_message(key: &str) -> String {
  format!(
    "Invalid key name: {key}. Examples: f, a, 1, f1, enter, escape, control, shift, alt, meta, arrowUp"
  )
}

fn send_key(enigo: &mut Enigo, parsed: ParsedKey, direction: enigo::Direction) {
  let result = match parsed {
    ParsedKey::Key(key) => enigo.key(key, direction),
    #[cfg(target_os = "macos")]
    ParsedKey::Raw(code) => enigo.raw(code, direction),
  };

  if let Err(e) = result {
    eprintln!("Failed to send key: {:?}", e);
  }
}

fn parse_button(button: &str) -> Option<Button> {
  match button.to_lowercase().as_str() {
    "left" => Some(Button::Left),
    "right" => Some(Button::Right),
    "middle" => Some(Button::Middle),
    "scrollup" | "scroll_up" => Some(Button::ScrollUp),
    "scrolldown" | "scroll_down" => Some(Button::ScrollDown),
    "scrollleft" | "scroll_left" => Some(Button::ScrollLeft),
    "scrollright" | "scroll_right" => Some(Button::ScrollRight),
    _ => None,
  }
}

fn invalid_button_message(button: &str) -> String {
  format!(
    "Invalid button name: {button}. Valid options are: left, right, middle, scrollUp, scrollDown, scrollLeft, scrollRight"
  )
}

#[napi]
fn move_mouse_rel(x: i32, y: i32) {
  with_enigo(|enigo| {
    if let Err(e) = enigo.move_mouse(x, y, Rel) {
      eprintln!("Failed to move mouse: {:?}", e);
    }
  });
}

#[napi]
fn move_mouse_abs(x: i32, y: i32) {
  with_enigo(|enigo| {
    if let Err(e) = enigo.move_mouse(x, y, Abs) {
      eprintln!("Failed to move mouse: {:?}", e);
    }
  });
}

#[napi]
fn mouse_click(#[napi(ts_arg_type = "MouseButton")] button: String) {
  with_enigo(|enigo| {
    let Some(button) = parse_button(&button) else {
      eprintln!("{}", invalid_button_message(&button));
      return;
    };
    if let Err(e) = enigo.button(button, Click) {
      eprintln!("Failed to click mouse: {:?}", e);
    }
  });
}

#[napi]
fn mouse_down(#[napi(ts_arg_type = "MouseButton")] button: String) {
  with_enigo(|enigo| {
    let Some(button) = parse_button(&button) else {
      eprintln!("{}", invalid_button_message(&button));
      return;
    };
    if let Err(e) = enigo.button(button, Press) {
      eprintln!("Failed to press mouse button: {:?}", e);
    }
  });
}

#[napi]
fn mouse_up(#[napi(ts_arg_type = "MouseButton")] button: String) {
  with_enigo(|enigo| {
    let Some(button) = parse_button(&button) else {
      eprintln!("{}", invalid_button_message(&button));
      return;
    };
    if let Err(e) = enigo.button(button, Release) {
      eprintln!("Failed to release mouse button: {:?}", e);
    }
  });
}

#[napi]
fn mouse_scroll(length: i32, is_vertical: bool) {
  with_enigo(|enigo| {
    let axis = if is_vertical { Vertical } else { Horizontal };
    if let Err(e) = enigo.scroll(length, axis) {
      eprintln!("Failed to scroll: {:?}", e);
    }
  });
}

#[napi]
fn key_tap(#[napi(ts_arg_type = "KeyboardKey")] key: String) {
  with_enigo(|enigo| {
    let Some(parsed) = parse_key(&key) else {
      eprintln!("{}", invalid_key_message(&key));
      return;
    };
    send_key(enigo, parsed, Click);
  });
}

#[napi]
fn key_down(#[napi(ts_arg_type = "KeyboardKey")] key: String) {
  with_enigo(|enigo| {
    let Some(parsed) = parse_key(&key) else {
      eprintln!("{}", invalid_key_message(&key));
      return;
    };
    send_key(enigo, parsed, Press);
  });
}

#[napi]
fn key_up(#[napi(ts_arg_type = "KeyboardKey")] key: String) {
  with_enigo(|enigo| {
    let Some(parsed) = parse_key(&key) else {
      eprintln!("{}", invalid_key_message(&key));
      return;
    };
    send_key(enigo, parsed, Release);
  });
}

#[napi]
fn type_text(text: String) {
  with_enigo(|enigo| {
    if let Err(e) = enigo.text(&text) {
      eprintln!("Failed to type text: {:?}", e);
    }
  });
}
