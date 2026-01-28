use std::time::Duration;

use sdl3::{event::Event, pixels::Color, rect::Rect};

use crate::{
    cell::Cell,
    client::traits::{App, EventHandler},
    math::Position,
    pos,
    world::World,
};

pub struct AppSdl {
    pub title: &'static str,
    sdl_ctx: Option<sdl3::Sdl>,
    video_subsystem: Option<sdl3::VideoSubsystem>,
    canvas: Option<sdl3::render::Canvas<sdl3::video::Window>>,
    event_pump: Option<sdl3::EventPump>,
    world: World,
    scale: i32,
    speed: usize,
    mod_render: ModRender,
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
            scale: 2,
            speed: 1,
            mod_render: ModRender::Default,
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

        self.world.add(pos!(50, 100), Cell::new());

        self
    }

    fn render(&mut self) {
        let canvas = self.canvas.as_mut().unwrap();
        let _ = canvas.window_mut().set_title(
            format!(
                "Cells count: {}; Mod render: {:?}; Speed: x{}",
                self.world.count_cells(),
                self.mod_render,
                self.speed
            )
            .as_str(),
        );
        canvas.set_draw_color(Color::RGB(25, 25, 30));
        canvas.clear();

        for (pos, cell) in self.world.iter() {
            match self.mod_render {
                ModRender::Default => canvas.set_draw_color(cell.color),
                ModRender::Energy => canvas.set_draw_color((
                    (cell.energy * 10.0) as u8,
                    (cell.energy * 10.0) as u8,
                    (cell.energy * 10.0) as u8,
                )),
                ModRender::Toxin => canvas.set_draw_color((
                    (cell.toxin * 10.0) as u8,
                    (cell.toxin * 10.0) as u8,
                    (cell.toxin * 10.0) as u8,
                )),
                ModRender::Health => canvas.set_draw_color((
                    (cell.health * 10.0) as u8,
                    (cell.health * 10.0) as u8,
                    (cell.health * 10.0) as u8,
                )),
            }

            canvas
                .fill_rect(Rect::new(
                    pos.x() * self.scale,
                    pos.y() * self.scale,
                    self.scale as u32,
                    self.scale as u32,
                ))
                .unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    fn update(&mut self) {
        for _ in 0..self.speed {
            self.world.update();
        }
    }
}

impl EventHandler for AppSdl {
    fn event_handler(&mut self) -> bool {
        for event in self.event_pump.as_mut().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if let Some(k) = keycode {
                        match k.name().as_str() {
                            "D" => self.mod_render = ModRender::Default,
                            "E" => self.mod_render = ModRender::Energy,
                            "H" => self.mod_render = ModRender::Health,
                            "1" => self.speed = 1,
                            "2" => self.speed = 2,
                            "3" => self.speed = 4,
                            "4" => self.speed = 8,
                            "5" => self.speed = 16,
                            "6" => self.speed = 32,
                            "7" => self.speed = 64,
                            _ => {}
                        }
                    }
                }
                Event::Quit { .. } => return true,
                _ => {}
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
enum ModRender {
    Default,
    Energy,
    Toxin,
    Health,
}
