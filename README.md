# RWKV Tokenizer


[![GitHub Actions Status](https://github.com/cahya-wirawan/rwkv-tokenizer/actions/workflows/CI.yml/badge.svg)](https://github.com/cahya-wirawan/rwkv-tokenizer/actions/)
[![Pypi.org Version](https://img.shields.io/pypi/v/pyrwkv-tokenizer.svg)](https://pypi.org/project/pyrwkv-tokenizer/)
[![Pypi.org Downloads](https://img.shields.io/pypi/dd/pyrwkv-tokenizer)](https://pypi.org/project/pyrwkv-tokenizer/)
[![Crates.io Version](https://img.shields.io/crates/v/rwkv-tokenizer.svg)](https://crates.io/crates/rwkv-tokenizer)
[![Crates.io Downloads](https://img.shields.io/crates/d/rwkv-tokenizer.svg)](https://crates.io/crates/rwkv-tokenizer)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/cahya-wirawan/rwkv-tokenizer/blob/main/LICENSE.txt)


A fast RWKV Tokenizer written in Rust that supports the World Tokenizer used by the 
[RWKV](https://github.com/BlinkDL/RWKV-LM) v5 and newer models.

## Installation and Usage

### Rust

Add the **rwkv-tokenizer** to the dependecy list in Cargo.toml or add it using `cargo add rwkv-tokenizer`.

#### Usage
Following is a simple Rust code to use it:
```
use rwkv_tokenizer;

fn main() {
    let tokenizer = rwkv_tokenizer::WorldTokenizer::new(None).unwrap();
    let text = "Today is a beautiful day. 今天是美好的一天。";
    let ids = tokenizer.encode(text);
    let tokens = tokenizer.decode(ids.clone()).unwrap();
    println!("Text: {text}");
    println!("Ids: {ids:?}");
    println!("Tokens: {tokens:?}");
}
```
And run it with `cargo run`:
```
$ cargo run
   Compiling hello_rwkv v0.1.0 (/home/cahya/Work/MachineLearning/Rust/hello_rwkv)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/hello_rwkv`
Text: Today is a beautiful day. 今天是美好的一天。
Ids: [33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080]
Tokens: "Today is a beautiful day. 今天是美好的一天。"
```

### Python binding

Install the rwkv-tokenizer python module:
```
$ pip install pyrwkv-tokenizer
```
#### Usage
```
>>> import pyrwkv_tokenizer
>>> tokenizer = pyrwkv_tokenizer.RWKVTokenizer()
>>> tokenizer.encode("Today is a beautiful day. 今天是美好的一天。")
[33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080]
>>> tokenizer.decode([33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080])
'Today is a beautiful day. 今天是美好的一天。'
>>> tokenizer.encode_batch(["Today is a beautiful day.", " 今天是美好的一天。"])
[[33520, 4600, 332, 59219, 21509, 47], [33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080]]
```

### WebAssembly binding

There are two WebAssembly modules: Nodejs and Web module.
Install the RWKV Tokenizer WebAssembly **rwkv-tokenizer** if the application runs only as nodejs application or WebAsesembly **rwkv-tokenizer-web** if the application is a web application. Following is an example to install and run a nodejs application:

```
$ npm install rwkv-tokenizer
```

#### Usage

Create an example javascript file wasm.js with following content:
```
const { WorldTokenizer } = require('rwkv-tokenizer');

async function runWasm() {
  try {
    const textToEncode = "Today is a beautiful day. 今天是美好的一天。";
    const tokenizer = new WorldTokenizer();
    let encodedText = tokenizer.encode(textToEncode);
    console.log(`Encoded text using tokenizer:`, encodedText);
    let decodedText = tokenizer.decode(encodedText);
    console.log(`Decoded text using tokenizer:`, decodedText);
    let encodedTextBatch = tokenizer.encode_batch([textToEncode, "Another sentence."]);
    console.log(`Encoded text using tokenizer:`, encodedTextBatch);

  } catch (error) {
    console.error("Error loading or using WASM module:", error);
  }
}

runWasm();
```

and execute it:
``` bash
$ node wasm.js
```
The ouput should be like:
```
Encoded text using tokenizer: Uint16Array(16) [
  33520,  4600,   332, 59219,
  21509,    47,    33, 10381,
  11639, 13091, 15597, 11685,
  14734, 10250, 11639, 10080
]
Decoded text using tokenizer: Today is a beautiful day. 今天是美好的一天。
Encoded text using tokenizer: [
  Uint16Array(16) [
    33520,  4600,   332, 59219,
    21509,    47,    33, 10381,
    11639, 13091, 15597, 11685,
    14734, 10250, 11639, 10080
  ],
  Uint16Array(3) [ 48358, 57192, 47 ]
]

```

A demo of the Webassembly RWKV Tokenizer running as web application is available at https://cahya-wirawan.github.io/rwkv-tokenizer-wasm/ with its source code https://github.com/cahya-wirawan/rwkv-tokenizer-wasm.

## Performance and Validity Test

We compared the encoding results of the Rust RWKV Tokenizer and the original tokenizer using
the English Wikipedia and Chinese poetries datasets. Both results are identical. The Rust RWKV Tokenizer also 
passes [the original tokenizer's unit test](https://github.com/BlinkDL/ChatRWKV/blob/main/tokenizer/rwkv_tokenizer.py). 
The following steps describe how to do the unit test:
```
$ pip install pytest pyrwkv-tokenizer
$ git clone https://github.com/cahya-wirawan/rwkv-tokenizer.git
$ cd rwkv-tokenizer
$ pytest
```

We did a performance comparison on [the simple English Wikipedia dataset 20220301.simple](https://huggingface.co/datasets/legacy-datasets/wikipedia)* among following tokenizer:
- The original RWKV tokenizer (BlinkDL)
- Huggingface implementaion of RWKV tokenizer
- Huggingface LLama tokenizer
- Huggingface Mistral tokenizer
- Bert tokenizer
- OpenAI Tiktoken
- The Rust RWKV tokenizer

The comparison is done using this [jupyter notebook](tools/rwkv_tokenizers.ipynb) in a M2 Mac mini. The Rust RWKV 
tokenizer is around 17x faster than the original tokenizer and 9.6x faster than OpenAI Tiktoken.

![performance-comparison](data/performance-comparison.png)

We updated the Rust RWKV world tokenizer to support batch encoding with multithreading. We ran the same comparison
[script](tools/test_tiktoken-huggingface-rwkv.py)  from the [Huggingface Tokenizers](https://github.com/huggingface/tokenizers)
with the additional rwkv tokenizer. The result shows that the rwkv world tokenizer is significantly faster than 
the Tiktoken and Huggingface tokenizers in all numbers of threads and document sizes (on average, its speed is ten times faster).

![performance-comparison](data/performance-comparison-multithreading.png) 

*The simple English Wikipedia dataset can be downloaded as jsonl file from
https://huggingface.co/datasets/cahya/simple-wikipedia/resolve/main/simple-wikipedia.jsonl?download=true

## Tools using this tokenizer

We also created the [json2bin](https://github.com/cahya-wirawan/json2bin) application to convert datasets from JSONL format 
into binidx format, a data format used for training RWKV models. It uses multithreading to scale up the performance and 
can convert a dataset more than 70 times faster (around 360 MB/s) than the original 
[json2binidx_tool](https://github.com/Abel2076/json2binidx_tool) written in Python.

## Changelog
- Version 0.10.0
  - Added a function to create the toknizer from a vocabulary stored in a buffer.
  - Added WebAssembly binding
- Version 0.9.1
  - Added utf8 error handling to decoder
- Version 0.9.0
  - Added multithreading for the function encode_batch()
  - Added batch/multithreading comparison
- Version 0.3.0
  - Fixed the issue where some characters were not encoded correctly

*This tokenizer is my very first Rust program, so it might still have many bugs and silly codes :-)*
