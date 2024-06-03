mod trie;
extern crate unescape;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use trie::Trie;
use unescape::unescape;

#[derive(Debug)]
#[pyclass]
pub(crate) struct Tokenizer {
    tokens: Vec<String>,
    trie: Trie
}

#[pymethods]
impl Tokenizer {
    #[new]
    pub(crate) fn new(filename: &str) -> io::Result<Self> {
        let mut tokenizer = Tokenizer {
            tokens: Vec::new(),
            trie: Trie::new()
        };
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        let re = Regex::new(r"(\d+)\s+(b?)(.+)\s+(\d+)").unwrap();
        tokenizer.tokens.push("".to_string());
        for line in reader.lines() {
            let line = line?;
            if let Some(captures) = re.captures(&line) {
                let id = captures[1].parse::<u16>().unwrap();
                let is_byte = captures[2].to_string();
                let _length = captures[4].parse::<usize>().unwrap();
                let mut string: String = captures[3].to_string();
                string = string[1..string.len()-1].parse().unwrap();
                let sbytes: Vec<u8>;
                if is_byte.len() == 0 {
                    string = unescape(string.as_str()).unwrap();
                    sbytes = string.clone().into_bytes();
                } else {
                    sbytes = hex_to_bytes(string.as_str()).unwrap();
                }
                tokenizer.trie.insert(&sbytes, id);
                tokenizer.tokens.push(string.to_string());
            }
            else {
                println!("Line with issue: {:?}", line)
            }
        }
        Ok(tokenizer)
    }

    pub(crate) fn encode(&self, word: &str) -> Vec<u16> {
        self.trie.tokenize(word)
    }

    pub(crate) fn decode(&self, vec: Vec<u16>) -> String {
        let mut result: String = "".to_owned();
        for index in vec.iter() {
            result.push_str(&*self.tokens[*index as usize]);
        }
        return result;
    }
}

fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {
    let hex = hex.replace("\\x", "");
    if hex.len() % 2 == 0 {
        (0..hex.len())
            .step_by(2)
            .map(|i| hex.get(i..i + 2)
                .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}

#[pymodule]
fn rwkv_tokenizer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    Ok(())
}
