use keyring::{Entry};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use reqwest::{StatusCode};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Serialize, Deserialize};

#[repr(u8)]
pub enum ApiError {
    ConnectionError,
    InvalidToken,
    InvalidCredentials,
    UnknownError,
    NotConnected
}

const DOMAIN: &str = "http://127.0.0.1:5000";

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum State {
    FriendRequestSent = 0,
    FriendRequestReceived = 1,
    Friends = 2,
    Blocked = 3
}

#[derive(Serialize, Deserialize)]
pub struct FriendData {
    username: String,
    state: State,
    picture: String,
    waiting: bool
}

#[derive(Serialize, Deserialize)]
pub struct LinkData {
    url: String,
    usages: u32,
    max_uses: Option<u32>,
    created: u64,
    end: u64
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    username: String,
    friends: Vec<FriendData>,
    links: Vec<LinkData>
}

#[derive(Clone)]
pub struct APIPoint {
    storable: bool,
    connected: Arc<Mutex<bool>>,
    token: Arc<Mutex<String>>,
    client: reqwest::Client,
}

impl APIPoint {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        let mut storable = false;
        let mut token = String::new();
        let mut connected = false;

        match Entry::new("WallSwapper", "User") {
            Ok(storage) => {
                storable = true;
                match storage.get_password(){
                    Ok(content) => {
                        token = content;
                        connected = true;
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }

        Self {
            storable:storable,
            connected:Arc::new(Mutex::new(connected)),
            token:Arc::new(Mutex::new(token)),
            client: client
        }
    }

    pub async fn get_informations(&self) -> Result<UserData, ApiError> {
        if !*self.connected.lock().unwrap() {
            return Err(ApiError::NotConnected);
        }

        let mut map = HashMap::new();
        map.insert("token", self.token.lock().unwrap().clone());

        let req = self.client.get(DOMAIN.to_owned() + "/api/get_informations")
            .json(&map)
            .send();

        match req.await {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        if let Ok(user_data) = resp.json::<UserData>().await {
                            println!("{:?}", user_data.username);
                            return Ok(user_data);
                        }
                        return Err(ApiError::UnknownError);
                    },
                    StatusCode::UNAUTHORIZED => {
                        self.disconnect().await;
                        return Err(ApiError::InvalidToken);
                    },
                    _ => {
                        return Err(ApiError::UnknownError)
                    }
                }
            },
            Err(_) => {
                return Err(ApiError::ConnectionError);
            }
        }
    }

    pub async fn get_image(&self) -> Result<String, ApiError> {
        if !*self.connected.lock().unwrap() {
            return Err(ApiError::NotConnected);
        }

        let mut map = HashMap::new();
        map.insert("token", self.token.lock().unwrap().clone());

        let req = self.client.post(DOMAIN.to_owned() + "/api/update")
            .json(&map)
            .send();

        match req.await {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        if let Ok(link) = resp.text().await {
                            return Ok(link);
                        }
                        return Err(ApiError::UnknownError);
                    },
                    StatusCode::UNAUTHORIZED => {
                        self.disconnect().await;
                        return Err(ApiError::InvalidToken);
                    },
                    _ => {
                        return Err(ApiError::UnknownError)
                    }
                }
            },
            Err(_) => {
                return Err(ApiError::ConnectionError);
            }
        }
    }

    pub async fn disconnect(&self) {
        *self.connected.lock().unwrap() = false;
        match Entry::new("WallSwapper", "User") {
            Ok(storage) => {
                let _ = storage.delete_password();
            },
            Err(_) => {},
        }
    }

    pub async fn connect(&self, username: &str, password: &str) -> Result<(), ApiError> {
        let mut map = HashMap::new();
        map.insert("username", username);
        map.insert("password", password);

        let req = self.client.post(DOMAIN.to_owned() + "/api/connect")
            .json(&map)
            .send();

        match req.await {
            Ok(resp) => {
                match resp.status() {
                    StatusCode::OK => {
                        if let Ok(text) = resp.text().await {
                            println!("{:?}", text);
                            *self.connected.lock().unwrap() = true;
                            *self.token.lock().unwrap() = text.clone();

                            if self.storable {
                                match Entry::new("WallSwapper", "User") {
                                    Ok(storage) => {
                                        let _ = storage.set_password(&text);
                                    },
                                    Err(_) => {},
                                }
                            }

                            return Ok(());
                        }
                        return Err(ApiError::UnknownError);
                    },
                    StatusCode::UNAUTHORIZED => {
                        return Err(ApiError::InvalidCredentials);
                    },
                    _ => {
                        return Err(ApiError::UnknownError)
                    }
                }
            },
            Err(_) => {
                return Err(ApiError::ConnectionError);
            }
        }
    }
}