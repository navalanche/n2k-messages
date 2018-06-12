use ::generated::enums::Dummy;

pub fn encode_u8(buf: &mut [u8], value: &u8, start: usize, length: usize, _resolution: usize) {
//    eprintln!("value = {:08b}", value);
//    eprintln!("start = {:#?}", start);
//    eprintln!("length = {:#?}", length);

    if (1 << length) - 1 < (*value as usize) {
        panic!("value too large for length")
    }
    let offset_in_first_byte = start % 8;
    let first_byte_index = start / 8;


    if offset_in_first_byte + length <= 8 {
        // all bits in first byte
        let bytes_left = 8 - offset_in_first_byte - length;
        let pattern = value << bytes_left;
        buf[first_byte_index] |= pattern;
    } else {
        let bits_in_first_byte = 8 - offset_in_first_byte;
        let bits_in_second_byte = offset_in_first_byte + length - 8;
//        eprintln!("bits_in_first_byte = {:#?}", bits_in_first_byte);
//        eprintln!("bits_in_second_byte = {:#?}", bits_in_second_byte);

        // first byte contains LSB bits
        // second byte contains MSB bits

        let lsb = *value << (8 - bits_in_first_byte);
//        eprintln!("lsb = {:08b}", lsb);
        let lsb = lsb >> offset_in_first_byte;
//        eprintln!("lsb = {:08b}", lsb);

        buf[first_byte_index] |= lsb;
        let msb = *value >> bits_in_first_byte;
//        eprintln!("msb = {:08b}", msb);
        let msb = msb << (8 - bits_in_second_byte);
//        eprintln!("msb = {:08b}", msb);

        buf[first_byte_index + 1] |= msb;
    }
}

pub fn encode_u16(buf: &mut [u8], value: &u16, start: usize, length: usize, _resolution: usize) {
//    eprintln!("value = {:X}", value);
//    eprintln!("start = {:#?}", start);
//    eprintln!("length = {:#?}", length);

    if (1 << length) - 1 < (*value as usize) {
        panic!("value too large for length")
    }

    let mut bits_left = length;
    let mut bits_to_write_now = 8 - (start % 8);
    let mut bit_offset = start % 8;
    let mut current_byte = start / 8;
    let mut current_value = *value;
    // writing one byte at a time
    while bits_left > 0 {
//        eprintln!("current_value = {:X}", current_value);
//        eprintln!("bits_left = {:#?}", bits_left);
//        eprintln!("bits_to_write_now = {:#?}", bits_to_write_now);
        if bit_offset == 0 {
            //take the lowest 8 bits, and put them in the buffer
            let mut pattern = current_value as u8;
            //if we are not writing a full byte, shift the value to the left
            if bits_to_write_now < 8 {
                pattern <<= 8 - bits_to_write_now;
            }
            //stamp it in the buffer
            buf[current_byte] |= pattern;
        } else {
            //this arm will only get executed once
            //shift the value to the right to align with the offset
            let pattern = (current_value as u8) >> bit_offset;
            //stamp it in the buffer
            buf[current_byte] |= pattern;
            //not doing this ever again
            bit_offset = 0;
        }
        current_value >>= bits_to_write_now;
        current_byte += 1;
        bits_left -= bits_to_write_now;
        bits_to_write_now = ::std::cmp::min(8, bits_left);
    }
}

pub fn encode_u32(buf: &mut [u8], value: &u32, start: usize, length: usize, _resolution: usize) {
//    eprintln!("value = {:032b}", value);
//    eprintln!("start = {:#?}", start);
//    eprintln!("length = {:#?}", length);

    if (1 << length) - 1 < (*value as usize) {
        panic!("value too large for length")
    }

    let mut bits_left = length;
    let mut bits_to_write_now = 8 - (start % 8);
    let mut bit_offset = start % 8;
    let mut current_byte = start / 8;
    let mut current_value = *value;
    // writing one byte at a time
    while bits_left > 0 {
//        eprintln!("current_value = {:X}", current_value);
//        eprintln!("bits_left = {:#?}", bits_left);
//        eprintln!("bits_to_write_now = {:#?}", bits_to_write_now);
        if bit_offset == 0 {
            //take the lowest 8 bits, and put them in the buffer
            let mut pattern = current_value as u8;
            //if we are not writing a full byte, shift the value to the left
            if bits_to_write_now < 8 {
                pattern <<= 8 - bits_to_write_now;
            }
            //stamp it in the buffer
            buf[current_byte] |= pattern;
        } else {
            //this arm will only get executed once
            //shift the value to the right to align with the offset
            let pattern = (current_value as u8) >> bit_offset;
            //stamp it in the buffer
            buf[current_byte] |= pattern;
            //not doing this ever again
            bit_offset = 0;
        }
        current_value >>= bits_to_write_now;
        current_byte += 1;
        bits_left -= bits_to_write_now;
        bits_to_write_now = ::std::cmp::min(8, bits_left);
    }
}

