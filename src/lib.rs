#![deny(clippy::all)]

use enigo::{
  Axis::{Horizontal, Vertical},
  Button,
  Coordinate::{Abs, Rel},
  Direction::Click,
  Enigo, Mouse, Settings,
};
use napi_derive::napi;
#[cfg(target_os = "windows")]
use std::sync::Once;

#[cfg(target_os = "windows")]
static SET_DPI_AWARENESS: Once = Once::new();

fn create_enigo() -> Option<Enigo> {
  #[cfg(target_os = "windows")]
  SET_DPI_AWARENESS.call_once(|| {
    if enigo::set_dpi_awareness().is_err() {
      eprintln!("Failed to set DPI awareness (non-fatal)");
    }
  });

  match Enigo::new(&Settings::default()) {
    Ok(enigo) => Some(enigo),
    Err(e) => {
      eprintln!("Failed to create Enigo: {:?}", e);
      None
    }
  }
}

#[napi]
fn move_mouse_rel(x: i32, y: i32) {
  if let Some(mut enigo) = create_enigo() {
    if let Err(e) = enigo.move_mouse(x, y, Rel) {
      eprintln!("Failed to move mouse: {:?}", e);
    }
  }
}

#[napi]
fn move_mouse_abs(x: i32, y: i32) {
  if let Some(mut enigo) = create_enigo() {
    if let Err(e) = enigo.move_mouse(x, y, Abs) {
      eprintln!("Failed to move mouse: {:?}", e);
    }
  }
}

#[napi]
fn mouse_click(button: String) {
  if let Some(mut enigo) = create_enigo() {
    let button = match button.as_str() {
      "left" => Button::Left,
      "right" => Button::Right,
      "middle" => Button::Middle,
      _ => {
        eprintln!("Invalid button specified");
        return;
      }
    };
    if let Err(e) = enigo.button(button, Click) {
      eprintln!("Failed to click mouse: {:?}", e);
    }
  }
}

#[napi]
fn mouse_down(button: String) {
  if let Some(mut enigo) = create_enigo() {
    let button = match button.as_str() {
      "left" => Button::Left,
      "right" => Button::Right,
      "middle" => Button::Middle,
      _ => {
        eprintln!("Invalid button specified");
        return;
      }
    };
    if let Err(e) = enigo.button(button, enigo::Direction::Press) {
      eprintln!("Failed to press mouse button: {:?}", e);
    }
  }
}

#[napi]
fn mouse_up(button: String) {
  if let Some(mut enigo) = create_enigo() {
    let button = match button.as_str() {
      "left" => Button::Left,
      "right" => Button::Right,
      "middle" => Button::Middle,
      _ => {
        eprintln!("Invalid button specified");
        return;
      }
    };
    if let Err(e) = enigo.button(button, enigo::Direction::Release) {
      eprintln!("Failed to release mouse button: {:?}", e);
    }
  }
}

#[napi]
fn mouse_scroll(length: i32, is_vertical: bool) {
  if let Some(mut enigo) = create_enigo() {
    let axis = if is_vertical { Vertical } else { Horizontal };
    if let Err(e) = enigo.scroll(length, axis) {
      eprintln!("Failed to scroll: {:?}", e);
    }
  }
}
