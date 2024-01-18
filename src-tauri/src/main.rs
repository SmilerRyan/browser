#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

#[allow(unused_must_use)]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.app_handle();
            tauri::WindowBuilder::new(
                &handle,
                "main",
                tauri::WindowUrl::External("https://www.bing.com/search?q=chat&showconv=1".parse().unwrap())
              )
              .initialization_script(include_str!("./preload.js"))
              .title("Bing Chat")
              .build().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
