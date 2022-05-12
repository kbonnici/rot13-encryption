# Encryption using ROT-13
Karl Bonnici

A simple encryption algorithm ([ROT-13](https://www.cs.mcgill.ca/~rwest/wikispeedia/wpcd/wp/r/ROT13.htm)) written in Rust.

Accepts a string as input, and produces the encryption version as output.

Does not encrypt anything that is not an alphabetic character `[a-zA-Z]`.

## How to run

```console
cargo run [input]
```
where `input` is a string.

## How to test

```console
cargo test
```

## References

- [ROT 13](https://www.cs.mcgill.ca/~rwest/wikispeedia/wpcd/wp/r/ROT13.htm)
