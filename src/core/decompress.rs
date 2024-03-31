use super::types::{AsciiString, LZWCodeTable, LZWEncodedArray};

pub struct DecmpressedFile {
    code_table: LZWCodeTable,
    output: AsciiString,
}

pub fn decompress(file_path: &str) -> DecmpressedFile {
    let read_file = std::fs::read(file_path).expect("file to be readable");
    let mut code_table = LZWCodeTable::new();

    for i in 0..256 {
        code_table.insert(vec![i as u8], i);
    }
    let mut code_table_index = 256;

    let mut old = vec![read_file[0]];
    let mut output = AsciiString::with_capacity(read_file.len());

    let codeword = code_table.get(&old).expect("s to be in code_table");
    // output.push(*codeword);

    for new in read_file.iter() {
        if !code_table.contains_key(&vec![*new]) {
            todo!()
        }
    }

    DecmpressedFile { code_table, output }
}

impl DecmpressedFile {
    pub fn save_to_file(&self, file_path: &str) {
        todo!()
    }
}
