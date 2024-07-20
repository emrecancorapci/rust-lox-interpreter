use std::{ fs, io::{ self, Write } };

pub fn tokenize(filename: &String) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    // Uncomment this block to pass the first stage
    if !file_contents.is_empty() {
        file_contents
            .chars()
            .into_iter()
            .for_each(|read| {
                match read {
                    '\u{0028}' => {
                        let _ = writeln!(io::stdout(), "LEFT_PAREN \u{0028} null");
                    }
                    '\u{0029}' => {
                        let _ = writeln!(io::stdout(), "RIGHT_PAREN \u{0029} null");
                    }
                    '\0'..='\'' | '*'..='\u{d7ff}' | '\u{e000}'..='\u{10ffff}' => todo!(),
                }
            });
        let _ = writeln!(io::stdout(), "EOF  null");
    } else {
        println!("EOF null"); // Placeholder, remove this line when implementing the scanner
    }
}
