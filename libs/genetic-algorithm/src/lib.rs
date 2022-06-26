#![feature(type_alias_impl_trait)]
use std::ops::Index;

use rand::{Rng, RngCore, seq::SliceRandom};

pub struct GeneticAlgorithm <S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S> 
where 
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self { selection_method,
                crossover_method: Box::new(crossover_method),
                mutation_method: Box::new(mutation_method),
            }
    }

    pub fn iterate<I>(&self, 
        population: &[I],
        rng: &mut dyn RngCore,
    )-> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        
        // pre condition checking

        assert!(!population.is_empty());

        let new_pop = (0..population.len())
            .map(|_| {
                let parent_a = self
                    .selection_method
                    .select(population, rng);
                let parent_b = self
                    .selection_method
                    .select(population, rng);

                let chromosome_a = parent_a.chromosome();
                let chromosome_b = parent_b.chromosome();

                let mut child = self
                     .crossover_method
                     .crossover(rng, chromosome_a, chromosome_b);

                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            })
          .collect();

          let stats = Statistics::new(population);

          (new_pop, stats)
    }
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index:usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>> (iter: T) -> Self {
        Self { 
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub trait CrossoverMethod {
     fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
     ) -> Chromosome;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}
impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
     ) -> Chromosome {

        assert_eq!(parent_a.len(), parent_b.len()); //precondition

        let parent_a_iter = parent_a.iter();
        let parent_b_iter = parent_b.iter();

        const RATIO:f64 = 0.5; 
        parent_a_iter
            .zip(parent_b_iter)
            .map(|(&a, &b)| if rng.gen_bool(RATIO) {a} else {b} )
            .collect()
        }
}

pub trait MutationMethod {
    fn mutate(
        &self,
        rng: &mut dyn RngCore,
        child: &mut Chromosome
    );
}
#[derive(Clone, Debug)]
pub struct GaussianMutation{
    chance: f32, //probability of one gene mutating

    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance:f32, coeff:f32) -> Self {
        assert!(chance>= 0.0 && chance <= 1.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(
        &self,
        rng: &mut dyn RngCore,
        child: &mut Chromosome
    ) {
        const NEGATIVE: f32 = -1.0;
        const POSITIVE: f32 = 1.0;
        const HALF: f64 = 0.5;
        child
        .iter_mut()
        .for_each(|gene| {
            let sign = if rng.gen_bool(HALF) {NEGATIVE} else {POSITIVE};
            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        })
    }
}
pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}
pub trait SelectionMethod {

    fn select<'a,I>(
        &self, 
        population: &'a [I],
        rng: &mut dyn RngCore) -> &'a I
    where 
        I: Individual;
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a,I>(
        &self, 
        population: &'a [I],
        rng: &mut dyn RngCore) -> &'a I
    where 
        I: Individual,
    {
        population
            .choose_weighted(rng, |indiv| indiv.fitness())
            .expect("Empty population")
    }
}

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fit: f32,
    max_fit: f32,
    avg_fit: f32,
}
impl Statistics {
    fn new<I>(pop: &[I]) -> Self 
    where
        I:Individual,
    {
        assert!(!pop.is_empty());

        let mut min_fit = pop[0].fitness();
        let mut max_fit = min_fit;
        let mut sum_fit = 0.0;

        for indiv in pop {
            let fitness = indiv.fitness();

            min_fit = min_fit.min(fitness);
            max_fit = max_fit.max(fitness);
            sum_fit += fitness;
        }

        Self {
            min_fit,
            max_fit,
            avg_fit: sum_fit / (pop.len() as f32),
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fit
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fit
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fit
    }
}
// Testing Part
//
//
//
//
//
//
//
//
//
//
//
//
//


#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndiv {
    WithChromosome { chromosome: Chromosome},

    WithFitness { fitness: f32}, 
}
#[cfg(test)]

 impl PartialEq for Chromosome {

    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(
             self.genes.as_slice(),
             other.genes.as_slice(),
        )
    }
}

#[cfg(test)]
impl TestIndiv {
     pub fn new(fitness: f32) -> Self {
     Self::WithFitness { fitness }
    }
}   

    #[cfg(test)]
    impl Individual for TestIndiv {

         fn fitness(&self)-> f32 {
            match self {

                Self::WithChromosome { chromosome} => {
                    chromosome.iter().sum()
                },
                Self::WithFitness { fitness } => *fitness, 
            }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {

                Self::WithChromosome { chromosome} => chromosome,
                Self::WithFitness {..} => {
                    panic!("not supported for TestIndiv::WithFitness")
                }
            }
        }

        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
         }
    }       

#[cfg(test)]
pub mod tests {
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn test() {
        let population = vec![
            TestIndiv::new(1.0),
            TestIndiv::new(3.0),
            TestIndiv::new(2.0),
            TestIndiv::new(5.0),
        ];

        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let mut actual_histogram = BTreeMap::new();

        const LIMIT: usize = 500;

        for _ in 0..LIMIT {
            let fitness = method
                .select( &population, &mut rng)
                .fitness() as i32;
            
                *actual_histogram
                    .entry(fitness)
                    .or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter(
            vec![
                (1,44),
                (2,90),
                (3,139),
                (5,227),
            ]);

            assert_eq!(actual_histogram, expected_histogram);
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test() {
        fn individual(genes: &[f32]) -> TestIndiv{
            let chromosome = genes.iter().cloned().collect();
        
            TestIndiv::create(chromosome)
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];
        for _ in 0..10 {
            // population = ga.iterate( &population, &mut rng);
        }

        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]),
            individual(&[1.21268670, 1.5538777, 2.8869110]),
            individual(&[1.06176780, 2.2657390, 4.4287640]),
            individual(&[0.95909685, 2.4618788, 4.0247330]),
        ];

        assert_eq!(population, expected_population);
    }
}