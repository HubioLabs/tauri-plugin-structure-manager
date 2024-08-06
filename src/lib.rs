use std::{path::PathBuf, sync::Mutex};
use log::{info, warn, error};

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
use desktop::StructureManager;
#[cfg(mobile)]
use mobile::StructureManager;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the structure-manager APIs.
pub trait StructureManagerExt<R: Runtime> {
    fn structure_manager(&self) -> &StructureManager<R>;
    fn dfs_verify(
        &self,
        path: PathBuf,
        structure_item: &StructureItem,
    ) -> std::result::Result<(), String>;
    fn verify_app_cache(&self) -> std::result::Result<(), String>;
    fn verify_app_config(&self) -> std::result::Result<(), String>;
    fn verify_app_data(&self) -> std::result::Result<(), String>;
    fn verify_app_local_data(&self) -> std::result::Result<(), String>;
    fn verify_app_log(&self) -> std::result::Result<(), String>;
    fn verify_audio(&self) -> std::result::Result<(), String>;
    fn verify_cache(&self) -> std::result::Result<(), String>;
    fn verify_config(&self) -> std::result::Result<(), String>;
    fn verify_data(&self) -> std::result::Result<(), String>;
    fn verify_desktop(&self) -> std::result::Result<(), String>;
    fn verify_document(&self) -> std::result::Result<(), String>;
    fn verify_download(&self) -> std::result::Result<(), String>;
    fn verify_executable(&self) -> std::result::Result<(), String>;
    fn verify_font(&self) -> std::result::Result<(), String>;
    fn verify_home(&self) -> std::result::Result<(), String>;
    fn verify_local_data(&self) -> std::result::Result<(), String>;
    fn verify_picture(&self) -> std::result::Result<(), String>;
    fn verify_public(&self) -> std::result::Result<(), String>;
    fn verify_resource(&self) -> std::result::Result<(), String>;
    fn verify_runtime(&self) -> std::result::Result<(), String>;
    fn verify_temp(&self) -> std::result::Result<(), String>;
    fn verify_template(&self) -> std::result::Result<(), String>;
    fn verify_video(&self) -> std::result::Result<(), String>;
}

impl<R: Runtime, T: Manager<R>> crate::StructureManagerExt<R> for T {
    fn structure_manager(&self) -> &StructureManager<R> {
        self.state::<StructureManager<R>>().inner()
    }

    /// Performs a depth-first search (DFS) verification of the structure of a directory based on the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the directory to be verified.
    /// * `source` - The structure item representing the directory and its options.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the directory structure is valid, or `Err(String)` with an error message if any issues are found.
    fn dfs_verify(
        &self,
        path: PathBuf,
        structure_item: &StructureItem,
    ) -> std::result::Result<(), String> {
        let mut repair = false;
        let mut _strict = false; // TODO: Implement strict verification

        match &structure_item.options {
            Some(options) => {
                if let Some(value) = options.repair {
                    repair = value;
                }

                if let Some(value) = options.strict {
                    _strict = value;
                }
            }
            None => {}
        }

        match &structure_item.files {
            Some(files) => {
                for file in files {
                    let file_path = path.join(file);
                    if !file_path.exists() {
                        return Err(format!("File not found: {:?}", file_path));
                    }
                }
            }
            None => {}
        }

        match &structure_item.dirs {
            Some(dirs) => {
                for (dir_name, dir) in dirs {
                    let dir_path = path.join(dir_name);
                    if !dir_path.exists() {
                        if repair {
                            std::fs::create_dir_all(&dir_path).map_err(|e| {
                                format!(
                                    "Failed to create directory: {:?}, error: {:?}",
                                    dir_path, e
                                )
                            })?;
                        } else {
                            return Err(format!("Directory not found: {:?}", dir_path));
                        }
                    }
                    self.dfs_verify(dir_path, dir)?;
                }
            }
            None => {}
        }

        Ok(())
    }

    /// Verifies the structure of the `appCache` directory based on the provided structure configuration.
    fn verify_app_cache(&self) -> std::result::Result<(), String> {
        let path = match self.path().app_cache_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve app cache path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.app_cache {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `appCache` not found".to_string()),
        }
    }

    /// Verifies the structure of the `appConfig` directory based on the provided structure configuration.
    fn verify_app_config(&self) -> std::result::Result<(), String> {
        let path = match self.path().app_config_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve app config path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.app_config {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `appConfig` not found".to_string()),
        }
    }

    /// Verifies the structure of the `app_data` directory based on the provided structure configuration.
    fn verify_app_data(&self) -> std::result::Result<(), String> {
        let path = match self.path().app_data_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve app data path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.app_data {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `appData` not found".to_string()),
        }
    }

    /// Verifies the structure of the `app_local_data` directory based on the provided structure configuration.
    fn verify_app_local_data(&self) -> std::result::Result<(), String> {
        let path = match self.path().app_local_data_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve app local data path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.app_local_data {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `appLocalData` not found".to_string()),
        }
    }

    /// Verifies the structure of the `app_log` directory based on the provided structure configuration.
    fn verify_app_log(&self) -> std::result::Result<(), String> {
        let path = match self.path().app_log_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve app log path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.app_log {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `appLog` not found".to_string()),
        }
    }

