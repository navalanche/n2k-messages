#[allow(unused_imports)]
use ::generated::enums::*;
#[allow(unused_imports)]
use ::types::*;
#[allow(unused_imports)]
use n2k_base::*;
use ::decoders::*;
use ::encoders::*;

/// Struct for message with PGN = 60928
/// Description =  ISO Address Claim
/// PGNInfo { pgn: "60928", id: "isoAddressClaim", description: "ISO Address Claim", length: "8", fields: Fields { fields: [Field { order: "1", id: "uniqueNumber", name: "Unique Number", bit_length: "21", bit_offset: "0", n2k_type: "Binary data", resolution: "", signed: "false" }, Field { order: "2", id: "manufacturerCode", name: "Manufacturer Code", bit_length: "11", bit_offset: "21", n2k_type: "Manufacturer code", resolution: "", signed: "false" }, Field { order: "3", id: "deviceInstanceLower", name: "Device Instance Lower", bit_length: "3", bit_offset: "32", n2k_type: "", resolution: "", signed: "false" }, Field { order: "4", id: "deviceInstanceUpper", name: "Device Instance Upper", bit_length: "5", bit_offset: "35", n2k_type: "", resolution: "", signed: "false" }, Field { order: "5", id: "deviceFunction", name: "Device Function", bit_length: "8", bit_offset: "40", n2k_type: "", resolution: "", signed: "false" }, Field { order: "6", id: "reserved", name: "Reserved", bit_length: "1", bit_offset: "48", n2k_type: "Binary data", resolution: "", signed: "false" }, Field { order: "7", id: "deviceClass", name: "Device Class", bit_length: "7", bit_offset: "49", n2k_type: "Lookup table", resolution: "", signed: "false" }, Field { order: "8", id: "systemInstance", name: "System Instance", bit_length: "4", bit_offset: "56", n2k_type: "", resolution: "", signed: "false" }, Field { order: "9", id: "industryGroup", name: "Industry Group", bit_length: "3", bit_offset: "60", n2k_type: "Lookup table", resolution: "", signed: "false" }, Field { order: "10", id: "reserved", name: "Reserved", bit_length: "1", bit_offset: "63", n2k_type: "Binary data", resolution: "", signed: "false" }] } }
#[derive(Debug)]
pub struct IsoAddressClaim {
	// Field { order: "1", id: "uniqueNumber", name: "Unique Number", bit_length: "21", bit_offset: "0", n2k_type: "Binary data", resolution: "", signed: "false" }
	pub unique_number: u32,
	// Field { order: "2", id: "manufacturerCode", name: "Manufacturer Code", bit_length: "11", bit_offset: "21", n2k_type: "Manufacturer code", resolution: "", signed: "false" }
	pub manufacturer_code: u16,
	// Field { order: "3", id: "deviceInstanceLower", name: "Device Instance Lower", bit_length: "3", bit_offset: "32", n2k_type: "", resolution: "", signed: "false" }
	pub device_instance_lower: u8,
	// Field { order: "4", id: "deviceInstanceUpper", name: "Device Instance Upper", bit_length: "5", bit_offset: "35", n2k_type: "", resolution: "", signed: "false" }
	pub device_instance_upper: u8,
	// Field { order: "5", id: "deviceFunction", name: "Device Function", bit_length: "8", bit_offset: "40", n2k_type: "", resolution: "", signed: "false" }
	pub device_function: u8,
	// Field { order: "6", id: "reserved", name: "Reserved", bit_length: "1", bit_offset: "48", n2k_type: "Binary data", resolution: "", signed: "false" }
	// reserved field, length = 1 bits
	// Field { order: "7", id: "deviceClass", name: "Device Class", bit_length: "7", bit_offset: "49", n2k_type: "Lookup table", resolution: "", signed: "false" }
	pub device_class: u8,
	// Field { order: "8", id: "systemInstance", name: "System Instance", bit_length: "4", bit_offset: "56", n2k_type: "", resolution: "", signed: "false" }
	pub system_instance: u8,
	// Field { order: "9", id: "industryGroup", name: "Industry Group", bit_length: "3", bit_offset: "60", n2k_type: "Lookup table", resolution: "", signed: "false" }
	pub industry_group: u8,
	// Field { order: "10", id: "reserved", name: "Reserved", bit_length: "1", bit_offset: "63", n2k_type: "Binary data", resolution: "", signed: "false" }
	// reserved field, length = 1 bits
}
impl IsoAddressClaim {
	pub fn decode(n2k_message: &N2kMessage) -> IsoAddressClaim {
		let unique_number = FromN2kMessage::from_n2kmessage(&n2k_message.body, 0, 21);
		let manufacturer_code = FromN2kMessage::from_n2kmessage(&n2k_message.body, 21, 11);
		let device_instance_lower = FromN2kMessage::from_n2kmessage(&n2k_message.body, 32, 3);
		let device_instance_upper = FromN2kMessage::from_n2kmessage(&n2k_message.body, 35, 5);
		let device_function = FromN2kMessage::from_n2kmessage(&n2k_message.body, 40, 8);
		// not decoding reserved field with 1 bits
		let device_class = FromN2kMessage::from_n2kmessage(&n2k_message.body, 49, 7);
		let system_instance = FromN2kMessage::from_n2kmessage(&n2k_message.body, 56, 4);
		let industry_group = FromN2kMessage::from_n2kmessage(&n2k_message.body, 60, 3);
		// not decoding reserved field with 1 bits
		IsoAddressClaim {
			unique_number,
			manufacturer_code,
			device_instance_lower,
			device_instance_upper,
			device_function,
			device_class,
			system_instance,
			industry_group,
		}
	}
	pub fn encode(&self, mut buffer: &mut [u8]) {
		encode_u32(&mut buffer, &self.unique_number, 0, 21, 0);
		encode_u16(&mut buffer, &self.manufacturer_code, 21, 11, 0);
		encode_u8(&mut buffer, &self.device_instance_lower, 32, 3, 0);
		encode_u8(&mut buffer, &self.device_instance_upper, 35, 5, 0);
		encode_u8(&mut buffer, &self.device_function, 40, 8, 0);
		encode_u8(&mut buffer, &self.device_class, 49, 7, 0);
		encode_u8(&mut buffer, &self.system_instance, 56, 4, 0);
		encode_u8(&mut buffer, &self.industry_group, 60, 3, 0);
	}
}
