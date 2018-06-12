#[allow(unused_imports)]
use ::generated::enums::*;
#[allow(unused_imports)]
use ::types::*;
#[allow(unused_imports)]
use n2k_base::*;
use ::decoders::*;
use ::encoders::*;

/// Struct for message with PGN = 59904
/// Description =  ISO Request
/// PGNInfo { pgn: "59904", id: "isoRequest", description: "ISO Request", length: "3", fields: Fields { fields: [Field { order: "1", id: "pgn", name: "PGN", bit_length: "24", bit_offset: "0", n2k_type: "Integer", resolution: "1", signed: "false" }] } }
#[derive(Debug)]
pub struct IsoRequest {
	// Field { order: "1", id: "pgn", name: "PGN", bit_length: "24", bit_offset: "0", n2k_type: "Integer", resolution: "1", signed: "false" }
	pub pgn: u32,
}
impl IsoRequest {
	pub fn decode(n2k_message: &N2kMessage) -> IsoRequest {
		let pgn = FromN2kMessage::from_n2kmessage(&n2k_message.body, 0, 24);
		IsoRequest {
			pgn,
		}
	}
	pub fn encode(&self, mut buffer: &mut [u8]) {
		encode_u32(&mut buffer, &self.pgn, 0, 24, 1);
	}
}
