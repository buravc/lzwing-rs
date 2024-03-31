use super::types::{self, AsciiString, LZWCodeTable, LZWEncodedArray};

pub struct CompressedFile {
    code_table: types::LZWCodeTable,
    output: LZWEncodedArray,
}

pub fn compress(file_path: &str) -> CompressedFile {
    let read_file = std::fs::read(file_path).expect("file to be readable");
    let mut code_table = LZWCodeTable::new();

    for i in 0..256 {
        code_table.insert(vec![i as u8], i);
    }

    let mut s: AsciiString = vec![read_file[0]];

    let mut output = LZWEncodedArray::with_capacity(read_file.len());

    let mut code_table_index = 256;

    for c in read_file.iter() {
        let s_plus_c: AsciiString = {
            let mut clone = s.clone();
            clone.push(*c);
            clone
        };

        if code_table.contains_key(&s_plus_c) {
            s = s_plus_c;
        } else {
            let codeword = code_table.get(&s).expect("s to be in code_table");
            output.push(*codeword);
            code_table.insert(s_plus_c, code_table_index);
            code_table_index += 1;
            s = vec![*c];
        }
    }

    let codeword = code_table.get(&s).expect("s to be in code_table");
    output.push(*codeword);

    CompressedFile { code_table, output }
}

impl CompressedFile {
    pub fn save_to_file(&self, file_path: &str) {
        std::fs::metadata(file_path).expect_err("file_path to be not existing");
        let flat_output: AsciiString = self
            .output
            .iter()
            .flat_map(|x| x.to_be_bytes().into_iter().collect::<AsciiString>())
            .collect();
        std::fs::write(file_path, flat_output).expect("write to be successful");
    }
}
