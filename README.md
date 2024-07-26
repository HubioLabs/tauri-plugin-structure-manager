# Tauri Plugin Structure Manager

A Tauri plugin for managing application structure, including directory and file creation and validation. This plugin helps ensure that the necessary project structure is maintained and allows for easy setup and verification of the application's file system.

## Features

- Create and verify directory and file structures
- Easily configurable through a JSON file
- Integrates seamlessly with Tauri applications
- Provides centralized management of project structures

## Installation

Add the plugin to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-structure-manager = "0.3.0"
```

## Usage

### Rust

In your Tauri application, register the plugin:

```rust
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(tauri_plugin_structure_manager::init())
        .setup(|app| {
            app.verify_document();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

You can call specific structure manager methodes from anywhere with an acces to the `appHandle`.

### Configuration

You can configure the structure in the `tauri.conf.json`.

```json
"plugins": {
    "structure_manager": {
      "document": {
        "files": [],
        "dirs": {
          "Hubio": {
            "options": {
              "repair": true
            },
            "files": [],
            "dirs": {
              "projects": {
                "files": [],
                "dirs": {}
              }
            }
          }
        }
      }
    }
  },
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.