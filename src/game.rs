use std::collections::HashMap;
use ::nalgebra::Pnt2;

use ::galaxy::Star;

#[derive(PartialEq,Eq,Hash)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right
}

pub struct Game<'a> {
    pub camera: Camera,

    pub hovered: Option<&'a Star>,
    pub selected: Option<&'a Star>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            camera: Camera::new(),
            hovered: None,
            selected: None,
        }
    }

    // Triggers on a left-click on galaxy map
    pub fn maybe_select(&mut self) {
        if let Some(star) = self.hovered {
            self.selected = Some(star);
        } else if self.selected.is_some() {
            self.selected = None;
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
        let mut a = 4.0;
        if self.camera.shift { a += 4.5; }
        a / self.camera.zoom_factor
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
    pub shift: bool,
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
            shift: false,
            zooming: Zooming::No,
        }
    }
}
