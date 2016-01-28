extern crate sdl2;
extern crate rand;
extern crate nalgebra;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

mod galaxy;
fn main() {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("space", 1280,1024)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let window_size = window.size();
    let mut renderer = window.renderer().build().unwrap();

    let mut event_pump = context.event_pump().unwrap();

    let stars = galaxy::initialize_stars();

    let zoom_factor = 1.0;


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown{keycode, repeat, ..} => {
                    if !repeat { println!("down {:?}", keycode); };
                },
                Event::KeyUp{keycode, ..} => {
                    println!("up {:?}", keycode);
                },
                _ => {}
            }
        }

        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255,255,255));
        for star in &stars {
            renderer.draw_point(Point::new(
                    (zoom_factor * star.x) as i32 + (window_size.0 / 2) as i32,
                    (zoom_factor * star.y) as i32 + (window_size.1 / 2) as i32)
                );
        }
        renderer.present();
    }
}
