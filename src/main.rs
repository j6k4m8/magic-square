mod dictionary;

use std::process::exit;

use dictionary::Dictionary;

const ATTEMPT_RENDER_FREQ: usize = 5;

///*
/// This tool generates word magic squares, which are NxM matrices of letters
/// arranged such that every row and every column is a valid dictionary word.
///
/// The user can pass in a custom dictionary file, or the default OS dict will
/// be used.
/// */

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
    editable_mask: Vec<Vec<bool>>,
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
            editable_mask: vec![vec![true; cols]; rows],
            dict: dict.clone(),
            _attempt: 0,
        }
    }

    fn set(&mut self, row: usize, col: usize, c: char) {
        self.square[row][col] = c;
    }
    fn set_and_harden(&mut self, row: usize, col: usize, c: char) {
        self.square[row][col] = c;
        if c != '_' {
            self.editable_mask[row][col] = false;
        }
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
                if *c == '_' && self.editable_mask[row][col] {
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

        // If this is a masked cell, move on to the next one:
        if !self.editable_mask[row][col] {
            // let (nrow, ncol) = self.find_first_empty_square().unwrap();
            // return self.fill_helper(nrow, ncol);
            return Ok(());
        }

        // Try every letter in the alphabet.
        // TODO: Randomized order??
        for c in 'a'..='z' {
            self._attempt += 1;
            // If the letter is valid, set it and try to fill the rest of the square
            if self.is_valid_letter(row, col, c) {
                // Only draw every Nth attempt
                if self._attempt % ATTEMPT_RENDER_FREQ == 0 {
                    self.clear_and_print();
                }
                self.set(row, col, c);
                if self.find_first_empty_square().is_none() {
                    return Ok(());
                }
                let (nrow, ncol) = self.find_first_empty_square().unwrap();
                if let Ok(()) = self.fill_helper(nrow, ncol) {
                    return Ok(());
                }
                // if let Ok(()) = self.fill_helper(row, col + 1) {
                //     return Ok(());
                // }
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
        if self.dict.contains(word_as_str.as_str()) || self.dict.count_with_template(word_as_str.as_str()) > 0 {
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
    let fixed_chars = if let Some(word) = std::env::args().nth(2) {
        word
    } else {
        "_____".to_string()
    };

    // If called with an integer as 3rd argument, use that as the number of
    // rows in the puzzle:
    let row_count = if let Some(rows) = std::env::args().nth(3) {
        rows.parse::<usize>().unwrap()
    } else {
        4
    };

    let fixed_char_words: Vec<&str> = fixed_chars.split("/").collect();

    // Create a dictionary from the default OS dictionary
    // let dict = Dictionary::from_os_dict().unwrap();
    let column_count = fixed_char_words[0].len();

    // Create a 4x4 magic square
    let mut square = MagicSquare::empty(row_count, column_count, &dict);

    // Set the first row:
    // square.set(0, 0, 'j');
    // square.set(0, 1, 'o');
    // square.set(0, 2, 'i');
    // square.set(0, 3, 'n');
    // square.set(0, 4, 't');

    // let w1 = first_word.chars().collect::<Vec<char>>();
    // for (i, &c) in w1.iter().enumerate() {
    //     square.set_and_harden(0, i, c);
    // }
    for (i, &c) in fixed_chars
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .filter(|x| **x != '/')
        .enumerate()
    {
        let row = i / column_count;
        let col = i % column_count;
        square.set_and_harden(row, col, c);
    }
    let fillres = square.fill();
    if fillres.is_err() {
        println!("Could not fill square.");
        exit(1);
    }

    // Print the square
    print!("{}[2J", 27 as char);

    for row in 0..square.square.len() {
        let rowv = square.get_row(row);
        let rowstr: Vec<String> = rowv.iter().map(|f| f.to_string()).collect();
        println!("{}", rowstr.join(""));
    }
    for col in 0..square.square[0].len() {
        let colv = square.get_col(col);
        let colstr: Vec<String> = colv.iter().map(|f| f.to_string()).collect();
        println!("{}", colstr.join(""));
    }
    println!("");

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
