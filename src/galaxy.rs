use std::f64::consts::PI;

use rand::{Rng, StdRng, SeedableRng};
use ::nalgebra::Pnt2;

const ARMS_COUNT: i32 = 3;
const ARMS_DISTANCE: f64 = 2.0 * PI / ARMS_COUNT as f64;
const ARM_MAX_OFFSET: f64 = 0.65;
const ROTATION_FACTOR: f64 = 6.4;
const RANDOM_OFFSET: f64 = 0.04;

const RADIUS: f64 = 300.0;
pub fn initialize_stars() -> Vec<Pnt2<f64>> {
    let mut stars = Vec::new();

    let seed: &[_] = &[666,999];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    for _ in 0..10000 {
        let mut distance: f64 = rng.gen_range(0.0,1.0);
        distance = distance.powf(2.0);

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
        stars.push(Pnt2::new(
                star_x * RADIUS,
                star_y * RADIUS
        ));

    }
    stars
}
