//! Mouse control module
//!
//! Provides mouse movement, clicking, dragging, and scrolling functionality.

use crate::state;
use enigo::{
  Axis::{Horizontal, Vertical},
  Button, Coordinate, Direction, Enigo, Mouse as MouseTrait,
};
use napi_derive::napi;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

/// Mouse position
#[napi(object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MousePosition {
  pub x: i32,
  pub y: i32,
}

/// Mouse controller
pub struct Mouse {
  enigo: Arc<Mutex<Enigo>>,
}

static MOUSE: OnceLock<Mouse> = OnceLock::new();

pub fn mouse() -> &'static Mouse {
  MOUSE.get_or_init(Mouse::new)
}

impl Mouse {
  fn new() -> Self {
    Self {
      enigo: state::enigo(),
    }
  }

  pub fn move_rel(&self, x: i32, y: i32) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.move_mouse(x, y, Coordinate::Rel) {
      eprintln!("Failed to move mouse: {:?}", e);
    }
  }

  pub fn move_abs(&self, x: i32, y: i32) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.move_mouse(x, y, Coordinate::Abs) {
      eprintln!("Failed to move mouse: {:?}", e);
    }
  }

  pub fn move_smooth(&self, x: i32, y: i32) {
    self.move_smooth_with_speed(x, y, 3.0);
  }

  pub fn move_smooth_with_speed(&self, x: i32, y: i32, speed: f64) {
    let Some(current) = self.get_position() else {
      return;
    };

    let start_x = current.x as f64;
    let start_y = current.y as f64;
    let end_x = x as f64;
    let end_y = y as f64;

    let distance = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
    let steps = (distance / speed).max(1.0) as u32;

    for i in 0..=steps {
      let t = i as f64 / steps as f64;
      let eased = t * t * (3.0 - 2.0 * t);
      let current_x = (start_x + (end_x - start_x) * eased) as i32;
      let current_y = (start_y + (end_y - start_y) * eased) as i32;

      self.move_abs(current_x, current_y);
      thread::sleep(Duration::from_millis(1));
    }
  }

  pub fn get_position(&self) -> Option<MousePosition> {
    let Ok(enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return None;
    };
    match enigo.location() {
      Ok((x, y)) => Some(MousePosition { x, y }),
      Err(e) => {
        eprintln!("Failed to get mouse position: {:?}", e);
        None
      }
    }
  }

  pub fn click(&self, button: Button) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.button(button, Direction::Click) {
      eprintln!("Failed to click mouse: {:?}", e);
    }
  }

  pub fn click_named(&self, button: &str) {
    let Some(button) = parse_button(button) else {
      eprintln!("{}", invalid_button_message(button));
      return;
    };
    self.click(button);
  }

  pub fn double_click(&self, button: Button) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.button(button, Direction::Click) {
      eprintln!("Failed to click mouse: {:?}", e);
      return;
    }
    thread::sleep(Duration::from_millis(50));
    if let Err(e) = enigo.button(button, Direction::Click) {
      eprintln!("Failed to double-click mouse: {:?}", e);
    }
  }

  pub fn press(&self, button: Button) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.button(button, Direction::Press) {
      eprintln!("Failed to press mouse button: {:?}", e);
    }
  }

  pub fn press_named(&self, button: &str) {
    let Some(button) = parse_button(button) else {
      eprintln!("{}", invalid_button_message(button));
      return;
    };
    self.press(button);
  }

  pub fn release(&self, button: Button) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.button(button, Direction::Release) {
      eprintln!("Failed to release mouse button: {:?}", e);
    }
  }

  pub fn release_named(&self, button: &str) {
    let Some(button) = parse_button(button) else {
      eprintln!("{}", invalid_button_message(button));
      return;
    };
    self.release(button);
  }

  pub fn drag(&self, x: i32, y: i32) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if let Err(e) = enigo.button(Button::Left, Direction::Press) {
      eprintln!("Failed to press mouse button: {:?}", e);
      return;
    }
    if let Err(e) = enigo.move_mouse(x, y, Coordinate::Abs) {
      eprintln!("Failed to move mouse: {:?}", e);
    }
    if let Err(e) = enigo.button(Button::Left, Direction::Release) {
      eprintln!("Failed to release mouse button: {:?}", e);
    }
  }

  pub fn scroll(&self, length: i32, is_vertical: bool) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    let axis = if is_vertical { Vertical } else { Horizontal };
    if let Err(e) = enigo.scroll(length, axis) {
      eprintln!("Failed to scroll: {:?}", e);
    }
  }

  pub fn scroll_xy(&self, x: i32, y: i32) {
    let Ok(mut enigo) = self.enigo.lock() else {
      eprintln!("Failed to lock Enigo");
      return;
    };
    if x != 0 {
      if let Err(e) = enigo.scroll(x, Horizontal) {
        eprintln!("Failed to scroll horizontally: {:?}", e);
      }
    }
    if y != 0 {
      if let Err(e) = enigo.scroll(y, Vertical) {
        eprintln!("Failed to scroll vertically: {:?}", e);
      }
    }
  }
}

pub fn invalid_button_message(button: &str) -> String {
  format!(
    "Invalid button name: {button}. Valid options are: left, right, middle, scrollUp, scrollDown, scrollLeft, scrollRight"
  )
}

pub fn parse_button(button: &str) -> Option<Button> {
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
