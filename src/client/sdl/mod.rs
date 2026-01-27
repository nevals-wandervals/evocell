use std::time::Duration;

use sdl3::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

use crate::{
    cell::Cell,
    client::traits::{App, EventHandler},
    math::Position,
    world::{HEIGHT, WIDTH, World},
};

pub struct AppSdl {
    pub title: &'static str,
    sdl_ctx: Option<sdl3::Sdl>,
    video_subsystem: Option<sdl3::VideoSubsystem>,
    canvas: Option<sdl3::render::Canvas<sdl3::video::Window>>,
    event_pump: Option<sdl3::EventPump>,
    world: World,
}

impl App for AppSdl {
    fn new() -> Self {
        Self {
            title: "EvoCell",
            sdl_ctx: None,
            video_subsystem: None,
            canvas: None,
            event_pump: None,
            world: World::new(),
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

        self.world.add(Position::new(50, 50), Cell::new());

        self
    }

    fn render(&mut self) {
        let canvas = self.canvas.as_mut().unwrap();
        let _ = canvas.window_mut().set_title(format!("Cells count: {}", self.world.count_cells()).as_str());
        canvas.set_draw_color(Color::RGB(25, 25, 30));
        canvas.clear();

        for (pos, cell) in self.world.iter() {
            canvas.set_draw_color(cell.color);
            canvas.fill_rect(Rect::new(pos.x() + WIDTH, pos.y() + HEIGHT, 1, 1)).unwrap();
        }

        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    fn update(&mut self) {
        self.world.update();
    }
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
