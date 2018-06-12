extern crate n2k_base;

mod generated;
mod types;
mod decoders;
mod encoders;
include!("generated-lib.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use n2k_base::*;

    #[test]
    pub fn decode_iso_address_claim() {
        let header = header::N2kHeader::new(3, 1, 255, 60928);
        let data = [0x21, 0x04, 0xb4, 0x2f, 0x00, 0x91, 0x79, 0xc0];
        let message = N2kMessage::new(header, &data);
        let decoded_claim = IsoAddressClaim::decode(&message);

        assert_eq!(decoded_claim.unique_number, 0x140421);
        assert_eq!(decoded_claim.manufacturer_code, 381);
        assert_eq!(decoded_claim.device_instance_lower, 0x0);
        assert_eq!(decoded_claim.device_instance_upper, 0x0);
        assert_eq!(decoded_claim.device_function, 145);
        assert_eq!(decoded_claim.device_class, 60);
        assert_eq!(decoded_claim.system_instance, 0);
        assert_eq!(decoded_claim.industry_group, 4);
    }

    #[test]
    pub fn encode_iso_address_claim() {
        let claim = IsoAddressClaim {
            unique_number: 0x140421,
            manufacturer_code: 381,
            device_instance_lower: 0,
            device_instance_upper: 0,
            device_function: 145,
            device_class: 60,
            system_instance: 0,
            industry_group: 4,
        };

        let mut buf = [0; 8];
        claim.encode(&mut buf);

        let expected = [0x21, 0x04, 0xb4, 0x2f, 0x00, 0x91, 0x79, 0xc0];

        for (index, value) in expected.iter().enumerate() {
            eprintln!("buf[{}] = {:08b}", index, buf[index]);
            eprintln!("value  = {:08b}", value);
            assert_eq!(buf[index], *value);
        }
    }

    #[test]
    pub fn decode_iso_address_claim2() {
        let header = header::N2kHeader::new(6, 10, 255, 60928);
        let data = [0x15, 0x11, 0x77, 0x22, 0x00, 0xBE, 0xA0, 0xC0];
        let message = N2kMessage::new(header, &data);
        let decoded_claim = IsoAddressClaim::decode(&message);

        assert_eq!(decoded_claim.unique_number, 0x171115);
        assert_eq!(decoded_claim.manufacturer_code, 275);
        assert_eq!(decoded_claim.device_instance_lower, 0x0);
        assert_eq!(decoded_claim.device_instance_upper, 0x0);
        assert_eq!(decoded_claim.device_function, 190);
    }

    #[test]
    pub fn decode_product_info() {
        let header = header::N2kHeader::new(6, 10, 255, 126996);
        let data = [21, 5, 101, 39, 84, 55, 32, 32, 32, 32, 32, 32, 32, 105, 71, 80, 83, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 48, 49, 48, 48, 48,
            95, 69, 32, 49, 49, 52, 51, 49, 49, 53, 48, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 48, 48, 56, 48, 52, 52, 35,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 1, 0
        ]; // From a B&G Zeus 7
        let message = N2kMessage::new(header, &data);
        let decoded_claim = ProductInformation::decode(&message);
        assert_eq!(decoded_claim.nmea2000_version, 1301);
        assert_eq!(decoded_claim.product_code, 10085);
        assert_eq!(decoded_claim.model_id, String::from("T7       iGPS"));
        assert_eq!(decoded_claim.software_version_code, String::from("01000_E 11431150"));
        assert_eq!(decoded_claim.model_version, String::from(""));
        assert_eq!(decoded_claim.model_serial_code, String::from("008044#"));
        assert_eq!(decoded_claim.certification_level, 1);
        assert_eq!(decoded_claim.load_equivalency, 0);
    }
}