use enigo::{Enigo, Settings};
use std::sync::{Arc, Mutex, OnceLock};
#[cfg(target_os = "windows")]
use std::sync::Once;

#[cfg(target_os = "windows")]
static SET_DPI_AWARENESS: Once = Once::new();

static ENIGO: OnceLock<Arc<Mutex<Enigo>>> = OnceLock::new();

#[cfg(target_os = "windows")]
fn init_dpi_awareness() {
  SET_DPI_AWARENESS.call_once(|| {
    if enigo::set_dpi_awareness().is_err() {
      eprintln!("Failed to set DPI awareness (non-fatal)");
    }
  });
}

fn create_enigo() -> Enigo {
  #[cfg(target_os = "windows")]
  init_dpi_awareness();

  match Enigo::new(&Settings::default()) {
    Ok(instance) => instance,
    Err(e) => {
      eprintln!("Failed to create Enigo: {:?}", e);
      panic!("Failed to create Enigo");
    }
  }
}

pub fn enigo() -> Arc<Mutex<Enigo>> {
  ENIGO
    .get_or_init(|| Arc::new(Mutex::new(create_enigo())))
    .clone()
}
