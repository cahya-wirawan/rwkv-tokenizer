# import the contents of the Rust library into the Python extension
from .pyrwkv_tokenizer import *
from .pyrwkv_tokenizer import __all__

# optional: include the documentation from the Rust module
from .pyrwkv_tokenizer import __doc__  # noqa: F401

__all__ = __all__ + ["RWKVTokenizer"]
__version__ = "0.7.2"


class RWKVTokenizer:
    def __init__(self, name="WorldTokenizer") -> None:
        if name != "WorldTokenizer":
            raise Exception(f"The {name} is not supported.")
        self.tokenizer = WorldTokenizer()

    def encode(self, text: str):
        tokens_ids = self.tokenizer.encode(text)
        return tokens_ids

    def decode(self, tokens_ids):
        text = self.tokenizer.decode(tokens_ids)
        return text
