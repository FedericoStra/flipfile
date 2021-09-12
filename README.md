# flipfile

Flip the bytes in multiple files

`flipfile` takes a number of files and transforms each byte.

The possible transformations are:

- `-f, --flip`: flip the bytes, i.e. negates each bit,
- `-r, --reverse`: reverse the bytes,
- `-s, --swab`: swab the bytes, i.e. swap the first 4 and the last 4 bits.
