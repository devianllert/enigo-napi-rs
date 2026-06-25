//! Keyboard control module
//!
//! Layout-independent key simulation and Unicode text input.

use crate::state;
use enigo::{Direction, Enigo, Key, Keyboard as EnigoKeyboard};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
enum ParsedKey {
  Key(Key),
  #[cfg(target_os = "macos")]
  Raw(u16),
}

/// Keyboard controller
pub struct Keyboard {
  enigo: Arc<Mutex<Enigo>>,
}

static KEYBOARD: OnceLock<Keyboard> = OnceLock::new();

pub fn keyboard() -> &'static Keyboard {
  KEYBOARD.get_or_init(Keyboard::new)
}

impl Keyboard {
  fn new() -> Self {
    Self {
      enigo: state::enigo(),
    }
  }

  pub fn tap(&self, key: &str, modifier: Option<&[String]>) {
    let Some(main) = parse_key(key) else {
      eprintln!("{}", invalid_key_message(key));
      return;
    };

    let modifiers = match modifier {
      Some(mods) if !mods.is_empty() => parse_modifiers(mods),
      _ => None,
    };

    if modifier.is_some() && modifiers.is_none() {
      return;
    }

    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };

    if let Some(ref mods) = modifiers {
      for parsed in mods {
        send_key(&mut enigo, *parsed, Direction::Press);
      }
    }

    send_key(&mut enigo, main, Direction::Click);

    if let Some(ref mods) = modifiers {
      for parsed in mods.iter().rev() {
        send_key(&mut enigo, *parsed, Direction::Release);
      }
    }
  }

  pub fn toggle(&self, key: &str, down: &str, modifier: Option<&[String]>) {
    let direction = match down {
      "down" => Direction::Press,
      "up" => Direction::Release,
      _ => {
        eprintln!("Invalid direction: {down}. Use 'down' or 'up'.");
        return;
      }
    };

    let Some(main) = parse_key(key) else {
      eprintln!("{}", invalid_key_message(key));
      return;
    };

    let modifiers = match modifier {
      Some(mods) if !mods.is_empty() => parse_modifiers(mods),
      _ => None,
    };

    if modifier.is_some() && modifiers.is_none() {
      return;
    }

    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };

    if let Some(ref mods) = modifiers {
      for parsed in mods {
        send_key(&mut enigo, *parsed, direction);
      }
    }

    send_key(&mut enigo, main, direction);
  }

  pub fn down(&self, key: &str, modifier: Option<&[String]>) {
    self.toggle(key, "down", modifier);
  }

  pub fn up(&self, key: &str, modifier: Option<&[String]>) {
    self.toggle(key, "up", modifier);
  }

  pub fn type_text(&self, text: &str) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.text(text) {
      eprintln!("Failed to type text: {:?}", e);
    }
  }

  pub fn type_text_delayed(&self, text: &str, cpm: u32) {
    let delay_ms = if cpm > 0 {
      (60000.0 / f64::from(cpm)) as u64
    } else {
      0
    };

    for ch in text.chars() {
      let Ok(mut enigo) = self.enigo.lock() else {
        eprintln!("Failed to lock Enigo");
        return;
      };
      if let Err(e) = enigo.text(&ch.to_string()) {
        eprintln!("Failed to type text: {:?}", e);
      }
      drop(enigo);

      if delay_ms > 0 {
        thread::sleep(Duration::from_millis(delay_ms));
      }
    }
  }

  pub fn unicode_tap(&self, ch: char) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.text(&ch.to_string()) {
      eprintln!("Failed to type unicode: {:?}", e);
    }
  }
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
    #[cfg(not(target_os = "macos"))]
    Ok(21) => Some(Key::F21),
    #[cfg(not(target_os = "macos"))]
    Ok(22) => Some(Key::F22),
    #[cfg(not(target_os = "macos"))]
    Ok(23) => Some(Key::F23),
    #[cfg(not(target_os = "macos"))]
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
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    "ralt" | "roption" | "rightalt" => Some(ParsedKey::Key(parse_right_alt())),
    #[cfg(target_os = "macos")]
    "rmeta" | "rcommand" | "rcmd" | "rwin" | "rightmeta" => Some(ParsedKey::Key(Key::RCommand)),
    #[cfg(target_os = "windows")]
    "rmeta" | "rcommand" | "rcmd" | "rwin" | "rightmeta" => Some(ParsedKey::Key(Key::RWin)),
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

fn parse_modifiers(mods: &[String]) -> Option<Vec<ParsedKey>> {
  let mut parsed = Vec::with_capacity(mods.len());
  for name in mods {
    let Some(key) = parse_key(name) else {
      eprintln!("{}", invalid_key_message(name));
      return None;
    };
    parsed.push(key);
  }
  Some(parsed)
}

fn send_key(enigo: &mut Enigo, parsed: ParsedKey, direction: Direction) {
  let result = match parsed {
    ParsedKey::Key(key) => enigo.key(key, direction),
    #[cfg(target_os = "macos")]
    ParsedKey::Raw(code) => enigo.raw(code, direction),
  };

  if let Err(e) = result {
    eprintln!("Failed to send key: {:?}", e);
  }
}
