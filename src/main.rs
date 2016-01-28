extern crate sdl2;
extern crate rand;
extern crate nalgebra;

use std::f64::consts::PI;

use rand::{Rng, StdRng, SeedableRng};

use nalgebra::Pnt2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
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

    let stars = initialize_stars();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }

        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255,255,255));
        for star in &stars {
            renderer.draw_point(Point::new(
                    star.x as i32 + (window_size.0 / 2) as i32
                    ,star.y as i32 + (window_size.1 / 2) as i32)
                );
        }
        renderer.present();
    }
}

fn initialize_stars() -> Vec<Pnt2<f64>> {
    let mut stars = Vec::new();

    let seed: &[_] = &[666,999];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for i in 0..10000 {
        let distance = rng.gen_range(0.0,300.0);

        let angle = rng.gen_range(0.0,1.0) * 2.0 * PI;

        // To cartesian
        stars.push(Pnt2::new(
            angle.cos() * distance,
            angle.sin() * distance
        ));

    }
    stars
}
