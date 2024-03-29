# DivvunSpell SDK for Python

[![PyPI](https://img.shields.io/pypi/v/divvunspell)](https://pypi.org/project/divvunspell)

## Installation

### From PyPI

```
pip install divvunspell
```

### From source

- Install Rust from https://rustup.rs/.
- Run `pip install .` or the equivalent for your environment

## Quick start

Import the library, then get the `Speller` instance via the `SpellerArchive`:

```python
import divvunspell


# Load a speller archive
archive = divvunspell.SpellerArchive("path/to/my/speller-archive.zhfst")
# Get the speller
speller = archive.speller()

# Use the speller:
ranked_suggestions = speller.suggest("thier")
# Smaller numbers mean a smaller weight, which means it's a more likely correction
print(ranked_suggestions)
# [('their', 1.00), ('shier', 4.5), ('tier', 4.75), ('theif', 4.8), ('Thieu', 11.3)]
```

## License

All code in this repository is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)