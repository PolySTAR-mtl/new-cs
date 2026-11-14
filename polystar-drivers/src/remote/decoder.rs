use crate::decoder::Decoder;

use super::protocol::{
    PROTOCOL_FIELDS_COUNT,
    DR16_PROTOCOL,
};


/// DJI DR16 decoder type.
pub type Dr16Decoder = Decoder<PROTOCOL_FIELDS_COUNT>;

/// Construct a DJI DR16 decoder.
pub fn create_dr16_decoder() -> Dr16Decoder {
    Decoder::new(&DR16_PROTOCOL)
}