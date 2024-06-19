# RWKV Tokenizer

A fast RWKV Tokenizer written in Rust that supports the World Tokenizer used by the 
[RWKV](https://github.com/BlinkDL/RWKV-LM) v5 and v6 models.

## Installation
Install the rwkv-tokenizer python module:
```
$ cargo add rwkv-tokenizer@=0.7.6
```
## Usage
```
use rwkv_tokenizer::WorldTokenizer;

let text = "Today is a beautiful day. 今天是美好的一天。";
let tokenizer = WorldTokenizer::new(None).unwrap();
let tokens_ids = tokenizer.encode(text);
let decoding = tokenizer.decode(tokens_ids);
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

The comparison is done using this [jupyter notebook](tools/rwkv_tokenizers.ipynb) in a M2 Mac mini. The Rust RWKV 
tokenizer is around 17x faster than the original tokenizer and 9.6x faster than OpenAI Tiktoken.

![performance-comparison](data/performance-comparison.png)

## Bugs
~~There are still bugs where some characters are not encoded correctly.~~ The bug have been fixed in the version 0.3.0.
*This tokenizer is my very first Rust program, so it might still have many bugs and silly codes :-)*