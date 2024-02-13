use keyring::{Entry};
    use wallpaper;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct WallpaperManager {
    storable: bool,
    connected: bool,
    token: String
}

impl WallpaperManager {
    pub fn new() -> Self {
        let mut storable = false;

        match Entry::new("WallSwapper", "User") {
            Ok(storage) => {
                storable = true;
                match storage.get_password(){
                    Ok(token) => {
                        return Self {
                            storable:storable,
                            connected:true,
                            token:token
                        }
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }

        Self {
            storable:storable,
            connected:false,
            token:String::new()
        }
    }

    pub fn is_connected(&self) -> bool {
        return self.connected;
    }

    pub fn connect(&self, _username: &str, _password: &str, save: bool) -> bool{

        println!("Nya!");
        // Connect
        let token = "test";
        let connected = true;

        if connected && save && self.storable {
            match Entry::new("WallSwapper", "User") {
                Ok(storage) => {let _ = storage.set_password(token);},
                Err(_) => {},
            }
        };

        connected
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
