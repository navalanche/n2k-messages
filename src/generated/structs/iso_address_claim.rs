#[allow(unused_imports)]
use ::generated::enums::*;
#[allow(unused_imports)]
use ::types::*;
pub struct IsoAddressClaim {
	// Field { order: "1", id: "uniqueNumber", name: "Unique Number", bit_length: "21", bit_offset: "0", n2k_type: "Binary data", resolution: "" }
	pub unique_number: u32,
	// Field { order: "2", id: "manufacturerCode", name: "Manufacturer Code", bit_length: "11", bit_offset: "21", n2k_type: "Manufacturer code", resolution: "" }
	pub manufacturer_code: u16,
	// Field { order: "3", id: "deviceInstanceLower", name: "Device Instance Lower", bit_length: "3", bit_offset: "32", n2k_type: "", resolution: "" }
	pub device_instance_lower: u8,
	// Field { order: "4", id: "deviceInstanceUpper", name: "Device Instance Upper", bit_length: "5", bit_offset: "35", n2k_type: "", resolution: "" }
	pub device_instance_upper: u8,
	// Field { order: "5", id: "deviceFunction", name: "Device Function", bit_length: "8", bit_offset: "40", n2k_type: "", resolution: "" }
	pub device_function: u16,
	// Field { order: "6", id: "reserved", name: "Reserved", bit_length: "1", bit_offset: "48", n2k_type: "Binary data", resolution: "" }
	// reserved field, length = 1 bits
	// Field { order: "7", id: "deviceClass", name: "Device Class", bit_length: "7", bit_offset: "49", n2k_type: "Lookup table", resolution: "" }
	pub device_class: Dummy,
	// Field { order: "8", id: "systemInstance", name: "System Instance", bit_length: "4", bit_offset: "56", n2k_type: "", resolution: "" }
	pub system_instance: u8,
	// Field { order: "9", id: "industryGroup", name: "Industry Group", bit_length: "3", bit_offset: "60", n2k_type: "Lookup table", resolution: "" }
	pub industry_group: Dummy,
	// Field { order: "10", id: "reserved", name: "Reserved", bit_length: "1", bit_offset: "63", n2k_type: "Binary data", resolution: "" }
	// reserved field, length = 1 bits
}
impl IsoAddressClaim {
	pub fn decode(/* TODO */) -> IsoAddressClaim {
		let unique_number = u32::default(); // TODO
		let manufacturer_code = u16::default(); // TODO
		let device_instance_lower = u8::default(); // TODO
		let device_instance_upper = u8::default(); // TODO
		let device_function = u16::default(); // TODO
		// not decoding reserved field with 1 bits
		let device_class = Dummy::default(); // TODO
		let system_instance = u8::default(); // TODO
		let industry_group = Dummy::default(); // TODO
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
	pub fn encode(&self) -> Vec<u8> {
		unimplemented!();
	}
}