    /// Verifies the structure of the `audio` directory based on the provided structure configuration.
    fn verify_audio(&self) -> std::result::Result<(), String> {
        let path = match self.path().audio_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve audio path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.audio {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `audio` not found".to_string()),
        }
    }

    /// Verifies the structure of the `cache` directory based on the provided structure configuration.
    fn verify_cache(&self) -> std::result::Result<(), String> {
        let path = match self.path().cache_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve cache path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.cache {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `cache` not found".to_string()),
        }
    }

    /// Verifies the structure of the `config` directory based on the provided structure configuration.
    fn verify_config(&self) -> std::result::Result<(), String> {
        let path = match self.path().config_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve config path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.config {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `config` not found".to_string()),
        }
    }

    /// Verifies the structure of the `data` directory based on the provided structure configuration.
    fn verify_data(&self) -> std::result::Result<(), String> {
        let path = match self.path().data_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve data path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.data {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `data` not found".to_string()),
        }
    }

    /// Verifies the structure of the `desktop` directory based on the provided structure configuration.
    fn verify_desktop(&self) -> std::result::Result<(), String> {
        let path = match self.path().desktop_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve desktop path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.desktop {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `desktop` not found".to_string()),
        }
    }

    /// Verifies the structure of the `document` directory based on the provided structure configuration.
    fn verify_document(&self) -> std::result::Result<(), String> {
        let path = match self.path().document_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve document path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.document {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `document` not found".to_string()),
        }
    }

    /// Verifies the structure of the `download` directory based on the provided structure configuration.
    fn verify_download(&self) -> std::result::Result<(), String> {
        let path = match self.path().download_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve download path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.download {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `download` not found".to_string()),
        }
    }

    /// Verifies the structure of the `executable` directory based on the provided structure configuration.
    fn verify_executable(&self) -> std::result::Result<(), String> {
        let path = match self.path().executable_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve executable path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.executable {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `executable` not found".to_string()),
        }
    }

    /// Verifies the structure of the `font` directory based on the provided structure configuration.
    fn verify_font(&self) -> std::result::Result<(), String> {
        let path = match self.path().font_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve font path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.font {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `font` not found".to_string()),
        }
    }

    /// Verifies the structure of the `home` directory based on the provided structure configuration.
    fn verify_home(&self) -> std::result::Result<(), String> {
        let path = match self.path().home_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve home path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.home {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `home` not found".to_string()),
        }
    }

    /// Verifies the structure of the `local_data` directory based on the provided structure configuration.
    fn verify_local_data(&self) -> std::result::Result<(), String> {
        let path = match self.path().local_data_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve local data path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.local_data {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `localData` not found".to_string()),
        }
    }

    /// Verifies the structure of the `picture` directory based on the provided structure configuration.
    fn verify_picture(&self) -> std::result::Result<(), String> {
        let path = match self.path().picture_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve picture path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.picture {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `picture` not found".to_string()),
        }
    }

    /// Verifies the structure of the `public` directory based on the provided structure configuration.
    fn verify_public(&self) -> std::result::Result<(), String> {
        let path = match self.path().public_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve public path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.public {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `public` not found".to_string()),
        }
    }

    /// Verifies the structure of the `resource` directory based on the provided structure configuration.
    fn verify_resource(&self) -> std::result::Result<(), String> {
        let path = match self.path().resource_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve resource path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.resource {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `resource` not found".to_string()),
        }
    }

    /// Verifies the structure of the `runtime` directory based on the provided structure configuration.
    fn verify_runtime(&self) -> std::result::Result<(), String> {
        let path = match self.path().runtime_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve runtime path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.runtime {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `runtime` not found".to_string()),
        }
    }

    /// Verifies the structure of the `temp` directory based on the provided structure configuration.
    fn verify_temp(&self) -> std::result::Result<(), String> {
        let path = match self.path().temp_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve temp path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.temp {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `temp` not found".to_string()),
        }
    }

    /// Verifies the structure of the `template` directory based on the provided structure configuration.
    fn verify_template(&self) -> std::result::Result<(), String> {
        let path = match self.path().template_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve template path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.template {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `template` not found".to_string()),
        }
    }

    /// Verifies the structure of the `video` directory based on the provided structure configuration.
    fn verify_video(&self) -> std::result::Result<(), String> {
        let path = match self.path().video_dir() {
            Ok(path) => path,
            Err(e) => return Err(format!("Failed to resolve video path: {:?}", e)),
        };

        let state_mutex = self.state::<Mutex<StructureConfig>>();
        let structure_config = state_mutex.lock().unwrap();

        match &structure_config.video {
            Some(structure_item) => self.dfs_verify(path, structure_item),
            None => Err("Structure configuration field `video` not found".to_string()),
        }
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, Option<StructureConfig>> {
    Builder::<R, Option<StructureConfig>>::new("structure-manager")
        .setup(|app, api| {
            match api.config() {
                Some(structure_config) => {
                    info!("Using provided structure configuration\n{:?}", structure_config);
                    app.manage(Mutex::new(structure_config.clone()))
                },
                None => {
                    warn!("Using default structure configuration");
                    app.manage(Mutex::new(StructureConfig::default()))
                },
            };

            #[cfg(mobile)]
            let structure_manager = mobile::init(app, api)?;
            #[cfg(desktop)]
            let structure_manager = desktop::init(app, api)?;
            app.manage(structure_manager);

            Ok(())
        })
        .build()
}
