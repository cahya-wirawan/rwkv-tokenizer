use std::collections::HashMap;
use pyo3::prelude::*;
use std::str;
use rwkv_tokenizer;

#[derive(Debug)]
#[pyclass]
pub(crate) struct WorldTokenizer {
    tokenizer: rwkv_tokenizer::WorldTokenizer,
}

#[pymethods]
impl WorldTokenizer {
    #[new]
    pub(crate) fn new(filename: &str) -> WorldTokenizer {
        WorldTokenizer {
            tokenizer: rwkv_tokenizer::WorldTokenizer::new(Option::from(filename)).unwrap()
        }
    }

    pub(crate) fn encode(&self, word: &str) -> Vec<u16> {
        self.tokenizer.encode(word)
    }

    pub(crate) fn encode_batch(&self, word_list: Vec<String>) -> Vec<Vec<u16>> {
        self.tokenizer.encode_batch(word_list)
    }

    pub(crate) fn decode(&self, vec: Vec<u16>) -> String {
        return self.tokenizer.decode(vec);
    }

    pub(crate) fn vocab_size(&self) -> usize {
        return self.tokenizer.vocab_size();
    }

    pub(crate) fn get_vocab(&self) -> HashMap<String, usize> {
        return self.tokenizer.get_vocab();
    }
}


#[pymodule]
fn pyrwkv_tokenizer(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WorldTokenizer>()?;
    Ok(())
}
