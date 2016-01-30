extern crate sdl2;
extern crate sdl2_ttf;
extern crate rand;
extern crate nalgebra;

use nalgebra::Pnt2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::Mouse;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2_ttf::Font;

mod game;
mod galaxy;

use galaxy::Star;

use game::Movement;
use game::Game;
use game::Zooming;

macro_rules! write {
    ($text:expr, $font:expr, $renderer:expr) => {
        {
        let t = $font.render($text).blended(Color::RGB(250,100,100)).unwrap();
        let tex = $renderer.create_texture_from_surface(t).unwrap();
        $renderer.copy(&tex, None, Some(Rect::new_unwrap(10,10,tex.query().width,tex.query().height)));
        }
    };
}

fn main() {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let ttf = sdl2_ttf::init().unwrap();
    let font = ttf.load_font(std::path::Path::new("fonts/lato.ttf"), 16).unwrap();

    let window = video_subsystem.window("space", 1280,1024)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let window_size = window.size();
    let mut renderer = window.renderer().build().unwrap();
    renderer.set_blend_mode(sdl2::render::BlendMode::Add);

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
                Event::MouseButtonDown{mouse_btn,..} => {
                    if mouse_btn == Mouse::Left {
                        game.maybe_select();
                    };
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

            if let Some(star) = game.selected {
                if star.position == star_pnt {
                    let c = star.color.rgb();
                    renderer.set_draw_color(Color::RGBA(c.0,c.1,c.2, 100));
                    renderer.fill_rect(Rect::new_unwrap(draw_point.x() - 10, draw_point.y() - 10, 20,20));
                }
            };
            renderer.draw_point(draw_point);
        }


        if game.hovered.is_some() {
            maybe_render_star_data(game.hovered, &font, &mut renderer);
        } else {
            maybe_render_star_data(game.selected, &font, &mut renderer);
        };
        renderer.present();
    }
}

fn maybe_render_star_data(star: Option<&Star>, font: &Font, renderer: &mut Renderer) {
    if let Some(star) = star {
        write!(format!("{}\nX: {}, Y: {}", star.name,star.position.x, star.position.y).as_str(),
        font, renderer);
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
            game.camera.panning.insert(Movement::Left, down);
        },
        Keycode::Right => {
            game.camera.panning.insert(Movement::Right, down);
        },
        Keycode::Up => {
            game.camera.panning.insert(Movement::Up, down);
        },
        Keycode::Down => {
            game.camera.panning.insert(Movement::Down, down);
        },
        Keycode::M => {
            if !down {
                game.target_nearby_star();
            }
        },
        Keycode::LShift | Keycode::RShift => {
            game.camera.shift = down
        },
        _ => {}
    }
}
