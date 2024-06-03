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
## Bugs
~~There are still bugs where some characters are not encoded correctly.~~ The bug have been fixed in the version 0.3.0.