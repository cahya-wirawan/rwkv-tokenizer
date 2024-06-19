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

    pub(crate) fn decode(&self, vec: Vec<u16>) -> String {
        return self.tokenizer.decode(vec);
    }
}


#[pymodule]
fn pyrwkv_tokenizer(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WorldTokenizer>()?;
    Ok(())
}
