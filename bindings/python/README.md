# RWKV Tokenizer

[![GitHub Actions Status](https://github.com/cahya-wirawan/rwkv-tokenizer/actions/workflows/CI.yml/badge.svg)](https://github.com/cahya-wirawan/rwkv-tokenizer/actions/)
[![Pypi.org Version](https://img.shields.io/pypi/v/pyrwkv-tokenizer.svg)](https://pypi.org/project/pyrwkv-tokenizer/)
[![Pypi.org Downloads](https://img.shields.io/pypi/dd/pyrwkv-tokenizer)](https://pypi.org/project/pyrwkv-tokenizer/)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/cahya-wirawan/rwkv-tokenizer/blob/main/LICENSE.txt)


A fast RWKV Tokenizer written in Rust that supports the World Tokenizer used by the 
[RWKV](https://github.com/BlinkDL/RWKV-LM) v5 and v6 models.

## Installation
Install the rwkv-tokenizer python module:
```
$ pip install pyrwkv-tokenizer
```
## Usage
```
>>> import pyrwkv_tokenizer
>>> tokenizer = pyrwkv_tokenizer.RWKVTokenizer()
>>> tokenizer.encode("Today is a beautiful day. 今天是美好的一天。")
[33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080]
>>> tokenizer.decode([33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080])
'Today is a beautiful day. 今天是美好的一天。'
>>> tokenizer.encode_batch(["Today is a beautiful day.", " 今天是美好的一天。"])
[[33520, 4600, 332, 59219, 21509, 47], [33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080]]
>>> tokenizer.encode_parallel(large_text)  # same ids as encode(), spread over all cores

```
Use `encode_batch` for many separate texts, and `encode_parallel` for a single large one.
`encode_parallel` returns exactly what `encode` returns and falls back to it below 64 KB.

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

`encode_parallel` encodes one large text on all cores instead of a single thread, and `decode`
no longer allocates once per token. On 2 MB of English prose on a 6-core (12-thread) i7-9750H,
`encode` takes 23.1 ms against 2.6 ms for `encode_parallel` (8.7x), and `decode` went from
28.1 ms to 7.4 ms (3.8x). Called from Python the `encode_parallel` gain is around 2.7x, since
building the returned Python list then dominates. Single-threaded `encode` is unchanged, so the
comparison against the other tokenizers above still holds.

## Changelog
- Unreleased
  - Added encode_parallel() to encode a single large text using all cores
  - Sped up decode() by removing its per-token allocation
- Version 0.9.1
  - Added utf8 error handling to decoder
- Version 0.9.0
  - Added multithreading for the function encode_batch()
  - Added batch/multithreading comparison
- Version 0.3.0
  - Fixed the issue where some characters were not encoded correctly

*This tokenizer is my very first Rust program, so it might still have many bugs and silly codes :-)*

