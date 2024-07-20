use std::{ collections::HashMap, fs, io::{ self, Write } };

pub fn tokenize(filename: &str) -> i32 {
    let mut result = 0;

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    let pairs = HashMap::from([
        ('(', "LEFT_PAREN"),
        (')', "RIGHT_PAREN"),
        ('{', "LEFT_BRACE"),
        ('}', "RIGHT_BRACE"),
        (';', "SEMICOLON"),
        (',', "COMMA"),
        ('+', "PLUS"),
        ('-', "MINUS"),
        ('*', "STAR"),
        ('!', "BANG_EQUAL"),
        ('=', "EQUAL_EQUAL"),
        ('<', "LESS_EQUAL"),
        ('>', "GREATER_EQUAL"),
        ('!', "BANG_EQUAL"),
        ('<', "LESS"),
        ('>', "GREATER"),
        ('/', "SLASH"),
        ('.', "DOT"),
    ]);

    // Uncomment this block to pass the first stage
    if !file_contents.is_empty() {
        file_contents
            .lines()
            .into_iter()
            .enumerate()
            .for_each(|(line_num, line)| {
                line.chars()
                    .into_iter()
                    .for_each(|ch| {
                        match pairs.get(&ch) {
                            Some(val) => {
                                println!("{val} {ch} null");
                            }
                            None => {
                                result = 65;
                                eprintln!(
                                    "[line {}] Error: Unexpected character: {ch}",
                                    line_num + 1
                                )
                            }
                        }
                    })
            });
        println!("EOF  null");
    } else {
        println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
    }
    
    return result;
}
