use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

/// Represents the options for a structure item.
///
/// By default, a None value is considered as false.
#[derive(Deserialize, Clone, Debug)]
pub struct StructureItemOptions {
    /// If set to true, the directory will be created if it does not exist.
    pub repair: Option<bool>,
    /// If set to true, the contents need to be exactly the same.
    ///
    /// By default, `StructureManager` will only verify if directories and files from the `StructureConfig` exist,
    /// but ignore extra files and directories.
    ///
    /// If `strict` is set to true, the contents of the directory (`StructureItem`) need to be exactly the same.
    pub strict: Option<bool>,
}

/// Represents an item in the structure (a directory in the OS), which can contain options, files, and directories.
#[derive(Deserialize, Clone, Debug)]
pub struct StructureItem {
    /// The options for the structure item.
    pub options: Option<StructureItemOptions>,
    /// The list of files in the structure item.
    pub files: Option<Vec<String>>,
    /// The list of directories in the structure item.
    pub dirs: Option<HashMap<String, StructureItem>>,
}

/// Represents the structure configuration.
#[derive(Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructureConfig {
    pub app_cache: Option<StructureItem>,
    pub app_config: Option<StructureItem>,
    pub app_data: Option<StructureItem>,
    pub app_local_data: Option<StructureItem>,
    pub app_log: Option<StructureItem>,
    pub audio: Option<StructureItem>,
    pub cache: Option<StructureItem>,
    pub config: Option<StructureItem>,
    pub data: Option<StructureItem>,
    pub desktop: Option<StructureItem>,
    pub document: Option<StructureItem>,
    pub download: Option<StructureItem>,
    pub executable: Option<StructureItem>,
    pub font: Option<StructureItem>,
    pub home: Option<StructureItem>,
    pub local_data: Option<StructureItem>,
    pub picture: Option<StructureItem>,
    pub public: Option<StructureItem>,
    pub resource: Option<StructureItem>,
    pub runtime: Option<StructureItem>,
    pub temp: Option<StructureItem>,
    pub template: Option<StructureItem>,
    pub video: Option<StructureItem>,
}
