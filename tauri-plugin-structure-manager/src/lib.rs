use tauri::{
    plugin::{Builder, TauriPlugin}, Manager, Runtime
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
use desktop::StructureManager;
#[cfg(mobile)]
use mobile::StructureManager;

mod manager;
use manager::{structure::StructureConfig, verification::verify_document};

use serde_json;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the structure-manager APIs.
pub trait StructureManagerExt<R: Runtime> {
    fn structure_manager(&self) -> &StructureManager<R>;
}

impl<R: Runtime, T: Manager<R>> crate::StructureManagerExt<R> for T {
    fn structure_manager(&self) -> &StructureManager<R> {
        self.state::<StructureManager<R>>().inner()
    }
}   

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, StructureConfig> {
Builder::<R, StructureConfig>::new("structure-manager")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
        #[cfg(mobile)]
        let structure_manager = mobile::init(app, api)?;
        #[cfg(desktop)]
        let structure_manager = desktop::init(app, api)?;
        app.manage(structure_manager);

        // Verify the structure of the app
        match &app.config().schema {
            Some(schema) => {
                let structure_config: StructureConfig = serde_json::from_str(&schema)?;
                // TODO: save the structure_config in the app state
            }
            None => {}
        }
        
        Ok(())
    })
    .build()
}
