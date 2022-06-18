#[cfg(test)]
pub mod test {
    mod random{
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;
        use approx::assert_relative_eq;
        use super::super::super::*;
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
        use super::super::super::*;  
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