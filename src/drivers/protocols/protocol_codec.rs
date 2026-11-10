use crate::drivers::protocols::decoder::{
    DecodeError, 
    DecodedField
};

pub trait DecoderProtocol<const N_FIELDS: usize> {
    fn decode(&self, frame: &[u8]) -> Result<[DecodedField; N_FIELDS], DecodeError>;
}

pub trait EncoderProtocol<const N_FIELDS: usize>{
    fn encode() -> ();
}