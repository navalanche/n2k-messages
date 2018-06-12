#[allow(unused_imports)]
use ::generated::enums::*;
#[allow(unused_imports)]
use ::types::*;
#[allow(unused_imports)]
use n2k_base::*;
use ::decoders::*;
use ::encoders::*;

/// Struct for message with PGN = 126996
/// Description =  Product Information
/// PGNInfo { pgn: "126996", id: "productInformation", description: "Product Information", length: "134", fields: Fields { fields: [Field { order: "1", id: "nmea2000Version", name: "NMEA 2000 Version", bit_length: "16", bit_offset: "0", n2k_type: "", resolution: "", signed: "false" }, Field { order: "2", id: "productCode", name: "Product Code", bit_length: "16", bit_offset: "16", n2k_type: "", resolution: "", signed: "false" }, Field { order: "3", id: "modelId", name: "Model ID", bit_length: "256", bit_offset: "32", n2k_type: "ASCII text", resolution: "", signed: "false" }, Field { order: "4", id: "softwareVersionCode", name: "Software Version Code", bit_length: "256", bit_offset: "288", n2k_type: "ASCII text", resolution: "", signed: "false" }, Field { order: "5", id: "modelVersion", name: "Model Version", bit_length: "256", bit_offset: "544", n2k_type: "ASCII text", resolution: "", signed: "false" }, Field { order: "6", id: "modelSerialCode", name: "Model Serial Code", bit_length: "256", bit_offset: "800", n2k_type: "ASCII text", resolution: "", signed: "false" }, Field { order: "7", id: "certificationLevel", name: "Certification Level", bit_length: "8", bit_offset: "1056", n2k_type: "", resolution: "", signed: "false" }, Field { order: "8", id: "loadEquivalency", name: "Load Equivalency", bit_length: "8", bit_offset: "1064", n2k_type: "", resolution: "", signed: "false" }] } }
#[derive(Debug)]
pub struct ProductInformation {
	// Field { order: "1", id: "nmea2000Version", name: "NMEA 2000 Version", bit_length: "16", bit_offset: "0", n2k_type: "", resolution: "", signed: "false" }
	pub nmea2000_version: u16,
	// Field { order: "2", id: "productCode", name: "Product Code", bit_length: "16", bit_offset: "16", n2k_type: "", resolution: "", signed: "false" }
	pub product_code: u16,
	// Field { order: "3", id: "modelId", name: "Model ID", bit_length: "256", bit_offset: "32", n2k_type: "ASCII text", resolution: "", signed: "false" }
	pub model_id: String,
	// Field { order: "4", id: "softwareVersionCode", name: "Software Version Code", bit_length: "256", bit_offset: "288", n2k_type: "ASCII text", resolution: "", signed: "false" }
	pub software_version_code: String,
	// Field { order: "5", id: "modelVersion", name: "Model Version", bit_length: "256", bit_offset: "544", n2k_type: "ASCII text", resolution: "", signed: "false" }
	pub model_version: String,
	// Field { order: "6", id: "modelSerialCode", name: "Model Serial Code", bit_length: "256", bit_offset: "800", n2k_type: "ASCII text", resolution: "", signed: "false" }
	pub model_serial_code: String,
	// Field { order: "7", id: "certificationLevel", name: "Certification Level", bit_length: "8", bit_offset: "1056", n2k_type: "", resolution: "", signed: "false" }
	pub certification_level: u8,
	// Field { order: "8", id: "loadEquivalency", name: "Load Equivalency", bit_length: "8", bit_offset: "1064", n2k_type: "", resolution: "", signed: "false" }
	pub load_equivalency: u8,
}
impl ProductInformation {
	pub fn decode(n2k_message: &N2kMessage) -> ProductInformation {
		let nmea2000_version = FromN2kMessage::from_n2kmessage(&n2k_message.body, 0, 16);
		let product_code = FromN2kMessage::from_n2kmessage(&n2k_message.body, 16, 16);
		let model_id = FromN2kMessage::from_n2kmessage(&n2k_message.body, 32, 256);
		let software_version_code = FromN2kMessage::from_n2kmessage(&n2k_message.body, 288, 256);
		let model_version = FromN2kMessage::from_n2kmessage(&n2k_message.body, 544, 256);
		let model_serial_code = FromN2kMessage::from_n2kmessage(&n2k_message.body, 800, 256);
		let certification_level = FromN2kMessage::from_n2kmessage(&n2k_message.body, 1056, 8);
		let load_equivalency = FromN2kMessage::from_n2kmessage(&n2k_message.body, 1064, 8);
		ProductInformation {
			nmea2000_version,
			product_code,
			model_id,
			software_version_code,
			model_version,
			model_serial_code,
			certification_level,
			load_equivalency,
		}
	}
	pub fn encode(&self, mut buffer: &mut [u8]) {
		encode_u16(&mut buffer, &self.nmea2000_version, 0, 16, 0);
		encode_u16(&mut buffer, &self.product_code, 16, 16, 0);
		encode_string(&mut buffer, &self.model_id, 32, 256, 0);
		encode_string(&mut buffer, &self.software_version_code, 288, 256, 0);
		encode_string(&mut buffer, &self.model_version, 544, 256, 0);
		encode_string(&mut buffer, &self.model_serial_code, 800, 256, 0);
		encode_u8(&mut buffer, &self.certification_level, 1056, 8, 0);
		encode_u8(&mut buffer, &self.load_equivalency, 1064, 8, 0);
	}
}
