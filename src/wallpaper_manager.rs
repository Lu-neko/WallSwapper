use keyring::{Entry};
    use wallpaper;
use std::thread;
use std::time::Duration;

pub struct WallpaperManager {
    storage: Option<Entry>,
    connected: bool,
    token: String
}

impl WallpaperManager {
    pub fn new() -> Self {
        let mut connected = false;
        let mut token = String::from("");

        let storage;

        match Entry::new("WallSwapper", "User") {
            Ok(storage_result) => {storage = storage_result},
            Err(_) => {
                return Self {
                    storage:None,
                    connected:false,
                    token:String::new()
                };
            },
        }

        match storage.get_password(){
            Ok(stored_token) => {
                connected=true;
                token = stored_token;
            },
            Err(_) => {},
        }

        Self {
            storage:Some(storage),
            connected:connected,
            token:token.to_string()
        }
    }

    pub fn is_connected(&self) -> bool {
        return self.connected;
    }

    pub fn connect(&self, _username: &str, _password: &str, save: bool){
        // Connect
        let token = "test";
        let connected = true;

        if connected && save {
            let _ = match &self.storage {
                Some(storage) => storage.set_password(token),
                None => Ok({}),
            };
        };
    }

    pub fn background_task(&self) {

        loop {
            if self.connected {
                // Request for background
                let mut url = String::from("https://wallswapper.luneko.dev/api/update/");

                url.push_str(&self.token);
                
                wallpaper::set_from_url(&url).unwrap();
                wallpaper::set_mode(wallpaper::Mode::Fit).unwrap();
            } else {
                continue;
            }
            thread::sleep(Duration::from_secs(60));
        }
    }
}
/*
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

pub fn background_task() -> Result<(), Err()> {

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
*/