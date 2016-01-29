use std::f64::consts::PI;

use rand::{Rng, StdRng, SeedableRng};
use ::nalgebra::Pnt2;

use ::sdl2::pixels::Color;

pub struct Star {
    pub position: Pnt2<f64>,
    pub color: Color,
}

impl Star {
    pub fn new(position: Pnt2<f64>, color: Color) -> Star {
        Star {
            position: position,
            color: color,
        }
    }
}

const ARMS_COUNT: i32 = 3;
const ARMS_DISTANCE: f64 = 1.9 * PI / ARMS_COUNT as f64;
const ARM_MAX_OFFSET: f64 = 0.95;
const ROTATION_FACTOR: f64 = 5.4;
const RANDOM_OFFSET: f64 = 0.10;

const RADIUS: f64 = 300.0;
pub fn initialize_stars() -> Vec<Star> {
    let mut stars = Vec::new();

    let seed: &[_] = &[666,999];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in 0..38000 {
        let mut distance: f64 = rng.gen_range(0.0,1.0);
        distance = distance.powf(1.25);

        let mut angle = rng.gen_range(0.0,1.0) * 2.0 * PI;
        let mut arm_offset = rng.gen_range(0.0,1.0) * ARM_MAX_OFFSET;
        arm_offset = arm_offset - ARM_MAX_OFFSET / 2.0;
        arm_offset = arm_offset * ( 1.0 / distance);

        let mut squared_offset = arm_offset.powf(2.0);
        if arm_offset < 0.0 {
            squared_offset = squared_offset * -1.0;
        }
        arm_offset = squared_offset;

        let rotation = distance * ROTATION_FACTOR;
        angle = ((angle / ARMS_DISTANCE) as i32) as f64 * ARMS_DISTANCE + arm_offset + rotation;


        // To cartesian


        let mut star_x = angle.cos() * distance;
        let mut star_y = angle.sin() * distance;

        star_x += rng.gen_range(0.0,1.0) * RANDOM_OFFSET;
        star_y += rng.gen_range(0.0,1.0) * RANDOM_OFFSET;
        let star = Star::new(Pnt2::new(
                star_x * RADIUS,
                star_y * RADIUS
        ), Color::RGB(rng.gen_range(130,230),rng.gen_range(130,230),rng.gen_range(130,230)));
        stars.push(star);

    }
    stars
}
