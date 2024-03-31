use std::collections::HashMap;

pub type AsciiString = Vec<u8>;
pub type LZWCodeTable = HashMap<AsciiString, u32>;
pub type LZWEncodedArray = Vec<u32>;
