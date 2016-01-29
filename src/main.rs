extern crate sdl2;
extern crate rand;
extern crate nalgebra;

use nalgebra::Pnt2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;

mod game;
mod galaxy;

use galaxy::Star;

use game::Movement;
use game::Game;
use game::Zooming;
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

    let mut game = Game::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown{keycode, repeat, ..} => {
                    if !repeat && keycode.is_some() { handle_input(true, keycode.unwrap(), &mut game); };
                },
                Event::KeyUp{keycode, ..} => {
                    if keycode.is_some() { handle_input(false, keycode.unwrap(), &mut game); }
                },
                Event::MouseMotion{x,y,..} => {
                },
                _ => {}
            }
        }

        game.update();

        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();
        let zoom_point = game.camera.padding;
        game.hovered = None;
        for star in &stars {
            renderer.set_draw_color(star.color);
            let star_pnt = star.position;
            let draw_point = Point::new(
                    (game.camera.zoom_factor * (star_pnt.x + zoom_point.x)) as i32 +
                    (window_size.0 / 2) as i32 ,

                    (game.camera.zoom_factor * (star_pnt.y + zoom_point.y)) as i32 +
                    (window_size.1 / 2) as i32 );

            let mstate = context.mouse().mouse_state();
            if game.hovered.is_none() && mstate.1 >= draw_point.x() - 10 && mstate.1 <= draw_point.x() + 10
            && mstate.2 >= draw_point.y() - 10 && mstate.2 <= draw_point.y() +10 {
                game.hovered = Some(&star);
                renderer.draw_rect(Rect::new_unwrap(draw_point.x() - 10, draw_point.y() - 10, 20,20));
            }
            renderer.draw_point(draw_point);
        }
        renderer.present();
    }
}

fn handle_input(down: bool, keycode: Keycode, game: &mut Game) {
    match keycode {
        Keycode::Plus => {
            game.camera.zooming = if down { Zooming::In }
            else { Zooming::No }
        },
        Keycode::Minus => {
            game.camera.zooming = if down { Zooming::Out }
            else { Zooming::No }
        },
        Keycode::Left => {
            game.camera.panning.insert(Movement::Left, if down { true }
            else { false });
        },
        Keycode::Right => {
            game.camera.panning.insert(Movement::Right, if down { true }
            else { false });
        },
        Keycode::Up => {
            game.camera.panning.insert(Movement::Up,  if down { true }
            else { false });
        },
        Keycode::Down => {
            game.camera.panning.insert(Movement::Down, if down { true }
            else { false });
        },

        Keycode::LShift | Keycode::RShift => {
            game.camera.shift = down
        },
        _ => {}
    }
}
