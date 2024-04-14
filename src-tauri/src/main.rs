// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, Manager, SystemTrayEvent};

mod api_point;
use api_point::{APIPoint, UserData};

mod background_manager;
use background_manager::{WallpaperManager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn connect(api: tauri::State<'_, APIPoint>, username: String, password: String) -> Result<(), u8>  {
    api.connect(&username, &password).await.map_err(|err| err as u8)
}

#[tauri::command]
async fn get_informations(api: tauri::State<'_, APIPoint>) -> Result<UserData, u8> {
    api.get_informations().await.map_err(|err| err as u8)
}

fn main() {
    let api = APIPoint::new();
    let mut wall_manager = WallpaperManager::new(api.clone());

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);
    tauri::Builder::default()
        .manage(api)
        .setup(|_| {
            tauri::async_runtime::spawn(async move {
                let _ = wall_manager.background_task().await;
            });
            //app.get_window("main").unwrap().hide().unwrap();
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![connect, get_informations])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
