# flipfile

[![Crates.io](https://img.shields.io/crates/v/flipfile)](https://crates.io/crates/flipfile)
[![docs.rs](https://img.shields.io/docsrs/flipfile)](https://docs.rs/flipfile)
[![MIT license](https://img.shields.io/crates/l/flipfile)](https://choosealicense.com/licenses/mit/)
![Lines of code](https://tokei.rs/b1/github/FedericoStra/flipfile?category=code)

Flip the bytes in multiple files.

`flipfile` takes a number of files and transforms each byte.

The possible transformations are:

- `-f, --flip`: flip the bytes, i.e. negates each bit,
- `-r, --reverse`: reverse the bytes,
- `-s, --swab`: swab the bytes, i.e. swap the first 4 and the last 4 bits.
