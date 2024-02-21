use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use std::thread;
use std::time::Duration;

use crate::wallpaper_manager::WallpaperManager;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Start,
    Update,
    Quit,
}

#[derive(Clone)]
pub struct ApplicationManager {
    receiver: fltk::app::Receiver<Message>,
    sender: fltk::app::Sender<Message>,
    wallpaper: WallpaperManager
}

impl ApplicationManager {
    pub fn new(wallpaper: WallpaperManager) -> Self {
        let (sender, receiver) = app::channel::<Message>();

        Self {
            receiver:receiver,
            sender:sender,
            wallpaper:wallpaper
        }
    }

    pub fn background_task(&self){
        loop {
            match self.receiver.recv() {
                Some(Message::Start) => {
                    let result = self.run();
                    if result {
                        return;
                    }
                },
                Some(Message::Quit) => {
                    return;
                },
                _ => {}
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn send(&self, message: Message){
        self.sender.send(message);
    }

    fn run(&self) -> bool{
        let app = app::App::default();
        let mut wind = Window::new(100, 100, 400, 300, "WallSwapper");
        let mut but = Button::new(160, 210, 80, 40, "Connect!");
        let mut _frame = Frame::new(0, 0, 400, 200, "");
        wind.end();
        wind.show();

        let mut wall = self.wallpaper.clone();
        let appli = self.clone();
        but.set_callback(move |_| {

            wall.connect("lune", "neko", false);
            appli.send(Message::Update);
        });

        while app.wait() {
            match self.receiver.recv() {
                Some(Message::Update) => {
                    
                },
                Some(Message::Quit) => {
                    return true;
                },
                _ => {}
            }
        }

        app.quit();

        false
    }
}