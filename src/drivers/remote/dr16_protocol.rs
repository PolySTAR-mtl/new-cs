use crate::drivers::protocols::protocol_codec::DecoderProtocol;
use crate::drivers::protocols::decoder::{
    Decoder,
    DecodeError,
    DecodedField
};

use super::dr16_fields::{
    PROTOCOL_FIELDS,
    PROTOCOL_FIELDS_COUNT,
    FRAME_SIZE
};

pub struct Dr16Protocol {
    decoder: Decoder<PROTOCOL_FIELDS_COUNT, FRAME_SIZE>
}

impl Dr16Protocol {
    pub fn new() -> Result<Self, DecodeError> {
        let decoder = Decoder::<PROTOCOL_FIELDS_COUNT, FRAME_SIZE>::new(&PROTOCOL_FIELDS)?;
        Ok(Self { decoder })
    }
}

impl DecoderProtocol<PROTOCOL_FIELDS_COUNT> for Dr16Protocol {
    fn decode(&self, frame: &[u8]) -> Result<[DecodedField; PROTOCOL_FIELDS_COUNT], DecodeError> {
        self.decoder.decode(frame)
    }
}