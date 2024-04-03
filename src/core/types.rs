use std::collections::HashMap;

pub type AsciiString = Vec<u8>;
pub type LZWEncodeTable = HashMap<AsciiString, u32>;
pub type LZWDecodeTable = HashMap<u32, AsciiString>;
pub type LZWEncodedArray = Vec<u32>;
