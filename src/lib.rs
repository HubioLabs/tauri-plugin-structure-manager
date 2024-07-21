use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::;
#[cfg(mobile)]
use mobile::;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the  APIs.
pub trait Ext<R: Runtime> {
  fn (&self) -> &<R>;
}

impl<R: Runtime, T: Manager<R>> crate::Ext<R> for T {
  fn (&self) -> &<R> {
    self.state::<<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let  = mobile::init(app, api)?;
      #[cfg(desktop)]
      let  = desktop::init(app, api)?;
      app.manage();
      Ok(())
    })
    .build()
}
