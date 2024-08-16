# import the contents of the Rust library into the Python extension
from .pyrwkv_tokenizer import *
from .pyrwkv_tokenizer import __all__
from pathlib import Path

# optional: include the documentation from the Rust module
from .pyrwkv_tokenizer import __doc__  # noqa: F401

__all__ = __all__ + ["RWKVTokenizer"]
__version__ = "0.9.0"


class RWKVTokenizer:
    def __init__(self, name="WorldTokenizer", vocab_filepath=None) -> None:
        if name != "WorldTokenizer":
            raise Exception(f"The {name} is not supported.")
        self.vocab_filepath = str(Path(__path__[0]) / "rwkv_vocab_v20230424.txt") \
            if vocab_filepath is None else vocab_filepath
        self.tokenizer = WorldTokenizer(self.vocab_filepath)

    def encode(self, text: str):
        tokens_ids = self.tokenizer.encode(text)
        return tokens_ids

    def encode_batch(self, text_list: [str]):
        tokens_ids = self.tokenizer.encode_batch(text_list)
        return tokens_ids

    def decode(self, tokens_ids):
        text = self.tokenizer.decode(tokens_ids)
        return text

    def vocab_size(self):
        return self.tokenizer.vocab_size()

    def get_vocab(self):
        return self.tokenizer.get_vocab()