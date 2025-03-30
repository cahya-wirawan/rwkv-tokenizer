# RWKV Tokenizer


[![GitHub Actions Status](https://github.com/cahya-wirawan/rwkv-tokenizer/actions/workflows/CI.yml/badge.svg)](https://github.com/cahya-wirawan/rwkv-tokenizer/actions/)
[![Crates.io Version](https://img.shields.io/crates/v/rwkv-tokenizer.svg)](https://crates.io/crates/rwkv-tokenizer)
[![Crates.io Downloads](https://img.shields.io/crates/d/rwkv-tokenizer.svg)](https://crates.io/crates/rwkv-tokenizer)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/cahya-wirawan/rwkv-tokenizer/blob/main/LICENSE.txt)


A fast RWKV Tokenizer written in Rust that supports the World Tokenizer used by the 
[RWKV](https://github.com/BlinkDL/RWKV-LM) v5 and v6 models.

## Installation
To use rwkv-tokenizer, add the following to your Cargo.toml file:
```
[dependencies]
rwkv-tokenizer = "0.9.1"
```
## Usage
```
use rwkv_tokenizer::WorldTokenizer;

let text = "Today is a beautiful day. 今天是美好的一天。";
let tokenizer = WorldTokenizer::new(None).unwrap();
let tokens_ids = tokenizer.encode(text);
let decoding = tokenizer.decode(tokens_ids).unwrap();
println!("tokens: {:?}", tokens_ids);
println!("text: {:?}", text);
println!("decoding: {:?}", decoding);
```

## Performance and Validity Test

We compared the encoding results of the Rust RWKV Tokenizer and the original tokenizer using
the English Wikipedia and Chinese poetries datasets. Both results are identical. The Rust RWKV Tokenizer also 
passes [the original tokenizer's unit test](https://github.com/BlinkDL/ChatRWKV/blob/main/tokenizer/rwkv_tokenizer.py). 
The following steps describe how to do the unit test:
```

```

We did a performance comparison on [the simple English Wikipedia dataset 20220301.en](https://huggingface.co/datasets/legacy-datasets/wikipedia) among following tokenizer:
- The original RWKV tokenizer (BlinkDL)
- Huggingface implementaion of RWKV tokenizer
- Huggingface LLama tokenizer
- Huggingface Mistral tokenizer
- Bert tokenizer
- OpenAI Tiktoken
- The Rust RWKV tokenizer

The comparison is done using this [jupyter notebook](https://github.com/cahya-wirawan/rwkv-tokenizer/blob/main/tools/rwkv_tokenizers.ipynb) in a M2 Mac mini. The Rust RWKV 
tokenizer is around 17x faster than the original tokenizer and 9.6x faster than OpenAI Tiktoken.

![performance-comparison](https://media.githubusercontent.com/media/cahya-wirawan/rwkv-tokenizer/main/data/performance-comparison.png)

## Changelog
- Version 0.9.1
  - Added utf8 error handling to decoder
- Version 0.9.0
  - Added multithreading for the function encode_batch()
  - Added batch/multithreading comparison
- Version 0.3.0
  - Fixed the issue where some characters were not encoded correctly

*This tokenizer is my very first Rust program, so it might still have many bugs and silly codes :-)*
