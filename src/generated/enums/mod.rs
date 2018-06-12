use ::decoders::*;

/// Structure that represents data that is not yet supported by the n2k-codegen crate
#[derive(Debug)]
pub struct Dummy;

impl Default for Dummy {
    fn default() -> Dummy {
        Dummy
    }
}


impl FromN2kMessage for Dummy {
    fn from_n2kmessage(_message: &[u8], _bit_offset: usize, _bit_length: usize) -> Dummy {
        Dummy::default()
    }
}
