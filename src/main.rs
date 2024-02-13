use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};
use std::thread;

mod wallpaper_manager;
mod application;

enum Message {
    Quit,
    Open,
}

fn main() {
    let mut tray = TrayItem::new(
        "WallSwapper",
        IconSource::Resource("name-of-icon-in-rc-file"),
    )
    .unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let open_tx = tx.clone();
    tray.add_menu_item("Open App", move || {
        open_tx.send(Message::Open).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    let manager = wallpaper_manager::WallpaperManager::new();

    let manage = manager.clone();

    thread::spawn(move || {
        // Run wallpaper manager in the background
        println!("{}", manage.is_connected());
        manage.background_task();
    });

    let appli = application::ApplicationManager::new(manager);

    let app = appli.clone();

    thread::spawn(move || {
        app.background_task()
    });


    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                // Shutdown the background task too I guess
                appli.send(application::Message::Quit);
                break;
            }
            Ok(Message::Open) => {

                println!("Open!");

                appli.send(application::Message::Start);
                // Open app
                
                // Try to receive at the start, and if quit exit

                // Verify if is connected, if yes, show buttons to create links,
                // If not, show connection panel

                // If exited, just stay as the background task and tray
            }
            _ => {}
        }
    }
}