use std::collections::HashSet;

/// A simple Dictionary implementation, with `contains` and `len` methods.
/// Clonable, so it can be passed around.
#[derive(Clone)]
pub struct Dictionary {
    words: HashSet<String>,
}

impl Dictionary {
    pub(crate) fn contains(&self, word: &str) -> bool {
        self.words.contains(word)
    }

    /// Return all the words that match a template. A template is a set of
    /// letters or a wildcard (_). For example, "__mon" will match "demon" and
    /// "lemon", but not "human".
    pub(crate) fn search_with_template(&self, template: &str) -> Vec<String> {
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

    pub(crate) fn count_with_template(&self, template: &str) -> usize {
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
    pub(crate) fn from_file(path: &str) -> Result<Dictionary, String> {
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
    pub(crate) fn from_os_dict() -> Result<Dictionary, String> {
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

struct TemplateTreeNode {
    word: String,
    subtemplates: Vec<TemplateTreeNode>,
}
struct TemplateTree {
    root_template: TemplateTreeNode,
}

impl TemplateTreeNode {
    pub(crate) fn matches(&self, template: &str) -> bool {
        let tmp = template.to_lowercase();

        // Short-circuit on length:
        if self.word.len() != tmp.len() {
            return false;
        }
        let mut chars = self.word.chars();
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
    }
}

impl TemplateTree {
    fn from_dict(dict: Dictionary) -> TemplateTree {
        //
        TemplateTree {
            root_template: TemplateTreeNode {
                word: "____".to_string(),
                subtemplates: vec![],
            },
        }
    }
}
