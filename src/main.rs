use std::collections::HashSet;

///*
/// This tool generates word magic squares, which are NxM matrices of letters
/// arranged such that every row and every column is a valid dictionary word.
///
/// The user can pass in a custom dictionary file, or the default OS dict will
/// be used.
/// */

/// A simple Dictionary implementation, with `contains` and `len` methods.
/// Clonable, so it can be passed around.
#[derive(Clone)]
struct Dictionary {
    words: HashSet<String>,
}

const ATTEMPT_RENDER_FREQ: usize = 5;

impl Dictionary {
    fn contains(&self, word: &str) -> bool {
        self.words.contains(word)
    }

    /// Return all the words that match a template. A template is a set of
    /// letters or a wildcard (_). For example, "__mon" will match "demon" and
    /// "lemon", but not "human".
    fn search_with_template(&self, template: &str) -> Vec<String> {
        let tmp = template.to_lowercase();
        self.words
            .iter()
            .filter(|word| {
                // Short-circuit on length:
                if word.len() != tmp.len() {
                    return false;
                }
                let mut chars = word.chars();
                for c in tmp.chars() {
                    if c == '_' {
                        chars.next();
                    } else {
                        if chars.next() != Some(c) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|s| s.to_string())
            .collect()
    }

    fn count_with_template(&self, template: &str) -> usize {
        self.search_with_template(template).len()
    }

    /// Create a new dictionary from a file.
    /// The file should contain one word per line.
    /// The words should be lowercase.
    ///
    /// # Arguments
    /// * `path` - The path to the dictionary file.
    ///
    /// # Returns
    /// * Ok(A new dictionary)
    /// * Err(String) if the file could not be read.
    fn from_file(path: &str) -> Result<Dictionary, String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let mut words = HashSet::new();
        for line in reader.lines() {
            words.insert(line.map_err(|e| e.to_string())?);
        }

        Ok(Dictionary { words })
    }

    /// Create a new dictionary from the OS dictionary.
    ///
    /// # Returns
    /// * Ok(A new dictionary)
    /// * Err(String) if the OS dictionary could not be read.
    fn from_os_dict() -> Result<Dictionary, String> {
        use std::process::Command;

        let output = Command::new("cat")
            .arg("/usr/share/dict/words")
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err("Could not read OS dictionary".to_string());
        }

        let words = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        let words = words
            .lines()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<String>>();

        Ok(Dictionary { words })
    }
}

/// Check if a word is a valid dictionary word.
///
/// # Arguments
///
/// * `word` - The word to check.
/// * `dict` - The dictionary to check against.
///
/// # Returns
///
/// * `true` if the word is valid, `false` otherwise.
fn is_valid_word(word: &str, dict: &Dictionary) -> bool {
    // All letters are alphanumeric, longer than 2 chars, and in the dictionary
    word.len() > 2 && word.chars().all(|c| c.is_alphanumeric()) && dict.contains(word)
}

/// A magic square is a NxM matrix of letters arranged such that every row and
/// every column is a valid dictionary word.
/// This struct represents a magic square.
/// It is a wrapper around a 2D vector of chars.
/// The `fill` method will fill the square with letters.
/// The `print` method will print the square to stdout.
/// The `empty` method will create an empty square.
struct MagicSquare {
    square: Vec<Vec<char>>,
    dict: Dictionary,
    _attempt: usize,
}

impl MagicSquare {
    /// Create an empty magic square.
    ///
    /// # Arguments
    ///
    /// * `rows` - The number of rows in the square.
    /// * `cols` - The number of columns in the square.
    /// * `dict` - The dictionary to use.
    ///
    /// # Returns
    ///
    /// * A new empty magic square.
    fn empty(rows: usize, cols: usize, dict: &Dictionary) -> MagicSquare {
        MagicSquare {
            square: vec![vec!['_'; cols]; rows],
            dict: dict.clone(),
            _attempt: 0,
        }
    }

    fn set(&mut self, row: usize, col: usize, c: char) {
        self.square[row][col] = c;
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.square[row][col]
    }

    /// Fill the square with letters.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the square was filled successfully.
    /// * `Err(String)` if the square could not be filled.
    fn fill(&mut self) -> Result<(), String> {
        // Starting at the top left, fill the square with letters such that
        // every row and column is a valid dictionary word. This is done by
        // recursively filling the square with letters, and backtracking if
        // any of the crosswords become a template with no valid matches.

        // Get the first un-filled square
        let (row, col) = self.find_first_empty_square().unwrap();

        // Fill the square with letters
        self.fill_helper(row, col)
    }

    fn find_first_empty_square(&self) -> Option<(usize, usize)> {
        for (row, row_vec) in self.square.iter().enumerate() {
            for (col, c) in row_vec.iter().enumerate() {
                if *c == '_' {
                    return Some((row, col));
                }
            }
        }

        None
    }

