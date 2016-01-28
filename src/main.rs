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

const arms_count: i32 = 3;
const arms_distance: f64 = 2.0 * PI / arms_count as f64;
const arm_max_offset: f64 = 0.65;
const rotation_factor: f64 = 6.4;
const random_offset: f64 = 0.04;

const radius: f64 = 300.0;
fn initialize_stars() -> Vec<Pnt2<f64>> {
    let mut stars = Vec::new();

    let seed: &[_] = &[666,999];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in 0..10000 {
        let mut distance: f64 = rng.gen_range(0.0,1.0);
        distance = distance.powf(2.0);

        let mut angle = rng.gen_range(0.0,1.0) * 2.0 * PI;
        let mut arm_offset = rng.gen_range(0.0,1.0) * arm_max_offset;
        arm_offset = arm_offset - arm_max_offset / 2.0;
        arm_offset = arm_offset * ( 1.0 / distance);

        let mut squared_offset = arm_offset.powf(2.0);
        if arm_offset < 0.0 {
            squared_offset = squared_offset * -1.0;
        }
        arm_offset = squared_offset;

        let rotation = distance * rotation_factor;
        angle = ((angle / arms_distance) as i32) as f64 * arms_distance + arm_offset + rotation;


        // To cartesian


        let mut star_x = (angle.cos() * distance);
        let mut star_y = (angle.sin() * distance);

        star_x += rng.gen_range(0.0,1.0) * random_offset;
        star_y += rng.gen_range(0.0,1.0) * random_offset;
        stars.push(Pnt2::new(
                star_x * radius,
                star_y * radius
        ));

    }
    stars
}
