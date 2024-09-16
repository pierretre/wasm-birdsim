mod utils;

use crate::utils::*;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct DirectionVector {
    dx: i32,
    dy: i32,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Bird {
    coord_x: u32,
    coord_y: u32,
    direction: DirectionVector,
}

#[wasm_bindgen]
pub struct Area {
    width: u32,
    height: u32,
    birds: Vec<Bird>,
}

#[wasm_bindgen]
pub struct Point {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
impl Area {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Area {
        let width = 1000;
        let height = 800;
        let birds = Vec::new();

        Area {
            width,
            height,
            birds,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn add_bird(&mut self) {
        let int_radius: u32 = RADIUS.round().abs() as u32;
        let x = rand::thread_rng().gen_range(int_radius..self.width - int_radius);
        let y = rand::thread_rng().gen_range(int_radius..self.height - int_radius);
        let bird: Bird = Bird::new(x, y);
        self.birds.push(bird);
    }

    pub fn tick(&mut self) {
        let mut birds = self.birds.to_vec();

        // Do the math here !!

        self.birds = birds
    }

    pub fn get_birds(&self) -> Vec<Bird> {
        return self.birds.to_vec();
    }

    pub fn nb_birds(&self) -> usize {
        return self.birds.len();
    }
}

#[wasm_bindgen]
impl Bird {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> Bird {
        Bird {
            coord_x: x,
            coord_y: y,
            direction: DirectionVector { dx: 1, dy: 0 },
        }
    }

    pub fn coord_x(&self) -> u32 {
        return self.coord_x;
    }
    pub fn coord_y(&self) -> u32 {
        return self.coord_y;
    }
    pub fn direction(&self) -> DirectionVector {
        return self.direction.clone();
    }

    pub fn direction_line_stop(&self) -> Point {
        let distance: f32 = RADIUS; // arbitrary value to set

        let dx = self.direction.dx as f32;
        let dy = self.direction.dy as f32;
        let coordx = self.coord_x as f32;
        let coordy = self.coord_y as f32;

        let inv_length = fast_inv_sqrt(dx.powi(2) + dy.powi(2));
        return Point {
            x: (coordx + dx * inv_length * distance) as u32,
            y: (coordy + dy * inv_length * distance) as u32,
        };
    }
}

#[wasm_bindgen]
impl DirectionVector {
    #[wasm_bindgen(constructor)]
    pub fn new(dx: i32, dy: i32) -> DirectionVector {
        DirectionVector { dx: dx, dy: dy }
    }

    pub fn dx(&self) -> i32 {
        return self.dx;
    }

    pub fn dy(&self) -> i32 {
        return self.dy;
    }
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }
    }

    pub fn x(&self) -> u32 {
        return self.x;
    }

    pub fn y(&self) -> u32 {
        return self.y;
    }
}

#[wasm_bindgen]
pub fn bird_radius() -> f32 {
    return RADIUS;
}
#[wasm_bindgen]
pub fn max_angle() -> f32 {
    return MAX_ANGLE;
}
