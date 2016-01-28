use std::collections::HashMap;
use ::nalgebra::Pnt2;

#[derive(PartialEq,Eq,Hash)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right
}

pub struct Game {
    pub camera: Camera
}

impl Game {
    pub fn new() -> Game {
        Game {
            camera: Camera::new(),
        }
    }

    pub fn update(&mut self) {
        match self.camera.zooming {
            Zooming::In => {
                self.camera.zoom_factor *= 1.02;
            },
            Zooming::Out => {
                if self.camera.zoom_factor >= 1.0 {
                    self.camera.zoom_factor /= 1.02;
                }
            },
            _ => {}
        }

        if self.camera.panning[&Movement::Up] {
            self.camera.padding.y += self.padding_formula();
        }
        if self.camera.panning[&Movement::Down] {
            self.camera.padding.y -= self.padding_formula();
        }
        if self.camera.panning[&Movement::Left] {
            self.camera.padding.x += self.padding_formula();
        }
        if self.camera.panning[&Movement::Right] {
            self.camera.padding.x -= self.padding_formula();
        }
    }

    fn padding_formula(&self) -> f64 {
       10.0 / self.camera.zoom_factor
    }
}

pub enum Zooming {
    In,
    Out,
    No,
}
pub struct Camera {
    pub zoom_factor: f64,
    pub padding: Pnt2<f64>,
    pub panning: HashMap<Movement, bool>,
    pub zooming: Zooming,
}

impl Camera {
    pub fn new() -> Camera {
        let mut panning = HashMap::new();
        panning.insert(Movement::Left, false);
        panning.insert(Movement::Right, false);
        panning.insert(Movement::Down, false);
        panning.insert(Movement::Up, false);
        Camera {
            zoom_factor: 1.0,
            padding: Pnt2::new(0.0,0.0),
            panning: panning,
            zooming: Zooming::No,
        }
    }
}
