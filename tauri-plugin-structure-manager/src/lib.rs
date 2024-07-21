/**
 * This module contains the implementation of a Tauri plugin called `structure-manager`.
 * The plugin provides APIs to access and verify the structure of various directories in the application.
 * It defines a `StructureConfig` struct that represents the configuration for the directory structure.
 * The plugin also extends the functionality of the `tauri::App`, `tauri::AppHandle`, and `tauri::Window` types
 * with the `StructureManagerExt` trait, which allows accessing the structure-manager APIs.
 * The `init` function initializes the plugin and sets up the necessary event handlers.
 * The `verify_structure` function verifies the structure of the application directories based on the provided configuration.
 * The remaining functions are helper functions used for verifying specific directories.
 */

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

use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};
use tauri::path::PathResolver;
use serde_json;

#[derive(Deserialize)]
enum ConfigValue {
    File(String),
    Directory(HashMap<String, ConfigValue>),
}

#[derive(Deserialize)]
pub struct StructureConfig {
    app_cache: HashMap<String, ConfigValue>,
    app_config: HashMap<String, ConfigValue>,
    app_data: HashMap<String, ConfigValue>,
    app_local_data: HashMap<String, ConfigValue>,
    app_log: HashMap<String, ConfigValue>,
    audio: HashMap<String, ConfigValue>,
    cache: HashMap<String, ConfigValue>,
    config: HashMap<String, ConfigValue>,
    data: HashMap<String, ConfigValue>,
    desktop: HashMap<String, ConfigValue>,
    document: HashMap<String, ConfigValue>,
    download: HashMap<String, ConfigValue>,
    executable: HashMap<String, ConfigValue>,
    font: HashMap<String, ConfigValue>,
    home: HashMap<String, ConfigValue>,
    local_data: HashMap<String, ConfigValue>,
    picture: HashMap<String, ConfigValue>,
    public: HashMap<String, ConfigValue>,
    resource: HashMap<String, ConfigValue>,
    runtime: HashMap<String, ConfigValue>,
    temp: HashMap<String, ConfigValue>,
    template: HashMap<String, ConfigValue>,
    video: HashMap<String, ConfigValue>,
}

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
                let path_resolver = app.path();
                let structured_config: StructureConfig = serde_json::from_str(&schema).unwrap();
                verify_structure(&path_resolver, &structured_config)?;
            }
            None => {}
        }
        
        Ok(())
    })
    .build()
}

/// Performs a depth-first search to verify the structure of the directories based on the provided path.
fn dfs_verify(path: PathBuf, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    for (key, value) in source {
        match value {
        ConfigValue::File(file) => {
            let file_path = path.join(file);
            if !file_path.exists() {
                return Err(format!("File not found: {:?}", file_path));
            }
        }
        ConfigValue::Directory(directory) => {
            let directory_path = path.join(key);
            if !directory_path.exists() {
                return Err(format!("Directory not found: {:?}", directory_path));
            }
            dfs_verify(directory_path, directory)?;
        }
        }
    }

Ok(())
}

/// Verifies the structure of the application directories based on the provided configuration.
fn verify_structure<R: Runtime>(path_resolver: &PathResolver<R>, config: &StructureConfig) -> std::result::Result<(), String> {
    verify_from_app_cache(&path_resolver, &config.app_cache)?;
    verify_from_app_config(&path_resolver, &config.app_config)?;
    verify_from_app_data(&path_resolver, &config.app_data)?;
    verify_from_app_local_data(&path_resolver, &config.app_local_data)?;
    verify_from_app_log(&path_resolver, &config.app_log)?;
    verify_from_audio(&path_resolver, &config.audio)?;
    verify_from_cache(&path_resolver, &config.cache)?;
    verify_from_config(&path_resolver, &config.config)?;
    verify_from_data(&path_resolver, &config.data)?;
    verify_from_desktop(&path_resolver, &config.desktop)?;
    verify_from_document(&path_resolver, &config.document)?;
    verify_from_download(&path_resolver, &config.download)?;
    verify_from_executable(&path_resolver, &config.executable)?;
    verify_from_font(&path_resolver, &config.font)?;
    verify_from_home(&path_resolver, &config.home)?;
    verify_from_local_data(&path_resolver, &config.local_data)?;
    verify_from_picture(&path_resolver, &config.picture)?;
    verify_from_public(&path_resolver, &config.public)?;
    verify_from_resource(&path_resolver, &config.resource)?;
    verify_from_runtime(&path_resolver, &config.runtime)?;
    verify_from_temp(&path_resolver, &config.temp)?;
    verify_from_template(&path_resolver, &config.template)?;
    verify_from_video(&path_resolver, &config.video)?;

    Ok(())
}

