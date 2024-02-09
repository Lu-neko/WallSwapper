use keyring::{Entry, Result};
use wallpaper;
use std::thread;
use std::time::Duration;

let connected = false;
let token = "";

let storage = Entry::new("WallSwapper", "User")?;

match storage.get_password(){
    Ok(stored_token) => {
        connected=true;
        token = stored_token;
    },
    Err(_) => break,
}

fn is_connected() -> Result<bool> {
    return connected;
}

fn connect(username: &str, password: &str) -> Result <(), Err()> {
    // Verify if the creditentials are good, get the token
    // Or return an error
    let connect_token = "Salut";
    token = connect_token;
    connected = true;
    storage.set_password(connect_token)?;
}

fn background_task() -> Result<(), Err()> {

    loop {
        if (connected){
            // Request for background
            let url = "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSgqQAGfEEbIV5AYM6_BlOQ-M73TKzuEBqYDJGn7Jih6g&s";
            
            wallpaper::set_from_url(url).unwrap();
            wallpaper::set_mode(wallpaper::Mode::Fit).unwrap();
        }
        thread::sleep(Duration::from_secs(60));
    }

    Ok(())
}