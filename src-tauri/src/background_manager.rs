use wallpaper;
use std::fs;
use futures::StreamExt;
use tokio::time::{sleep, Duration};

use crate::api_point::{APIPoint};

const DOMAIN: &str = "http://127.0.0.1:5000";

#[derive(Clone)]
pub struct WallpaperManager {
    image_link: String,
    api: APIPoint
}

impl WallpaperManager {
    pub fn new(api: APIPoint) -> Self {
        Self {
            image_link: String::new(),
            api: api
        }
    }

    pub async fn background_task(&mut self) {

        loop {
            if let Ok(url) =  self.api.get_image().await {

                if self.image_link != url {
                    self.image_link = url.clone();

                    let cache_dir = dirs::cache_dir();
                    let mut file_path = cache_dir.expect("no cache dir").join("wallswapper-cache");

                    if file_path.exists() {
                        fs::remove_dir_all(file_path.clone()).unwrap();
                    }

                    fs::create_dir(file_path.clone()).unwrap();

                    file_path = file_path.join(url.clone());

                    if let Ok(mut file) = tokio::fs::File::create(&file_path).await {
                        if let Ok(request) = reqwest::get(&(DOMAIN.to_owned() + "/api/images/" + &url.as_str())).await {
                            let mut byte_stream = request.bytes_stream();

                            while let Some(item) = byte_stream.next().await {
                                if let Ok(part) = item {
                                    tokio::io::copy(&mut part.as_ref(), &mut file).await.unwrap();
                                }
                            }

                            wallpaper::set_from_path(&file_path.into_os_string().into_string().unwrap()).unwrap();
                            wallpaper::set_mode(wallpaper::Mode::Fit).unwrap();
                        }
                    }
                }
            }
            sleep(Duration::from_secs(60)).await;
        }
    }
}