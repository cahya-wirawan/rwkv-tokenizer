use wasm_bindgen::prelude::*;
use rwkv_tokenizer;
extern crate console_error_panic_hook;
use wasm_bindgen::JsValue;
use js_sys::{Uint16Array, Array};

static VOCAB_BUFFER: &'static [u8] = include_bytes!("../../../rwkv-tokenizer/assets/rwkv_vocab_v20230424.txt");

#[wasm_bindgen]
pub(crate) struct WorldTokenizer {
    tokenizer: rwkv_tokenizer::WorldTokenizer,
}


#[wasm_bindgen]
impl WorldTokenizer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WorldTokenizer {
        WorldTokenizer {
            tokenizer: rwkv_tokenizer::WorldTokenizer::from_buffer(VOCAB_BUFFER).unwrap()
        }
    }

    #[wasm_bindgen]
    pub fn encode(&self, word: &str) -> Uint16Array {
        let encoded_vec = self.tokenizer.encode(word);
        // Convert Vec<u16> to Uint16Array
        Uint16Array::from(&encoded_vec[..])
    }

    #[wasm_bindgen]
    pub fn encode_batch(&self, word_list: Vec<String>) -> Array {
        let encoded_batch = self.tokenizer.encode_batch(word_list);
        let js_array = Array::new_with_length(encoded_batch.len() as u32);
        for (i, vec) in encoded_batch.into_iter().enumerate() {
            let uint16_array = Uint16Array::from(&vec[..]);
            js_array.set(i as u32, uint16_array.into());
        }
        js_array
    }

    #[wasm_bindgen]
    pub fn decode(&self, vec: Vec<u16>) -> String {
        return self.tokenizer.decode(vec).unwrap();
    }

    #[wasm_bindgen]
    pub fn vocab_size(&self) -> usize {
        return self.tokenizer.vocab_size();
    }

    #[wasm_bindgen]
    pub fn get_vocab_as_json(&self) -> Result<JsValue, JsValue> {
        let vocab = self.tokenizer.get_vocab();
        // Serialize the HashMap into a JSON string
        let json_string = serde_json::to_string(&vocab)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize vocab: {}", e)))?;

        // Convert the JSON string to a JsValue and return it
        Ok(JsValue::from_str(&json_string))
    }
}