mod utils;

use core::fmt;
use std::{f32::consts::PI, ops::Deref};

use crate::utils::*;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct DirectionVector {
    dx: f32,
    dy: f32,
    rad: f32,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Bird {
    coord_x: f32,
    coord_y: f32,
    direction: DirectionVector,
}

#[wasm_bindgen]
pub struct Area {
    width: f32,
    height: f32,
    birds: Vec<Bird>,
}

#[wasm_bindgen]
pub struct Point {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Area {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Area {
        let width = 1000.0;
        let height = 800.0;
        let birds = Vec::new();

        Area {
            width,
            height,
            birds,
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn add_bird(&mut self) {
        let int_radius: f32 = RADIUS.round().abs();
        let x = rand::thread_rng().gen_range(int_radius..self.width - int_radius);
        let y = rand::thread_rng().gen_range(int_radius..self.height - int_radius);

        let bird: Bird = Bird::new(x, y, DirectionVector::random());
        self.birds.push(bird);
    }

    pub fn tick(&mut self) {
        let birds_clone = self.birds.clone();
        let mut tmp = vec![];
        for (i, bird) in self.birds.iter_mut().enumerate() {
            let pt: Point = bird.direction_line_stop();

            if pt.x < 0.0 || pt.x >= self.width {
                bird.wall_bounce(HORIZONTAL_BOUNCE);
            } else if pt.y < 0.0 || pt.y >= self.height {
                bird.wall_bounce(VERTICAL_BOUNCE);
            } else {
                bird.fly();
            }
            tmp.push(bird.clone());
        }

        self.birds = tmp;
    }

    pub fn get_birds(&self) -> Vec<Bird> {
        self.birds.to_vec()
    }

    pub fn nb_birds(&self) -> usize {
        self.birds.len()
    }

    pub fn bird_radius(&self) -> f32 {
        RADIUS
    }
}

#[wasm_bindgen]
impl Bird {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, direction: DirectionVector) -> Bird {
        Bird {
            coord_x: x,
            coord_y: y,
            direction: direction,
        }
    }

    pub fn coord_x(&self) -> f32 {
        self.coord_x
    }

    pub fn coord_y(&self) -> f32 {
        self.coord_y
    }

    pub fn direction(&self) -> DirectionVector {
        self.direction.clone()
    }

    pub fn direction_line_stop(&self) -> Point {
        Point {
            x: self.coord_x + self.direction.norm_cos(),
            y: self.coord_y + self.direction.norm_sin(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "[x: {}; y: {}; direction: {}]",
            self.coord_x,
            self.coord_y,
            self.direction.to_string()
        )
    }

    fn fly(&mut self) {
        self.direction.randomize_in_range();
        self.update_coordinates();
    }

    fn wall_bounce(&mut self, fact: f32) {
        self.direction.invert(fact);
        self.direction.randomize_in_range();
        self.update_coordinates();
    }

    fn update_coordinates(&mut self) {
        self.coord_x += self.direction.cos() * SPEED;
        self.coord_y += self.direction.sin() * SPEED;
    }
}

#[wasm_bindgen]
impl DirectionVector {
    pub fn dx(&self) -> f32 {
        self.dx
    }

    pub fn dy(&self) -> f32 {
        self.dy
    }

    pub fn rad(&self) -> f32 {
        self.rad
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.dx, self.dy)
    }

    pub fn random() -> DirectionVector {
        let rand_angle: f32 = rand::thread_rng().gen_range(0.0..=2.0 * PI);
        DirectionVector {
            dx: rand_angle.cos(),
            dy: rand_angle.sin(),
            rad: rand_angle,
        }
    }

    pub fn cos(&self) -> f32 {
        self.rad.cos()
    }

    pub fn sin(&self) -> f32 {
        self.rad.sin()
    }

    pub fn norm_cos(&self) -> f32 {
        self.cos() * RADIUS
    }

    pub fn norm_sin(&self) -> f32 {
        self.sin() * RADIUS
    }

    pub fn randomize_in_range(&mut self) {
        let half_angle = MAX_ANGLE / 2.0;
        let rand_angle_offset: f32 = rand::thread_rng().gen_range(-half_angle..=half_angle);
        let rand_angle = self.rad + rand_angle_offset;

        self.dx = rand_angle.cos();
        self.dy = rand_angle.sin();
        self.rad = rand_angle;
    }

    fn invert(&mut self, fact: f32) {
        self.rad = fact * PI - self.rad
    }
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn x(&self) -> f32 {
        return self.x;
    }

    pub fn y(&self) -> f32 {
        return self.y;
    }
}
