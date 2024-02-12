use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};
use std::thread;

mod wallpaper_manager;

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

    let red_tx = tx.clone();
    tray.add_menu_item("Open App", move || {
        red_tx.send(Message::Open).unwrap();
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

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                // Shutdown the background task too I guess
                println!("Quit");
                break;
            }
            Ok(Message::Open) => {
                // Open app
                loop {
                    // Try to receive at the start, and if quit exit

                    // Verify if is connected, if yes, show buttons to create links,
                    // If not, show connection panel

                    // If exited, just stay as the background task and tray
                }
            }
            _ => {}
        }
    }
}