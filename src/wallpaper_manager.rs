use keyring::{Entry};
    use wallpaper;
use std::thread;
use std::time::Duration;
use reqwest::{StatusCode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


const SERVER_URL: &str = "http://127.0.0.1:5000";


#[derive(Clone)]
pub struct WallpaperManager {
    storable: bool,
    connected: Arc<Mutex<bool>>,
    token: Arc<Mutex<String>>
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
                            connected:Arc::new(Mutex::new(true)),
                            token:Arc::new(Mutex::new(token))
                        }
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }

        Self {
            storable:storable,
            connected:Arc::new(Mutex::new(false)),
            token:Arc::new(Mutex::new(String::new()))
        }
    }

    pub fn is_connected(&self) -> bool {
        return *self.connected.lock().unwrap();
    }

    pub fn connect(&mut self, username: &str, password: &str, save: bool) -> bool{

        let mut form = HashMap::new();
        form.insert("username", username);
        form.insert("password", password);

        let client = reqwest::blocking::Client::new();
        let res = client.post(SERVER_URL.to_owned() + "/api/connect")
            .json(&form)
            .send();

        match res {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        if let Ok(text) = resp.text() {
                            println!("{:?}", text);
                            {
                                let mut token = self.token.lock().unwrap();
                                *token = text.to_string();

                                let mut connected = self.connected.lock().unwrap();
                                *connected = true;
                            }
                        }
                    },
                    StatusCode::UNAUTHORIZED => {
                        return false;
                    },
                    _ => {
                        println!("Nya Error");
                    }
                }
            },
            Err(_) => {
                println!("Error connect");
            }
        }

        if self.is_connected() && save && self.storable {
            match Entry::new("WallSwapper", "User") {
                Ok(storage) => {
                    let _ = storage.set_password(self.token.lock().unwrap().as_str());
                },
                Err(_) => {},
            }
        };

        self.is_connected()
    }

    pub fn background_task(&self) {

        loop {
            println!("{}", self.is_connected());
            if self.is_connected() {
                // Request for background

                {
                    let mut form = HashMap::new();
                    let token = self.token.lock().unwrap();
                    form.insert("token", token.as_str());

                    //let mut url = String::from("https://wallswapper.luneko.dev/api/update/");

                    let client = reqwest::blocking::Client::new();
                    let res = client.post(SERVER_URL.to_owned() + "/api/update")
                        .json(&form)
                        .send();

                    match res {
                        Ok(resp) => {
                            match resp.status() {
                                StatusCode::OK => {
                                    if let Ok(url) = resp.text() {
                                        println!("{:?}", &(SERVER_URL.to_owned() + "/api/images/" + &url.as_str()));
                                        wallpaper::set_from_url(&(SERVER_URL.to_owned() + "/api/images/" + &url.as_str())).unwrap();
                                        wallpaper::set_mode(wallpaper::Mode::Fit).unwrap();
                                    }
                                },
                                _ => {
                                    println!("Nya Error");
                                }
                            }
                        },
                        Err(_) => {
                            println!("Error get");
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    }
}
