use na::Point2;
use nalgebra as na;
use rand::{Rng, RngCore};
pub struct Simulation{
    world: World,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal {
    position: Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

#[derive(Debug)]
pub struct Food{
    position: Point2<f32>,
}


impl Simulation{
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self { world: World::random(rng), }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movement();
    }

    fn process_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);
            
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(
                    &animal.position,
                    &food.position,
                );
                if distance <= 0.01 {

                    food.position = rng.gen::<na::Point2<f32>>();
                }
            }
        }
    }
}

impl World {
    pub fn random(rng:&mut dyn RngCore) -> Self {
        const NUM_OF_ANIMALS: usize = 40;
        const NUM_OF_FOOD: usize = 60;

        let animals = (0..NUM_OF_ANIMALS)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..NUM_OF_FOOD)
            .map(|_| Food::random(rng))
            .collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }
    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        const INIT_SPEED:f32 = 0.002;
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: INIT_SPEED,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self { position: rng.gen() }
    }
    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}