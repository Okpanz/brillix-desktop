#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_deep_link::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;

        // Runtime scheme registration is only meaningful on Windows/Linux for
        // `cargo run`-style dev builds without a real app bundle. On macOS the
        // bundled .app's Info.plist (embedded at build time from
        // tauri.conf.json's plugins.deep-link config) already registers
        // brillix:// — calling register() again here crashed on launch.
        #[cfg(any(windows, target_os = "linux"))]
        {
          use tauri_plugin_deep_link::DeepLinkExt;
          app.deep_link().register("brillix")?;
        }
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
