use crate::*;
use na::Point2;
use lib_genetic_algorithm as ga;

const INIT_SPEED:f32 = 0.002;
#[derive(Debug)]
pub struct Animal {
    pub(crate) position: Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye : Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,

}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {


        let eye = Eye::default();

        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed:INIT_SPEED,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore
    ) -> Self {
        let eye = Eye::default();

        let brain =  Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }
    pub fn position(&self) -> Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
