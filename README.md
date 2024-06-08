# RWKV Tokenizer (WIP)

A fast RWKV Tokenizer using Rust, it supports the World tokenizer used by the RWKV v5 and v6 model.
*This is my very first Rust program, so there might still have many bugs and silly codes :-)*

## Installation
Install the rwkv-tokenizer python module:
```
$ pip install rwkv-tokenizer
```
## Usage
```
>>> import rwkv_tokenizer
>>> tokenizer = rwkv_tokenizer.RWKVTokenizer()
>>> tokenizer.encode("Today is a beautiful day. 今天是美好的一天。")
[33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080]
>>> tokenizer.decode([33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080])
'Today is a beautiful day. 今天是美好的一天。'

```

## Performance and Validity Test

We compared the encoding results of the Rust RWKV Tokenizer and the original tokenizer using some parts of 
English Wikipedia and Chinese poetries dataset. Both results are identical. The Rust RWKV Tokenizer passes
also the unit test that the original tokenizer provided. Following steps describe how to do the unittest:
```
$ pip install pytest rwkv-tokenizer
$ git clone https://github.com/cahya-wirawan/rwkv-tokenizer.git
$ cd rwkv-tokenizer
$ pytest
```

We did a performance comparison on [the simple English Wikipedia dataset 20220301.en](https://huggingface.co/datasets/legacy-datasets/wikipedia) among following tokenizer:
- The original RWKV tokenizer (BlinkDL)
- Huggingface implementaion of RWKV tokenizer
- Huggingface LLama tokenizer
- OpenAI Tiktoken
- The Rust RWKV tokenizer

The comparison is done using [this jupyter notebook](rwkv_tokenizers.ipynb) in Google Colab. The Rust RWKV tokenizer 
is around 7x faster than the original tokenizer and 3x faster than OpenAI Tiktoken.

![performance-comparison](data/performance-comparison.png)

## Bugs
~~There are still bugs where some characters are not encoded correctly.~~ The bug have been fixed in the version 0.3.0.