fn verify_from_app_cache<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let app_cache = path_resolver.app_cache_dir();

    match app_cache {
        Ok(app_cache) => {
            dfs_verify(app_cache, source)?;
        }
        Err(_) => {
            return Err("App cache directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_app_config<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let app_config = path_resolver.app_config_dir();

    match app_config {
        Ok(app_config) => {
            dfs_verify(app_config, source)?;
        }
        Err(_) => {
            return Err("App config directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_app_data<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let app_data = path_resolver.app_data_dir();

    match app_data {
        Ok(app_data) => {
            dfs_verify(app_data, source)?;
        }
        Err(_) => {
            return Err("App data directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_app_local_data<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let app_local_data = path_resolver.app_local_data_dir();

    match app_local_data {
        Ok(app_local_data) => {
            dfs_verify(app_local_data, source)?;
        }
        Err(_) => {
            return Err("App local data directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_app_log<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let app_log = path_resolver.app_log_dir();

    match app_log {
        Ok(app_log) => {
            dfs_verify(app_log, source)?;
        }
        Err(_) => {
            return Err("App log directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_audio<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let audio = path_resolver.audio_dir();

    match audio {
        Ok(audio) => {
            dfs_verify(audio, source)?;
        }
        Err(_) => {
            return Err("Audio directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_cache<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let cache = path_resolver.cache_dir();

    match cache {
        Ok(cache) => {
            dfs_verify(cache, source)?;
        }
        Err(_) => {
            return Err("Cache directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_config<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let config = path_resolver.config_dir();

    match config {
        Ok(config) => {
            dfs_verify(config, source)?;
        }
        Err(_) => {
            return Err("Config directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_data<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let data = path_resolver.data_dir();

    match data {
        Ok(data) => {
            dfs_verify(data, source)?;
        }
        Err(_) => {
            return Err("Data directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_desktop<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let desktop = path_resolver.desktop_dir();

    match desktop {
        Ok(desktop) => {
            dfs_verify(desktop, source)?;
        }
        Err(_) => {
            return Err("Desktop directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_document<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let document = path_resolver.document_dir();

    match document {
        Ok(document) => {
            dfs_verify(document, source)?;
        }
        Err(_) => {
            return Err("Document directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_download<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let download = path_resolver.download_dir();

    match download {
        Ok(download) => {
            dfs_verify(download, source)?;
        }
        Err(_) => {
            return Err("Download directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_executable<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let executable = path_resolver.executable_dir();

    match executable {
        Ok(executable) => {
            dfs_verify(executable, source)?;
        }
        Err(_) => {
            return Err("Executable directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_font<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let font = path_resolver.font_dir();

    match font {
        Ok(font) => {
            dfs_verify(font, source)?;
        }
        Err(_) => {
            return Err("Font directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_home<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let home = path_resolver.home_dir();

    match home {
        Ok(home) => {
            dfs_verify(home, source)?;
        }
        Err(_) => {
            return Err("Home directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_local_data<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let local_data = path_resolver.local_data_dir();

    match local_data {
        Ok(local_data) => {
            dfs_verify(local_data, source)?;
        }
        Err(_) => {
            return Err("Local data directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_picture<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let picture = path_resolver.picture_dir();

    match picture {
        Ok(picture) => {
            dfs_verify(picture, source)?;
        }
        Err(_) => {
            return Err("Picture directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_public<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let public = path_resolver.public_dir();

    match public {
        Ok(public) => {
            dfs_verify(public, source)?;
        }
        Err(_) => {
            return Err("Public directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_resource<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let resource = path_resolver.resource_dir();

    match resource {
        Ok(resource) => {
            dfs_verify(resource, source)?;
        }
        Err(_) => {
            return Err("Resource directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_runtime<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let runtime = path_resolver.runtime_dir();

    match runtime {
        Ok(runtime) => {
            dfs_verify(runtime, source)?;
        }
        Err(_) => {
            return Err("Runtime directory not found".to_string());
        }
    }

    Ok(())
}

fn verify_from_temp<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let temp = path_resolver.temp_dir();

    match temp {
        Ok(temp) => {
            dfs_verify(temp, source)?;
        }
        Err(_) => {
            return Err("Temp directory not found".to_string());
        }
    }

    Ok(())
}
  
fn verify_from_template<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let template = path_resolver.template_dir();

    match template {
        Ok(template) => {
            dfs_verify(template, source)?;
        }
        Err(_) => {
            return Err("Template directory not found".to_string());
        }
    }

    Ok(())
}
  
fn verify_from_video<R: Runtime>(path_resolver: &PathResolver<R>, source: &HashMap<String, ConfigValue>) -> std::result::Result<(), String> {
    let video = path_resolver.video_dir();

    match video {
        Ok(video) => {
            dfs_verify(video, source)?;
        }
        Err(_) => {
            return Err("Video directory not found".to_string());
        }
    }

    Ok(())
}
  