pub fn encode_dummy(_buf: &[u8], _value: &Dummy, _start: usize, _length: usize, _resolution: usize) {
    unimplemented!("encode_dummy")
}

pub fn encode_string(_buf: &[u8], _value: &String, _start: usize, _length: usize, _resolution: usize) {
    unimplemented!("encode_string")
}

#[cfg(test)]
mod test {
    use super::*;

    use ::decoders::FromN2kMessage;

    #[test]
    fn test_u8_1() {
        for val in 0..256_u16 {
            for shift in 0..9 {
                test_u8_for(val as u8, shift);
            }
        }
    }

    fn test_u8_for(val: u8, start: usize) {
        let mut buf = [0; 2];
        encode_u8(&mut buf, &val, start, 8, 0);
        let read: u8 = FromN2kMessage::from_n2kmessage(&buf, start, 8);
        assert_eq!(read, val);
    }

    #[test]
    fn test_u32() {
        let mut buf = [0; 4];
        encode_u32(&mut buf, &0xFFFFFFFF, 0, 32, 0);
        assert_eq!(buf[0], 255);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 255);
        assert_eq!(buf[3], 255);

        let mut buf = [0; 6];
        encode_u32(&mut buf, &0xFFFFFFFF, 0, 48, 0);
        assert_eq!(buf[0], 255);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 255);
        assert_eq!(buf[3], 255);
        assert_eq!(buf[4], 0);
        assert_eq!(buf[5], 0);
    }

    #[test]
    fn test_u32_2() {
        let mut buf = [0; 4];
        encode_u32(&mut buf, &0xFFFFFFFF, 0, 32, 0);
        assert_eq!(buf[0], 255);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 255);
        assert_eq!(buf[3], 255);

        let mut buf = [0; 5];
        encode_u32(&mut buf, &0xFFFFFFFF, 1, 32, 0);
        assert_eq!(buf[0], 127);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 255);
        assert_eq!(buf[3], 255);
        assert_eq!(buf[4], 128);

        let mut buf = [0; 5];
        encode_u32(&mut buf, &0xFFFFFFFF, 2, 32, 0);
        assert_eq!(buf[0], 63);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 255);
        assert_eq!(buf[3], 255);
        assert_eq!(buf[4], 192);

        let mut buf = [0; 4];
        encode_u32(&mut buf, &0x1FFFFFFF, 2, 29, 0);
        assert_eq!(buf[0], 63);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 255);
        assert_eq!(buf[3], 254);


        let mut buf = [0; 4];
        encode_u32(&mut buf, &0x1FFFFFFF, 3, 29, 0);
        assert_eq!(buf[0], 31);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 255);
        assert_eq!(buf[3], 255);
    }

    #[test]
    fn test_u16() {
        let mut buf = [0; 2];
        encode_u16(&mut buf, &0xFFFF, 0, 16, 0);
        assert_eq!(buf[0], 255);
        assert_eq!(buf[1], 255);
    }

    #[test]
    fn test_u16_2() {
        let mut buf = [0; 3];
        encode_u16(&mut buf, &0xFFFF, 1, 16, 0);
        assert_eq!(buf[0], 127);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 128);
    }

    #[test]
    fn test_u16_3() {
        let mut buf = [0; 3];
        encode_u16(&mut buf, &0xFFFF, 2, 16, 0);
        assert_eq!(buf[0], 63);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 192);
    }

    #[test]
    fn test_u16_4() {
        let mut buf = [0; 3];
        encode_u16(&mut buf, &0x1FFF, 2, 13, 0);
        assert_eq!(buf[0], 63);
        assert_eq!(buf[1], 254);
        assert_eq!(buf[2], 0);
    }

    #[test]
    fn test_u16_5() {
        let mut buf = [0; 3];
        encode_u16(&mut buf, &0x1FFF, 3, 13, 0);
        assert_eq!(buf[0], 31);
        assert_eq!(buf[1], 255);
        assert_eq!(buf[2], 0);
    }
}