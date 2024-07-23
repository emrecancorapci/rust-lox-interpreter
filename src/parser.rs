use std::{ fs, iter::Peekable, str::Chars };

pub struct Parser {
    placeholder: u8,
}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {
            placeholder: 1,
        }
    }
    pub(crate) fn parse_file(&mut self, filename: &str) {
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {}", filename);
            String::new()
        });

        if !file_contents.is_empty() {
            file_contents
                .lines()
                .enumerate()
                .for_each(|(line_index, line)| {
                    self.parse_line(line_index, line);
                });
        }
    }

    fn parse_line(&mut self, index: usize, line: &str) {
        let mut iterator = line.chars().peekable();

        while let Some(ch) = iterator.peek() {
            match ch {
                // '"' => self.tokenize_string(&mut iterator, index),
                '0'..='9' => self.parse_number(&mut iterator),
                'a'..='z' | 'A'..='Z' | '_' => self.parse_text(&mut iterator),
                _ => {
                    iterator.next();
                    continue;
                }
                // if self.tokenize_characters(&mut iterator, index).is_break() {
                //     return;
                // }
            }
        }
    }

    fn parse_text(&mut self, iterator: &mut Peekable<Chars<'_>>) {
        let mut string = String::new();

        while let Some(ch) = iterator.peek() {
            if matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                string.push(*ch);
                iterator.next();
                continue;
            } else {
                break;
            }
        }

        if matches!(string.as_str(), "true" | "false" | "nil") {
            println!("{string}");
            return;
        }
    }

    fn parse_number(&mut self, iterator: &mut Peekable<Chars<'_>>) {
        let mut number = String::new();

        while let Some(ch) = iterator.peek() {
            match ch {
                '0'..='9' => {
                    number.push(*ch);

                    iterator.next();
                }
                '.' => {
                    if number.contains('.') {
                        break;
                    } else {
                        number.push(*ch);

                        iterator.next();
                    }
                }
                _ => {
                    break;
                }
            }
        }

        if !number.contains('.') {
            number.push_str(".0");
        } else if number.ends_with('.') {
            number.push('0');
        }

        println!("{number}");
    }
}
