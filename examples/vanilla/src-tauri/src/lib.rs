use tauri_plugin_structure_manager::StructureManagerExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_structure_manager::init())
    .setup(|app| {
      app.verify_document()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
