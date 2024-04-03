use super::types::{AsciiString, LZWDecodeTable};

pub struct DecompressedFile {
    code_table: LZWDecodeTable,
    output: AsciiString,
}

pub fn decompress(file_path: &str) -> DecompressedFile {
    let read_file = std::fs::read(file_path).expect("file to be readable");
    let mut code_table = LZWDecodeTable::new();
    let mut output = AsciiString::with_capacity(read_file.len() / 4);

    for i in 0..256 {
        code_table.insert(i, vec![i as u8]);
    }
    let mut code_table_index = 256;

    let mut old = u32::from_be_bytes(read_file[0..4].try_into().unwrap());
    let mut s = code_table
        .get(&old)
        .expect("encoded value to be present in the code table")
        .clone();

    output.append(&mut s.clone());

    let mut c = s[0];

    let mut i = 0;
    for new in read_file
        .windows(4)
        .step_by(4)
        .skip(1)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
    {
        // println!("i: {}", i);
        // println!("code_table: {:?}", code_table);
        // println!("s: {:?}", s);

        if !code_table.contains_key(&new) {
            s = code_table
                .get(&old)
                .expect("encoded value to be present in the code table")
                .clone();
            // println!("old: {}", old);

            s.push(c);
        } else {
            s = code_table
                .get(&new)
                .expect("encoded value to be present in the code table")
                .clone();

            // println!("new: {}", new);
        }

        output.append(&mut s.clone());
        // println!("s: {:?}", s);
        c = s[0];

        let old_plus_c = {
            let mut clone = code_table
                .get(&old)
                .expect("encoded value to be present in the code table")
                .clone();
            clone.push(c);
            clone
        };

        code_table.insert(code_table_index, old_plus_c);
        code_table_index += 1;

        old = new;
    }

    DecompressedFile { code_table, output }
}

impl DecompressedFile {
    pub fn save_to_file(&self, file_path: &str) {
        std::fs::metadata(file_path).expect_err("file_path to be not existing");
        std::fs::write(file_path, &self.output).expect("write to be successful");
    }
}
