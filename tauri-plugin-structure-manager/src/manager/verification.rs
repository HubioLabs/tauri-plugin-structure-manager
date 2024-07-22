use std::path::PathBuf;

use tauri::{Runtime, path::PathResolver};

use super::structure::{StructureConfig, StructureItem};

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
fn dfs_verify(path: PathBuf, structure_item: &StructureItem) -> std::result::Result<(), String> {
    let mut repair = false;
    let mut strict = false;

    match &structure_item.options {
        Some(options) => {
            match options.repair {
                Some(value) => repair = value,
                None => {}
            }

            match options.strict {
                Some(value) => strict = value,
                None => {}
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
                        std::fs::create_dir_all(&dir_path).map_err(|e| format!("Failed to create directory: {:?}, error: {:?}", dir_path, e))?;
                    } else {
                        return Err(format!("Directory not found: {:?}", dir_path));
                    }
                }
                dfs_verify(dir_path, &dir)?;
            }
        }
        None => {}
    }

    Ok(())
}

/// Verifies the structure of the `appCache` directory based on the provided structure configuration.
pub fn verify_app_cache<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.app_cache_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve app cache path: {:?}", e))
    };

    match &structure_config.app_cache {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `appCache` not found".to_string())
    }
}

/// Verifies the structure of the `appConfig` directory based on the provided structure configuration.
pub fn verify_app_config<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.app_config_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve app config path: {:?}", e))
    };

    match &structure_config.app_config {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `appConfig` not found".to_string())
    }
}

/// Verifies the structure of the `audio` directory based on the provided structure configuration.
pub fn verify_audio<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.audio_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve audio path: {:?}", e))
    };

    match &structure_config.audio {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `audio` not found".to_string())
    }
}

/// Verifies the structure of the `cache` directory based on the provided structure configuration.
pub fn verify_cache<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.cache_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve cache path: {:?}", e))
    };

    match &structure_config.cache {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `cache` not found".to_string())
    }
}

/// Verifies the structure of the `config` directory based on the provided structure configuration.
pub fn verify_config<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.config_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve config path: {:?}", e))
    };

    match &structure_config.config {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `config` not found".to_string())
    }
}

/// Verifies the structure of the `data` directory based on the provided structure configuration.
pub fn verify_data<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.data_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve data path: {:?}", e))
    };

    match &structure_config.data {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `data` not found".to_string())
    }
}

/// Verifies the structure of the `desktop` directory based on the provided structure configuration.
pub fn verify_desktop<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.desktop_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve desktop path: {:?}", e))
    };

    match &structure_config.desktop {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `desktop` not found".to_string())
    }
}

/// Verifies the structure of the `document` directory based on the provided structure configuration.
pub fn verify_document<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.document_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve document path: {:?}", e))
    };

    match &structure_config.document {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `document` not found".to_string())
    }
}

/// Verifies the structure of the `download` directory based on the provided structure configuration.
pub fn verify_download<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.download_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve download path: {:?}", e))
    };

    match &structure_config.download {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `download` not found".to_string())
    }
}

/// Verifies the structure of the `executable` directory based on the provided structure configuration.
pub fn verify_executable<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.executable_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve executable path: {:?}", e))
    };

    match &structure_config.executable {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `executable` not found".to_string())
    }
}

/// Verifies the structure of the `font` directory based on the provided structure configuration.
pub fn verify_font<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.font_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve font path: {:?}", e))
    };

    match &structure_config.font {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `font` not found".to_string())
    }
}

/// Verifies the structure of the `home` directory based on the provided structure configuration.
pub fn verify_home<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.home_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve home path: {:?}", e))
    };

    match &structure_config.home {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `home` not found".to_string())
    }
}

/// Verifies the structure of the `localData` directory based on the provided structure configuration.
pub fn verify_local_data<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.local_data_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve local data path: {:?}", e))
    };

    match &structure_config.local_data {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `localData` not found".to_string())
    }
}

/// Verifies the structure of the `appData` directory based on the provided structure configuration.
pub fn verify_app_data<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.app_data_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve app data path: {:?}", e))
    };

    match &structure_config.app_data {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `appData` not found".to_string())
    }
}

/// Verifies the structure of the `appLocalData` directory based on the provided structure configuration.
pub fn verify_app_local_data<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.app_local_data_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve app local data path: {:?}", e))
    };

    match &structure_config.app_local_data {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `appLocalData` not found".to_string())
    }
}

/// Verifies the structure of the `appLog` directory based on the provided structure configuration.
pub fn verify_app_log<R: Runtime>(path_resolver: &PathResolver<R>, structure_config: &StructureConfig) -> std::result::Result<(), String> {
    let path = match path_resolver.app_log_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve app log path: {:?}", e))
    };

    match &structure_config.app_log {
        Some(structure_item) => dfs_verify(path, &structure_item),
        None => Err("Structure configuration field `appLog` not found".to_string())
    }
}
