mod utils;

use crate::utils::*;
use rand::Rng;
use std::f32::consts::PI;
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

struct NeighborInfo {
    bird_id: usize,
    direction: DirectionVector,
    distance: f32,
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
        let n = self.nb_birds();
        let mut tmp = vec![];
        let mut tmp_birds = self.birds.clone();

        for i in 0..n {
            let mut bird = tmp_birds[i].clone();
            self.handle_bird_movement(i, &mut bird, &tmp_birds);
            tmp.push(bird.clone());
        }

        self.birds = tmp;
    }

    fn handle_bird_movement(&mut self, i: usize, bird: &mut Bird, tmp_birds: &[Bird]) {
        let pt: Point = bird.direction_line_stop();

        if pt.x < 0.0 || pt.x >= self.width {
            bird.wall_bounce(HORIZONTAL_BOUNCE);
        } else if pt.y < 0.0 || pt.y >= self.height {
            bird.wall_bounce(VERTICAL_BOUNCE);
        } else {
            let neighbors = self.update_neighbors(i, bird, tmp_birds);
            if neighbors.len() > 0 {
                let average_direction = self.compute_average_direction(&neighbors);
                bird.set_direction(DirectionVector::from_rad(average_direction));
            }
            bird.fly();
        }
    }

    fn update_neighbors(&self, i: usize, bird: &mut Bird, tmp_birds: &[Bird]) -> Vec<NeighborInfo> {
        let mut neighbors = vec![];

        for (j, bird_j) in tmp_birds.iter().enumerate() {
            if i != j {
                let distance = bird.distance_to(bird_j);

                if distance <= RADIUS {
                    neighbors.push(NeighborInfo {
                        bird_id: j,
                        direction: bird_j.direction(),
                        distance,
                    });
                }
            }
        }
        neighbors
    }

    fn compute_average_direction(&self, neighbors: &[NeighborInfo]) -> f32 {
        let mut sum_directions = 0.0;

        for neighbor in neighbors {
            sum_directions += neighbor.direction.rad();
        }

        sum_directions / (neighbors.len() as f32) % (2.0 * PI)
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
            direction,
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

    fn set_direction(&mut self, direction: DirectionVector) {
        self.direction = direction;
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

    fn distance_to(&self, other: &Bird) -> f32 {
        let x_diff = self.coord_x - other.coord_x;
        let y_diff = self.coord_y - other.coord_y;

        (x_diff.powi(2) + y_diff.powi(2)).sqrt()
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
    fn from_rad(rad: f32) -> DirectionVector {
        DirectionVector {
            dx: rad.cos(),
            dy: rad.sin(),
            rad,
        }
    }

    pub fn dx(&self) -> f32 {
        self.dx
    }

    pub fn dy(&self) -> f32 {
        self.dy
    }

    pub fn rad(&self) -> f32 {
        self.rad % (2.0 * PI)
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
        let nb: f32 = rand::thread_rng().gen_range(0.0..=1.1);
        let sign = if rand::thread_rng().gen_bool(0.5) {
            -1
        } else {
            1
        };
        let rand_angle = self.rad + (nb.powi(2) * half_angle * sign as f32);

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
        Point { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
