/// TensorFlow tensor shape and dtype model for SigmaOS ML pipeline bridge.

#[derive(Debug, Clone, PartialEq)]
pub enum DType {
    Float32,
    Float64,
    Int32,
    Int64,
    Bool,
    String,
}

#[derive(Debug, Clone)]
pub struct TensorShape {
    pub dims: Vec<Option<i64>>,
}

impl TensorShape {
    pub fn new(dims: &[Option<i64>]) -> Self {
        Self {
            dims: dims.to_vec(),
        }
    }

    pub fn rank(&self) -> usize {
        self.dims.len()
    }

    pub fn num_elements(&self) -> Option<i64> {
        self.dims.iter().try_fold(1i64, |acc, d| d.map(|v| acc * v))
    }

    pub fn is_fully_defined(&self) -> bool {
        self.dims.iter().all(|d| d.is_some())
    }
}

#[derive(Debug, Clone)]
pub struct ModelSignature {
    pub name: String,
    pub inputs: Vec<(String, TensorShape, DType)>,
    pub outputs: Vec<(String, TensorShape, DType)>,
}

impl ModelSignature {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, name: &str, shape: TensorShape, dtype: DType) {
        self.inputs.push((name.into(), shape, dtype));
    }

    pub fn add_output(&mut self, name: &str, shape: TensorShape, dtype: DType) {
        self.outputs.push((name.into(), shape, dtype));
    }

    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_shape_rank_aaa() {
        let s = TensorShape::new(&[Some(1), Some(224), Some(224), Some(3)]);
        assert_eq!(s.rank(), 4);
    }

    #[test]
    fn test_tensor_shape_num_elements_aaa() {
        let s = TensorShape::new(&[Some(2), Some(3)]);
        assert_eq!(s.num_elements(), Some(6));
    }

    #[test]
    fn test_tensor_shape_dynamic_dim_aaa() {
        let s = TensorShape::new(&[None, Some(224), Some(224)]);
        assert!(!s.is_fully_defined());
        assert_eq!(s.num_elements(), None);
    }

    #[test]
    fn test_model_signature_add_io_aaa() {
        let mut sig = ModelSignature::new("serving_default");
        sig.add_input(
            "image",
            TensorShape::new(&[None, Some(224), Some(224), Some(3)]),
            DType::Float32,
        );
        sig.add_output(
            "logits",
            TensorShape::new(&[None, Some(1000)]),
            DType::Float32,
        );
        assert_eq!(sig.input_count(), 1);
        assert_eq!(sig.output_count(), 1);
    }
}