    /// Helper function for `fill`.
    /// Recursively fill the square with letters.
    /// If any of the crosswords become a template with no valid matches,
    /// backtrack and try a different letter.
    /// If all letters have been tried and none of them work, return an error.
    /// If the square is filled successfully, return `Ok(())`.
    /// This function is recursive.
    fn fill_helper(&mut self, row: usize, col: usize) -> Result<(), String> {
        // If we've reached the end of the square, we're done
        if row == self.square.len() {
            return Ok(());
        }

        // If we've reached the end of the row, move to the next row
        if col == self.square[row].len() {
            return self.fill_helper(row + 1, 0);
        }

        // Try every letter in the alphabet
        for c in 'a'..='z' {
            // self._attempt += 1;
            // If the letter is valid, set it and try to fill the rest of the square
            if self.is_valid_letter(row, col, c) {
                // Only draw every Nth attempt
                // if self._attempt++ % ATTEMPT_RENDER_FREQ == 0 {
                self.clear_and_print();
                // }
                self.set(row, col, c);
                if let Ok(()) = self.fill_helper(row, col + 1) {
                    return Ok(());
                }
            }
        }

        // If we've tried every letter and none of them work, backtrack
        self.set(row, col, '_');
        Err(format!("Could not fill square at ({}, {})", row, col))
    }

    /// Check if a letter is valid at a given position in the square.
    /// A letter is valid if its crosswords are valid words or valid templates.
    fn is_valid_letter(&self, row: usize, col: usize, c: char) -> bool {
        // Check if the letter is valid in the row
        let ww = self.get_row(row);
        // Set the col'th letter to c
        let ww = ww
            .iter()
            .enumerate()
            .map(|(i, &x)| if i == col { c } else { x })
            .collect::<Vec<char>>();
        if !self.is_valid_word_or_template(&ww) {
            return false;
        }

        // Check if the letter is valid in the column
        let ww = self.get_col(col);
        // Set the row'th letter to c
        let www = ww
            .iter()
            .enumerate()
            .map(|(i, &x)| if i == row { c } else { x })
            .collect::<Vec<char>>();
        if !self.is_valid_word_or_template(&www) {
            return false;
        }

        true
    }

    /// Get the row at a given index.
    fn get_row(&self, row: usize) -> Vec<char> {
        self.square[row].clone()
    }

    /// Get the column at a given index.
    fn get_col(&self, col: usize) -> Vec<char> {
        self.square.iter().map(|r| r[col]).collect()
    }

    /// Check if a word or template is valid.
    /// A word is valid if it is a valid dictionary word or has nonzero
    /// template matches.
    fn is_valid_word_or_template(&self, word: &Vec<char>) -> bool {
        let word_as_str = word.iter().collect::<String>();
        // Check if the word is a valid dictionary word
        if self.dict.contains(word_as_str.as_str()) {
            return true;
        }

        // Check if the word has nonzero template matches
        if self.dict.count_with_template(word_as_str.as_str()) > 0 {
            return true;
        }

        false
    }

    /// Print the square to stdout.
    fn print(&self) {
        for row in self.square.iter() {
            for &c in row.iter() {
                print!("{} ", c);
            }
            println!("");
        }
    }

    // Print, clearing the screen first
    fn clear_and_print(&self) {
        print!("{}[2J", 27 as char);
        self.print();
    }
}

fn main() {
    // If called with a file name, use that file as the dictionary
    let dict = if let Some(filename) = std::env::args().nth(1) {
        Dictionary::from_file(filename.as_str()).unwrap()
    } else {
        // Otherwise, use the default OS dictionary
        Dictionary::from_os_dict().unwrap()
    };

    // If called with a string word, use that as the first word (comes before
    // the dict path)
    let first_word = if let Some(word) = std::env::args().nth(2) {
        word
    } else {
        "_____".to_string()
    };

    // Create a dictionary from the default OS dictionary
    // let dict = Dictionary::from_os_dict().unwrap();

    // Create a 4x4 magic square
    let mut square = MagicSquare::empty(4, first_word.len(), &dict);

    // Set the first row:
    // square.set(0, 0, 'j');
    // square.set(0, 1, 'o');
    // square.set(0, 2, 'i');
    // square.set(0, 3, 'n');
    // square.set(0, 4, 't');

    let w1 = first_word.chars().collect::<Vec<char>>();
    for (i, &c) in w1.iter().enumerate() {
        square.set(0, i, c);
    }
    square.fill().unwrap();

    // Print the square
    print!("Magic square:\n");
    square.print();

    // Print the capitalized letters all concatenated
    let mut capitalized = String::new();
    for row in square.square.iter() {
        for &c in row.iter() {
            capitalized.push(c.to_ascii_uppercase());
        }
    }
    println!("\n{}", capitalized);

    // // Satisfy the "_ _ M O " template
    // let re = dict.search_with_template("aaru");
    // println!("{} words satisfy the \"_ _ M O \" template", re.len());
    // for word in re {
    //     println!("{}", word);
    // }
}
