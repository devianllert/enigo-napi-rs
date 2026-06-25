#![deny(clippy::all)]

mod keyboard;
mod mouse;
mod state;

use napi_derive::napi;

#[napi]
fn move_mouse_rel(x: i32, y: i32) {
  mouse::mouse().move_rel(x, y);
}

#[napi]
fn move_mouse_abs(x: i32, y: i32) {
  mouse::mouse().move_abs(x, y);
}

#[napi]
fn move_mouse_smooth(x: i32, y: i32) {
  mouse::mouse().move_smooth(x, y);
}

#[napi]
fn move_mouse_smooth_with_speed(x: i32, y: i32, speed: f64) {
  mouse::mouse().move_smooth_with_speed(x, y, speed);
}

#[napi]
fn get_mouse_pos() -> Option<mouse::MousePosition> {
  mouse::mouse().get_position()
}

#[napi]
fn mouse_click(#[napi(ts_arg_type = "MouseButton")] button: String) {
  mouse::mouse().click_named(&button);
}

#[napi]
fn mouse_double_click(#[napi(ts_arg_type = "MouseButton")] button: String) {
  let Some(button) = mouse::parse_button(&button) else {
    eprintln!("{}", mouse::invalid_button_message(&button));
    return;
  };
  mouse::mouse().double_click(button);
}

#[napi]
fn mouse_down(#[napi(ts_arg_type = "MouseButton")] button: String) {
  mouse::mouse().press_named(&button);
}

#[napi]
fn mouse_up(#[napi(ts_arg_type = "MouseButton")] button: String) {
  mouse::mouse().release_named(&button);
}

#[napi]
fn mouse_drag(x: i32, y: i32) {
  mouse::mouse().drag(x, y);
}

#[napi]
fn mouse_scroll(length: i32, is_vertical: bool) {
  mouse::mouse().scroll(length, is_vertical);
}

#[napi]
fn mouse_scroll_xy(x: i32, y: i32) {
  mouse::mouse().scroll_xy(x, y);
}

#[napi]
fn key_tap(
  #[napi(ts_arg_type = "KeyboardKey")] key: String,
  modifier: Option<Vec<String>>,
) {
  keyboard::keyboard().tap(&key, modifier.as_deref());
}

#[napi]
fn key_toggle(
  #[napi(ts_arg_type = "KeyboardKey")] key: String,
  down: String,
  modifier: Option<Vec<String>>,
) {
  keyboard::keyboard().toggle(&key, &down, modifier.as_deref());
}

#[napi]
fn key_down(
  #[napi(ts_arg_type = "KeyboardKey")] key: String,
  modifier: Option<Vec<String>>,
) {
  keyboard::keyboard().down(&key, modifier.as_deref());
}

#[napi]
fn key_up(
  #[napi(ts_arg_type = "KeyboardKey")] key: String,
  modifier: Option<Vec<String>>,
) {
  keyboard::keyboard().up(&key, modifier.as_deref());
}

#[napi]
fn type_text(text: String) {
  keyboard::keyboard().type_text(&text);
}

#[napi]
fn type_text_delayed(text: String, cpm: u32) {
  keyboard::keyboard().type_text_delayed(&text, cpm);
}

#[napi]
fn unicode_tap(ch: String) {
  if let Some(c) = ch.chars().next() {
    keyboard::keyboard().unicode_tap(c);
  }
}
