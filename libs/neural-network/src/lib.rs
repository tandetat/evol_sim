use rand::{Rng, RngCore};
pub struct Network{
    layers: Vec<Layer>,
}

struct Layer{
    neurons: Vec<Neuron>,
}
struct Neuron{
    bias: f32,
    weights: Vec<f32>,
}

pub struct LayerTopology {
    pub neurons: usize,
}
impl Network{
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
        .iter()
        .fold(inputs, |inputs, layer| layer.propagate(inputs))

    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

       Self{
        layers: layers
        .windows(2)
        .map(|layers|
            Layer::random(rng,layers[0].neurons, layers[1].neurons))
        .collect(),
       } 
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
        .iter()
        .map(|neuron| neuron.propagate(&inputs))
        .collect()
    }

    fn random(rng: &mut dyn RngCore, input: usize, output:usize) -> Self {
        Self { 
            neurons: (0..output)
            .map(|_| Neuron::random(rng,input))
            .collect(),
         }
    }
}

impl Neuron{
    fn propagate(&self, inputs: &[f32]) -> f32{
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
         .iter()
         .zip(&self.weights)
         .fold(0.0, |output:f32, (input, weight)| output + input*weight);

        (self.bias + output).max(0.0)

    }

    fn random (rng: &mut dyn RngCore,input: usize) -> Self {
        let bias: f32 = rng.gen_range(-1.0..=1.0);

        Self {bias, weights: (0..input).
        map(|_| rng.gen_range(-1.0..=1.0))
        .collect(),}
    }
}
#[cfg(test)]
pub mod tests {
    mod random{
        use crate::Neuron;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;
        use approx::assert_relative_eq;
        #[test]
        fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 5);

        assert_relative_eq!(neuron.bias, -0.6255188);

         assert_relative_eq!(neuron.weights.as_slice(), [
            0.67383957,
            0.8181262,
            0.26284897,
            0.5238807,
            -0.53516835,
    ].as_ref());
        }
    }
    mod propagate {
        use crate::Neuron;
        #[test]
        fn test(){
            let neuron = Neuron {
                bias: 0.2,
                weights: vec![0.5, -0.1],
            };

            approx::assert_relative_eq!(
                neuron.propagate(&[-7.0, -5.0]),
                0.0,
            );
            
            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 0.2]),
                (0.5*0.5) + (0.2*(-0.1)) + 0.2,
            );
            
            }
        }
    
}