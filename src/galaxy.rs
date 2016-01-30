use std::f64::consts::PI;

use rand::{Rng, StdRng, SeedableRng};
use ::nalgebra::Pnt2;

use ::sdl2::pixels::Color;

pub struct Star {
    pub position: Pnt2<f64>,
    pub z: usize,
    pub color: Color,
    pub name: String,
    pub sector: Pnt2<i32>
}

const SYLLABLES: [&'static str; 13] = ["um", "za", "dor", "lim", "fal", "saf", "ele", "ziu", "jin", "rou", "wer", "ni", "le"];
impl Star {
    pub fn new(position: Pnt2<f64>, z: usize, sector: Pnt2<i32>, color: Color) -> Star {
        Star {
            position: position,
            z: z,
            sector: sector,
            color: color,
            name: "UNNAMED THIS IS A BUG".to_string(), //XXX
        }
    }

    pub fn gen_name(&mut self) {
        let seed: &[_] = &[self.position.x as usize, self.position.y as usize,
        self.color.rgb().0 as usize, self.color.rgb().1 as usize, self.color.rgb().2 as usize];

        let mut rng: StdRng = SeedableRng::from_seed(seed);


        self.name = String::new();

        for _ in 0..rng.gen_range(2,5) {
            let i = rng.gen_range(0,12);
            self.name = format!("{}{}",self.name,SYLLABLES[i]);
        };
    }
}

const ARMS_COUNT: i32 = 3;
const ARMS_DISTANCE: f64 = 1.9 * PI / ARMS_COUNT as f64;
const ARM_MAX_OFFSET: f64 = 0.95;
const ROTATION_FACTOR: f64 = 5.4;
const RANDOM_OFFSET: f64 = 0.10;

const RADIUS: f64 = 300.0;
pub fn initialize_stars() -> Vec<Vec<Star>> {
    let mut all_stars = Vec::new();

    for i in 0..80 {
        let mut stars = Vec::new();

        let seed: &[_] = &[666,999, i as usize];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        for _ in 0..5000 {
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

            let cart_pos = Pnt2::new(star_x * RADIUS,
                                     star_y * RADIUS);

            let mut star = Star::new( cart_pos, i,
                                      Pnt2::new((cart_pos.x / 10.0) as i32,
                                      (cart_pos.y / 10.0) as i32),
                                      Color::RGB(rng.gen_range(130,230),rng.gen_range(130,230),rng.gen_range(130,230)));
            //star.gen_name();


            stars.push(star);

        }
        all_stars.push(stars);
    }
    all_stars
}
