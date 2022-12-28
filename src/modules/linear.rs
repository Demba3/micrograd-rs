use super::Module;
use crate::prelude::*;
use crate::utils::WeightInit;

pub struct Linear {
    weights: Tensor<Ix2>,
    biases: Tensor<Ix1>,
}

impl Linear {
    pub fn new(nin: usize, nout: usize) -> Self {
        let weights = Tensor::from_shape_simple_fn((nout, nin), || {
            WeightInit::GlorotUniform.sample([nin, nout])
        });
        let biases = Tensor::zeros(nout);

        Linear { weights, biases }
    }
}

impl Module for Linear {
    type Dim = Ix1;

    fn parameters(&self) -> Vec<Value> {
        let mut params = self.weights.clone().into_raw_vec();
        params.append(&mut self.biases.clone().into_raw_vec());

        params
    }

    fn forward(&self, input: Tensor<Self::Dim>) -> Tensor<Self::Dim> {
        let weights_t = self.weights.t().into_owned();

        input.dot(&weights_t) + &self.biases
    }
}
