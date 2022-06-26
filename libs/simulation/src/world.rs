use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
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