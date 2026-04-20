//! Intermediate Representation for the Super-Compiler.

pub struct Layer {
    pub layer_type: LayerType,
    pub weight_data: Vec<u8>,
}

pub enum LayerType {
    Dense,
    Attention,
    Convolutional,
}

pub struct ModelIR {
    pub layers: Vec<Layer>,
}
