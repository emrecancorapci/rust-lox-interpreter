use std::{ collections::HashMap, fs, iter::Peekable, str::Chars };

pub fn tokenize(filename: &str) -> i32 {
    let mut result = 0;

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    // Uncomment this block to pass the first stage
    if !file_contents.is_empty() {
        file_contents
            .lines()
            .enumerate()
            .for_each(|(line_index, line)| {
                if tokenize_line(line_index, line) {
                    result = 65;
                }
            });
        println!("EOF  null");
    } else {
        println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
    }

    return result;
}

fn tokenize_line(index: usize, line: &str) -> bool {
    let mut result = false;

    let double_pairs = HashMap::from([
        ("!=", "BANG_EQUAL"),
        ("==", "EQUAL_EQUAL"),
        ("<=", "LESS_EQUAL"),
        (">=", "GREATER_EQUAL"),
        ("!=", "BANG_EQUAL"),
    ]);

    let mut iterator = line.chars().into_iter().peekable();

    while let Some(ch) = iterator.next() {
        if ch == '"' {
            match tokenize_string(&mut iterator, index) {
                true => {
                    result = true;
                }
                false => {}
            }
            continue;
        }

        if let Some(next_ch) = iterator.peek() {
            let peeked = format!("{}{}", ch, next_ch);

            if peeked.as_str() == "//" {
                return result;
            }

            if let Some(value) = double_pairs.get(peeked.as_str()) {
                println!("{value} {peeked} null");
                iterator.next();
                continue;
            }
        }

        if print_pair(&ch).is_err() {
            match ch {
                '\u{0009}' | ' ' => {
                    continue;
                }

                _ => {
                    eprintln!("[line {}] Error: Unexpected character: {ch}", index + 1);
                    result = true;
                }
            }
        }
    }

    return result;
}

fn tokenize_string(iterator: &mut Peekable<Chars>, index: usize) -> bool {
    let mut string = String::new();

    'string_looper: loop {
        match iterator.next() {
            Some('"') => {
                break 'string_looper;
            }
            Some(string_ch) => {
                string.push(string_ch);
            }
            None => {
                eprintln!("[line {}] Error: Unterminated string.", index + 1);
                return true;
            }
        }
    }

    println!("STRING \"{string}\" {string}");
    return false;
}

fn print_pair(ch: &char) -> Result<(), ()> {
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
        ('<', "LESS"),
        ('>', "GREATER"),
        ('/', "SLASH"),
        ('.', "DOT"),
        ('=', "EQUAL"),
        ('!', "BANG"),
    ]);

    match pairs.get(&ch) {
        Some(value) => {
            println!("{value} {ch} null");
            return Ok(());
        }
        None => {
            return Err(());
        }
    }
}
