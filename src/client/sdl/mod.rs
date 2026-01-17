use std::time::Duration;

use sdl3::{event::Event, keyboard::Keycode, pixels::Color};

use crate::client::traits::{App, EventHandler};

pub struct AppSdl {
    pub title: &'static str,
    sdl_ctx: Option<sdl3::Sdl>,
    video_subsystem: Option<sdl3::VideoSubsystem>,
    canvas: Option<sdl3::render::Canvas<sdl3::video::Window>>,
    event_pump: Option<sdl3::EventPump>,
}

impl App for AppSdl {
    fn new() -> Self {
        Self {
            title: "EvoCell",
            sdl_ctx: None,
            video_subsystem: None,
            canvas: None,
            event_pump: None,
        }
    }

    fn init(mut self) -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(self.title, 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window.into_canvas();

        self.sdl_ctx = Some(sdl_context);
        self.video_subsystem = Some(video_subsystem);
        self.canvas = Some(canvas);
        self.event_pump = Some(self.sdl_ctx.as_ref().unwrap().event_pump().unwrap());

        self
    }

    fn render(&mut self) {
        let canvas = self.canvas.as_mut().unwrap();

        canvas.set_draw_color(Color::RGB(25, 25, 30));
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    fn update(&mut self) {}
}

impl EventHandler for AppSdl {
    fn event_handler(&mut self) -> bool {
        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }

        false
    }
}
