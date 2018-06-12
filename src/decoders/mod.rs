use std::cmp::min;


pub trait FromN2kMessage {
    fn from_n2kmessage(_message: &[u8], start: usize, length: usize) -> Self;
}

impl FromN2kMessage for u32 {
    fn from_n2kmessage(message: &[u8], start: usize, length: usize) -> u32 {
        // by choice: if a zero length field is read, we return 0 (zero) as result
        if length == 0 {
            return 0;
        }

        // check we do not read stuff that does not fit in a u32
        if length > 32 {
            panic!("Reading more than 32 bits not supported");
        }

        // check we do not read past the end of the data buffer
        let last_bit = start + length;
        if message.len() * 8 < last_bit {
            panic!("Data too short for operation");
        }


        let mut result: u32 = 0;  //result value
        let mut remaining = length; //remaining bits
        let mut start_idx = start; //index to start reading (in bits)
        let mut magnitude: usize = 0; //current magnitude

        while remaining > 0 {
            //take a byte out of the data array that contains the bit start_idx
            let byte = message[start_idx / 8];
            // amount of least significant bits not to use in this byte
            let skip_lsbs = start_idx % 8;
            // amount of bytes that need to be read from this byte
            let tbd_in_byte = min(remaining, 8 - skip_lsbs);
            // amount of most significant bits not to use in this byte
            let skip_msbs = 8 - skip_lsbs - tbd_in_byte;

            // first mask out the unnecessary MSB's
            let masked = if skip_msbs == 0 {
                byte
            } else {
                // create a mask of all zeros and shift it for all skippable bits
                let mask = 0xFF >> skip_msbs;
                byte & mask
            };

            // now shift out the LSB's and make it 32 bit wide
            let shifted = (masked >> skip_lsbs) as u32;

            // add (by OR-ing) to the result shifted by magnitude
            result = result | (shifted << magnitude);

            // now change all offsets and repeat
            remaining -= tbd_in_byte;
            magnitude += tbd_in_byte;
            start_idx += tbd_in_byte;
        }
        result
    }
}


impl FromN2kMessage for u16 {
    fn from_n2kmessage(message: &[u8], start: usize, length: usize) -> u16 {
// by choice: if a zero length field is read, we return 0 (zero) as result
        if length == 0 {
            return 0;
        }

        // check we do not read stuff that does not fit in a u32
        if length > 16 {
            panic!("Reading more than 16 bits not supported");
        }

        // check we do not read past the end of the data buffer
        let last_bit = start + length;
        if message.len() * 8 < last_bit {
            panic!("Data too short for operation");
        }


        let mut result: u16 = 0;  //result value
        let mut remaining = length; //remaining bits
        let mut start_idx = start; //index to start reading (in bits)
        let mut magnitude: usize = 0; //current magnitude

        while remaining > 0 {
            //take a byte out of the data array that contains the bit start_idx
            let byte = message[start_idx / 8];
            // amount of least significant bits not to use in this byte
            let skip_lsbs = start_idx % 8;
            // amount of bytes that need to be read from this byte
            let tbd_in_byte = min(remaining, 8 - skip_lsbs);
            // amount of most significant bits not to use in this byte
            let skip_msbs = 8 - skip_lsbs - tbd_in_byte;

            // first mask out the unnecessary MSB's
            let masked = if skip_msbs == 0 {
                byte
            } else {
                // create a mask of all zeros and shift it for all skippable bits
                let mask = 0xFF >> skip_msbs;
                byte & mask
            };

            // now shift out the LSB's and make it 16 bit wide
            let shifted = (masked >> skip_lsbs) as u16;

            // add (by OR-ing) to the result shifted by magnitude
            result = result | (shifted << magnitude);

            // now change all offsets and repeat
            remaining -= tbd_in_byte;
            magnitude += tbd_in_byte;
            start_idx += tbd_in_byte;
        }
        result
    }
}


impl FromN2kMessage for u8 {
    fn from_n2kmessage(message: &[u8], start: usize, length: usize) -> u8 {
        if length == 0 {
            return 0;
        }

        // check we do not read stuff that does not fit in a u32
        if length > 16 {
            panic!("Reading more than 16 bits not supported");
        }

        // check we do not read past the end of the data buffer
        let last_bit = start + length;
        if message.len() * 8 < last_bit {
            panic!("Data too short for operation");
        }


        let mut result: u8 = 0;  //result value
        let mut remaining = length; //remaining bits
        let mut start_idx = start; //index to start reading (in bits)
        let mut magnitude: usize = 0; //current magnitude

        while remaining > 0 {
            //take a byte out of the data array that contains the bit start_idx
            let byte = message[start_idx / 8];
            // amount of least significant bits not to use in this byte
            let skip_lsbs = start_idx % 8;
            // amount of bytes that need to be read from this byte
            let tbd_in_byte = min(remaining, 8 - skip_lsbs);
            // amount of most significant bits not to use in this byte
            let skip_msbs = 8 - skip_lsbs - tbd_in_byte;

            // first mask out the unnecessary MSB's
            let masked = if skip_msbs == 0 {
                byte
            } else {
                // create a mask of all zeros and shift it for all skippable bits
                let mask = 0xFF >> skip_msbs;
                byte & mask
            };

            // now shift out the LSB's and make it 16 bit wide
            let shifted = (masked >> skip_lsbs) as u8;

            // add (by OR-ing) to the result shifted by magnitude
            result = result | (shifted << magnitude);

            // now change all offsets and repeat
            remaining -= tbd_in_byte;
            magnitude += tbd_in_byte;
            start_idx += tbd_in_byte;
        }
        result
    }

}

impl FromN2kMessage for String {
    fn from_n2kmessage(message: &[u8], start: usize, length: usize) -> String {
        // println!("read_ascii, start at {}, length {}", _start,_length);

        if start % 8 == 0 && length % 8 == 0 {
            let mut res = String::with_capacity(length / 8 + 1);

            let start_byte = start / 8;
            let end_byte = start_byte + (length / 8);

            // println!("start byte {}, end byte {}", start_byte, end_byte);

            for index in start_byte..end_byte {
                let b = message[index];
                //                result.a
                //    print!("{:X}:", b);
                if b.is_ascii() && b != 0 {
                    res.push(b as char);
                    //    print!("{} -", b as char);
                }
            }

            res.trim().to_string()
        } else {
            "<<cannot decode>>".to_string()
        }
    }
}
