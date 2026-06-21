#![deny(clippy::all)]

use enigo::{
  Axis::{Horizontal, Vertical},
  Button,
  Coordinate::{Abs, Rel},
  Direction::Click,
  Enigo, Mouse, Settings,
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

fn parse_button(button: &str) -> Option<Button> {
  match button {
    "left" => Some(Button::Left),
    "right" => Some(Button::Right),
    "middle" => Some(Button::Middle),
    _ => None,
  }
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
fn mouse_click(button: String) {
  with_enigo(|enigo| {
    let Some(button) = parse_button(&button) else {
      eprintln!("Invalid button specified");
      return;
    };
    if let Err(e) = enigo.button(button, Click) {
      eprintln!("Failed to click mouse: {:?}", e);
    }
  });
}

#[napi]
fn mouse_down(button: String) {
  with_enigo(|enigo| {
    let Some(button) = parse_button(&button) else {
      eprintln!("Invalid button specified");
      return;
    };
    if let Err(e) = enigo.button(button, enigo::Direction::Press) {
      eprintln!("Failed to press mouse button: {:?}", e);
    }
  });
}

#[napi]
fn mouse_up(button: String) {
  with_enigo(|enigo| {
    let Some(button) = parse_button(&button) else {
      eprintln!("Invalid button specified");
      return;
    };
    if let Err(e) = enigo.button(button, enigo::Direction::Release) {
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
