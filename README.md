# RWKV Tokenizer (WIP)

A fast RWKV Tokenizer using Rust. *This is my very first program using Rust, so there are still many things to improve and to fix :-)*

## Installation
You need first to install cargo (rust compiler), if you don't have it already. 
*Cargo will be not needed when I later publish the whl package in pypi*
```
$ sudo apt install cargo
```
Then install the rwkv-tokenizer python module:
```
$ pip install rwkv-tokenizer
```
## Usage
```
>>> import rwkv_tokenizer
>>> tokenizer = rwkv_tokenizer.Tokenizer("./rwkv_vocab_v20230424.txt")
>>> tokenizer.encode("Today is a beautiful day. 今天是美好的一天。")
[33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080]
>>> tokenizer.decode([33520, 4600, 332, 59219, 21509, 47, 33, 10381, 11639, 13091, 15597, 11685, 14734, 10250, 11639, 10080])
'Today is a beautiful day. 今天是美好的一天。'

```

## Performance and Validity Test

We compared the encoding results of the Rust RWKV Tokenizer and the original tokenizer using some parts of 
English Wikipedia and Chinese poetries dataset. Both results are identical. The Rust RWKV Tokenizer passes
also the unit test that the original tokenizer provided.

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