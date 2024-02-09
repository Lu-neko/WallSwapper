use wallpaper;
use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

enum Message {
    Quit,
    Green,
    Red,
}

fn main() {
    let mut tray = TrayItem::new(
        "WallSwapper",
        IconSource::Resource("name-of-icon-in-rc-file"),
    )
    .unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let red_tx = tx.clone();
    tray.add_menu_item("Red", move || {
        wallpaper::set_from_url("https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSgqQAGfEEbIV5AYM6_BlOQ-M73TKzuEBqYDJGn7Jih6g&s").unwrap();
        wallpaper::set_mode(wallpaper::Mode::Fit).unwrap();
        red_tx.send(Message::Red).unwrap();
    })
    .unwrap();

    let green_tx = tx.clone();
    tray.add_menu_item("Green", move || {
        wallpaper::set_from_url("https://browsecat.art/sites/default/files/animated-nature-background-126761-1031169-5603413.png").unwrap();
        wallpaper::set_mode(wallpaper::Mode::Fit).unwrap();
        green_tx.send(Message::Green).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::Red) => {
                println!("Red");
                tray.set_icon(IconSource::Resource("another-name-from-rc-file"))
                    .unwrap();
            }
            Ok(Message::Green) => {
                println!("Green");
                tray.set_icon(IconSource::Resource("name-of-icon-in-rc-file"))
                    .unwrap()
            }
            _ => {}
        }
    }
}