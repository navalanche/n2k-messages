#[allow(unused_imports)]
use ::generated::enums::*;
#[allow(unused_imports)]
use ::types::*;
pub struct IsoRequest {
	// Field { order: "1", id: "pgn", name: "PGN", bit_length: "24", bit_offset: "0", n2k_type: "Integer", resolution: "1" }
	pub pgn: u32,
}
impl IsoRequest {
	pub fn decode(/* TODO */) -> IsoRequest {
		let pgn = u32::default(); // TODO
		IsoRequest {
			pgn,
		}
	}
	pub fn encode(&self) -> Vec<u8> {
		unimplemented!();
	}
}
